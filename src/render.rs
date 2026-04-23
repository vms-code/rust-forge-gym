use crate::error::Error;
use crate::models::Quiz;
use oqueue::{Color::Red, Sequencer};
use parking_lot::Mutex;
use pulldown_cmark::{html as markdown_html, Parser as MarkdownParser};
use rayon::ThreadPoolBuilder;
use std::collections::BTreeMap;
use std::env;
use std::env::consts::EXE_EXTENSION;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::hash::{Hash, Hasher};

pub const MARKDOWN_FORMAT: &str = "
    Type: stdout|compile|text|multiple-choice
    Question: What does this program output?
    Answer: 999
    Difficulty: 1|2|3
    Tags: tag1, tag2
    Warnings: warning1, warning2

    # Options

    A. First option
    B. Second option

    # Hint

    <!-- markdown -->

    # Explanation

    <!-- markdown -->
";

fn scan_quiz_files<P: AsRef<Path>>(dir: P, files: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            scan_quiz_files(path, files)?;
        } else if path.to_string_lossy().ends_with(".rs") {
            files.push(path);
        }
    }
    Ok(())
}

pub fn build() -> Result<(), Error> {
    let mut question_files = Vec::new();
    scan_quiz_files("quizzes", &mut question_files)?;
    question_files.sort();

    let cpus = num_cpus::get();
    let pool = ThreadPoolBuilder::new()
        .num_threads(cpus)
        .build()
        .map_err(Error::Rayon)?;

    let oqueue = Sequencer::stderr();
    let questions = Mutex::new(BTreeMap::new());
    pool.scope(|scope| {
        for _ in 0..cpus {
            scope.spawn(|_| worker(&oqueue, &question_files, &questions));
        }
    });

    let questions = questions.into_inner();
    if questions.len() < question_files.len() {
        // Error already printed.
        process::exit(1);
    }

    let json_object = serde_json::to_string_pretty(&questions)?;
    fs::write("quizzes.json", json_object)?;

    Ok(())
}

fn worker(oqueue: &Sequencer, files: &[PathBuf], out: &Mutex<BTreeMap<u32, Quiz>>) {
    loop {
        let task = oqueue.begin();
        let Some(rs_path) = files.get(task.index) else {
            return;
        };

        writeln!(task, "evaluating {}", rs_path.display());

        if let Err(err) = work(rs_path, out) {
            task.bold_color(Red);
            write!(task, "ERROR");
            task.bold();
            writeln!(task, ": {}", err);
        }
    }
}

fn work(rs_path: &Path, out: &Mutex<BTreeMap<u32, Quiz>>) -> Result<(), Error> {
    let code = fs::read_to_string(rs_path)?;

    let md_path = rs_path.with_extension("md");
    let mut md_content = fs::read_to_string(&md_path)?;
    md_content = md_content.replace("\r\n", "\n");

    let lines: Vec<&str> = md_content.lines().collect();

    // Find section boundaries
    let mut options_start = None;
    let mut hint_start = None;
    let mut explanation_start = None;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("# Options") {
            options_start = Some(i);
        } else if line.starts_with("# Hint") {
            hint_start = Some(i);
        } else if line.starts_with("# Explanation") {
            explanation_start = Some(i);
        }
    }

    let first_section_start = options_start
        .or(hint_start)
        .or(explanation_start)
        .unwrap_or(lines.len());

    if hint_start.is_none() && options_start.is_none() && explanation_start.is_none() {
        return Err(Error::MarkdownFormat(md_path));
    }

    // Parse metadata from lines before the first section
    let mut answer = String::new();
    let mut difficulty = 1u8;
    let mut tags = Vec::new();
    let mut warnings = Vec::new();
    let mut quiz_type = String::new();
    let mut question = String::new();

    let mut i = 0;
    while i < first_section_start {
        let line = lines[i];

        if let Some(rest) = line.strip_prefix("Answer:") {
            answer = rest.trim().to_owned();
            // Look ahead for multi-line answer until next metadata or section
            while i + 1 < first_section_start &&
                  !lines[i+1].starts_with("Difficulty:") &&
                  !lines[i+1].starts_with("Tags:") &&
                  !lines[i+1].starts_with("Warnings:") &&
                  !lines[i+1].starts_with("Type:") &&
                  !lines[i+1].starts_with("Question:") &&
                  !lines[i+1].starts_with("#")
            {
                i += 1;
                let next_line = lines[i].trim();
                if !next_line.is_empty() {
                    if !answer.is_empty() {
                        answer.push('\n');
                    }
                    answer.push_str(next_line);
                }
            }
        } else if let Some(rest) = line.strip_prefix("Difficulty:") {
            difficulty = rest.trim().parse().unwrap_or(1);
        } else if let Some(rest) = line.strip_prefix("Tags:") {
            tags = rest.split(',')
                .map(|s| s.trim().trim_matches('"').trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect();
        } else if let Some(rest) = line.strip_prefix("Warnings:") {
            warnings = rest.split(',')
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect();
        } else if let Some(rest) = line.strip_prefix("Type:") {
            quiz_type = rest.trim().to_lowercase().to_owned();
        } else if let Some(rest) = line.strip_prefix("Question:") {
            question = rest.trim().to_owned();
            // Look ahead for multi-line question
            while i + 1 < first_section_start &&
                  !lines[i+1].starts_with("Answer:") &&
                  !lines[i+1].starts_with("Difficulty:") &&
                  !lines[i+1].starts_with("Tags:") &&
                  !lines[i+1].starts_with("Warnings:") &&
                  !lines[i+1].starts_with("Type:") &&
                  !lines[i+1].starts_with("#")
            {
                i += 1;
                let next_line = lines[i].trim();
                if !next_line.is_empty() {
                    if !question.is_empty() {
                        question.push('\n');
                    }
                    question.push_str(next_line);
                }
            }
        }
        i += 1;
    }

    // Parse options
    let mut options = Vec::new();
    if let Some(opts_start) = options_start {
        let opts_end = hint_start.or(explanation_start).unwrap_or(lines.len());
        for line in lines.iter().skip(opts_start + 1).take(opts_end - opts_start - 1) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                options.push(trimmed.to_string());
            }
        }
    }

    // Parse hint and explanation
    let mut hint_md = String::new();
    let mut explanation_md = String::new();
    let mut in_explanation = false;

    let hint_start_idx = hint_start.unwrap_or(lines.len());
    for line in lines.iter().skip(hint_start_idx + 1) {
        if line.starts_with("# Explanation") {
            in_explanation = true;
            continue;
        }
        if in_explanation {
            explanation_md.push_str(line);
            explanation_md.push('\n');
        } else {
            hint_md.push_str(line);
            hint_md.push('\n');
        }
    }

    let hint = render_to_html(hint_md.trim());
    let explanation = render_to_html(explanation_md.trim());
    let question_html = render_to_html(&question);

    // Validate based on quiz type
    if quiz_type.is_empty() {
        // Backward compatibility: infer type from answer and run full check
        check_answer(rs_path, &answer, &warnings)?;
        quiz_type = if answer == "undefined" || answer == "error" {
            "compile".to_string()
        } else {
            "stdout".to_string()
        };
    } else {
        match quiz_type.as_str() {
            "stdout" | "compile" => {
                check_answer(rs_path, &answer, &warnings)?;
            }
            "text" => {
                // No compilation or execution check; the code is illustrative
            }
            "multiple-choice" => {
                if options.is_empty() {
                    return Err(Error::MarkdownFormat(md_path));
                }
                let answer_key = answer.trim();
                let valid = options.iter().any(|opt| {
                    let opt_trimmed = opt.trim();
                    opt_trimmed.starts_with(answer_key)
                });
                if !valid {
                    return Err(Error::WrongOutput {
                        expected: format!("one of the options starting with '{}'", answer_key),
                        output: format!("options: {}", options.join(", ")),
                    });
                }
            }
            _ => return Err(Error::MarkdownFormat(md_path)),
        }
    }

    // Extract category and ID
    let rel_path = rs_path.strip_prefix("quizzes").unwrap_or(rs_path);
    let category = rel_path.parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| "uncategorized".to_string());

    // Use a hash of the relative path as the ID
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    rel_path.hash(&mut hasher);
    let number = (hasher.finish() & 0xFFFFFFFF) as u32;

    let quiz = Quiz {
        id: number,
        code,
        difficulty,
        answer,
        hint,
        explanation,
        tags,
        category,
        question: question_html,
        quiz_type,
        options,
    };

    let mut map = out.lock();
    map.insert(number, quiz);
    Ok(())
}

fn render_to_html(markdown: &str) -> String {
    let parser = MarkdownParser::new(markdown);
    let mut html = String::new();
    markdown_html::push_html(&mut html, parser);
    html = html.replace("<a href=\"", "<a target=\"_blank\" href=\"");
    html
}

#[derive(Copy, Clone)]
enum Status {
    Ok,
    Err,
}

fn check_answer(rs_path: &Path, expected: &str, warnings: &[String]) -> Result<(), Error> {
    let out_dir = env::temp_dir().join("rust-quiz");

    let mut cmd = rustc(&out_dir, rs_path);
    cmd.arg("--deny=warnings");
    for warning in warnings {
        cmd.arg("--allow").arg(warning);
    }

    let status = cmd.status().map_err(Error::Rustc)?;
    let status = match status.success() {
        true => Status::Ok,
        false => Status::Err,
    };

    if let Status::Err = status {
        if rustc(&out_dir, rs_path)
            .arg("--allow=warnings")
            .status()
            .map_err(Error::Rustc)?
            .success()
        {
            return Err(Error::CompiledWithWarnings);
        }
    }

    match (expected, status) {
        ("undefined", Status::Ok) | ("error", Status::Err) => {}
        ("undefined", Status::Err) => return Err(Error::UndefinedShouldCompile),
        ("error", Status::Ok) => return Err(Error::ShouldNotCompile),
        (_, Status::Err) => return Err(Error::ShouldCompile),
        (_, Status::Ok) => run(&out_dir, rs_path, expected)?,
    }

    if let Status::Ok = status {
        let mut missing_warnings = Vec::new();
        for check_warning in warnings {
            let mut cmd = rustc(&out_dir, rs_path);
            cmd.arg("--deny=warnings");
            for warning in warnings {
                if warning != check_warning {
                    cmd.arg("--allow").arg(warning);
                }
            }
            if cmd.status().map_err(Error::Rustc)?.success() {
                missing_warnings.push(check_warning.clone());
            }
        }
        if !missing_warnings.is_empty() {
            return Err(Error::MissingExpectedWarning(missing_warnings));
        }
    }

    Ok(())
}

fn rustc(out_dir: &Path, rs_path: &Path) -> Command {
    let mut cmd = Command::new("rustc");
    cmd.arg(rs_path)
        .arg("--edition=2021")
        .arg("--out-dir")
        .arg(out_dir)
        .stderr(Stdio::null());
    cmd
}

fn run(out_dir: &Path, rs_path: &Path, expected: &str) -> Result<(), Error> {
    let stem = rs_path.file_stem().unwrap();
    let exe = out_dir.join(stem).with_extension(EXE_EXTENSION);
    let output = Command::new(exe).output().map_err(Error::Execute)?;
    let output = String::from_utf8(output.stdout)?.replace("\r\n", "\n");
    let expected = expected.replace("\r\n", "\n");

    if output.trim() == expected.trim() {
        Ok(())
    } else {
        Err(Error::WrongOutput {
            expected: expected.trim().to_owned(),
            output: output.trim().to_owned(),
        })
    }
}
