use crate::governance::biblical::ingest::ScriptureLanguage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Token frequency statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStat {
    pub token: String,
    pub count: u32,
    pub language_hint: Option<ScriptureLanguage>,
}

/// Lexical analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexicalAnalysis {
    pub total_tokens: usize,
    pub unique_tokens: usize,
    pub greek_ratio: f32,
    pub hebrew_ratio: f32,
    pub tokens: Vec<TokenStat>,
}

/// Lightweight lexical analyzer for Greek/Hebrew passages.
#[derive(Debug, Clone, Copy, Default)]
pub struct LexicalAnalyzer;

impl LexicalAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, text: &str) -> LexicalAnalysis {
        let mut counts: HashMap<String, u32> = HashMap::new();
        let mut greek = 0usize;
        let mut hebrew = 0usize;
        let mut total = 0usize;

        for raw in text.split_whitespace() {
            if let Some(token) = Self::normalize(raw) {
                let lang = detect_language_hint(&token);
                if matches!(lang, Some(ScriptureLanguage::Greek)) {
                    greek += 1;
                } else if matches!(lang, Some(ScriptureLanguage::Hebrew)) {
                    hebrew += 1;
                }

                *counts.entry(token).or_insert(0) += 1;
                total += 1;
            }
        }

        let tokens: Vec<TokenStat> = counts
            .into_iter()
            .map(|(token, count)| TokenStat {
                language_hint: detect_language_hint(&token),
                token,
                count,
            })
            .collect();

        LexicalAnalysis {
            total_tokens: total,
            unique_tokens: tokens.len(),
            greek_ratio: if total == 0 {
                0.0
            } else {
                (greek as f32) / (total as f32)
            },
            hebrew_ratio: if total == 0 {
                0.0
            } else {
                (hebrew as f32) / (total as f32)
            },
            tokens,
        }
    }

    fn normalize(token: &str) -> Option<String> {
        let cleaned: String = token
            .chars()
            .filter(|ch| ch.is_alphabetic() || ch == &'-' || ch == &'\'')
            .collect();

        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned.to_lowercase())
        }
    }
}

fn detect_language_hint(token: &str) -> Option<ScriptureLanguage> {
    let mut greek = 0usize;
    let mut hebrew = 0usize;
    for ch in token.chars() {
        if matches!(ch as u32, 0x0370..=0x03FF | 0x1F00..=0x1FFF) {
            greek += 1;
        } else if matches!(ch as u32, 0x0590..=0x05FF) {
            hebrew += 1;
        }
    }

    match (greek, hebrew) {
        (g, h) if g > h && g > 0 => Some(ScriptureLanguage::Greek),
        (g, h) if h > g && h > 0 => Some(ScriptureLanguage::Hebrew),
        _ => None,
    }
}
