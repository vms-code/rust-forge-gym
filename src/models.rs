use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: u32,
    pub code: String,
    pub difficulty: u8,
    pub answer: String,
    pub hint: String,
    pub explanation: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub question: String,
    #[serde(default)]
    pub quiz_type: String, // compile | stdout | text | multiple-choice
    #[serde(default)]
    pub options: Vec<String>,
}

pub type QuizzesManifest = BTreeMap<u32, Quiz>;