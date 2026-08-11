use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageVariant {
    pub gallery_id: u32,
    /// korean, chinese, japanese
    pub name: String,
    /// 한국어, 中文, 日本語
    pub local_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gallery {
    pub id: u32,
    pub title: String,
    pub japanese_title: Option<String>,
    pub kind: String,
    /// (page, File)
    ///
    /// page starts from 1
    pub files: Vec<(usize, File)>,
    pub language: Option<String>,
    pub language_variants: Vec<LanguageVariant>,
    pub related: Vec<u32>,
    pub tags: Vec<Tag>,
    pub date: DateTime<Utc>,
}
