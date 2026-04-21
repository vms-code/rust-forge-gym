use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: u16,
    pub code: String,
    pub difficulty: u8,
    pub answer: String,
    pub hint: String,
    pub explanation: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub type QuizzesManifest = BTreeMap<u16, Quiz>;