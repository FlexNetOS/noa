//! Prompt cache for AI prompts

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// An AI prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub version: String,
    pub metadata: PromptMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub usage_stats: UsageStats,
}

/// Metadata for a prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub author: Option<String>,
    pub description: Option<String>,
    pub parameters: HashMap<String, String>,
    pub examples: Vec<PromptExample>,
    pub validation_rules: Vec<ValidationRule>,
    pub access_level: AccessLevel,
}

/// A prompt example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptExample {
    pub input: String,
    pub output: String,
    pub description: Option<String>,
}

/// A validation rule for prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub pattern: Option<String>,
    pub required: bool,
    pub message: String,
}

/// Access levels for prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessLevel {
    Public,
    Internal,
    Private,
    Restricted,
}

/// Usage statistics for prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub usage_count: u64,
    pub last_used: Option<DateTime<Utc>>,
    pub average_rating: Option<f64>,
    pub total_ratings: u64,
}

/// A cached prompt with access tracking
#[derive(Debug, Clone)]
pub struct CachedPrompt {
    pub prompt: Prompt,
    pub last_accessed: Instant,
    pub access_count: u64,
}

/// Cache for prompts
pub struct PromptCache {
    prompts: HashMap<String, CachedPrompt>,
    category_index: HashMap<String, Vec<String>>,
    tag_index: HashMap<String, Vec<String>>,
    author_index: HashMap<String, Vec<String>>,
    max_size: usize,
    ttl: Duration,
}

impl PromptCache {
    /// Create a new PromptCache
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            category_index: HashMap::new(),
            tag_index: HashMap::new(),
            author_index: HashMap::new(),
            max_size: 10000,
            ttl: Duration::from_secs(3600),
        }
    }

    /// Initialize the cache
    pub async fn initialize(&mut self) -> Result<()> {
        self.load_from_storage().await?;
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        self.remove_expired();
        self.save_to_storage().await?;
        Ok(())
    }

    /// Add a prompt to the cache
    pub fn add_prompt(&mut self, prompt: Prompt) -> Result<()> {
        let id = prompt.id.clone();

        let cached_prompt = CachedPrompt {
            prompt: prompt.clone(),
            last_accessed: Instant::now(),
            access_count: 0,
        };

        // Update indices
        self.update_indices(&prompt, true);

        // Add to cache
        self.prompts.insert(id, cached_prompt);

        // Evict if necessary
        self.evict_if_needed();

        Ok(())
    }

    /// Get a prompt by ID
    pub fn get_prompt(&mut self, id: &str) -> Option<Prompt> {
        if let Some(cached) = self.prompts.get_mut(id) {
            cached.last_accessed = Instant::now();
            cached.access_count += 1;
            Some(cached.prompt.clone())
        } else {
            None
        }
    }

    /// Get prompts by category
    pub fn get_by_category(&self, category: &str) -> Vec<Prompt> {
        self.category_index
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.prompts.get(id).map(|c| c.prompt.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get prompts by tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<Prompt> {
        self.tag_index
            .get(tag)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.prompts.get(id).map(|c| c.prompt.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove a prompt
    pub fn remove_prompt(&mut self, id: &str) -> Option<Prompt> {
        if let Some(cached) = self.prompts.remove(id) {
            self.update_indices(&cached.prompt, false);
            Some(cached.prompt)
        } else {
            None
        }
    }

    fn update_indices(&mut self, prompt: &Prompt, add: bool) {
        let id = &prompt.id;

        if add {
            self.category_index
                .entry(prompt.category.clone())
                .or_default()
                .push(id.clone());

            for tag in &prompt.tags {
                self.tag_index
                    .entry(tag.clone())
                    .or_default()
                    .push(id.clone());
            }

            if let Some(author) = &prompt.metadata.author {
                self.author_index
                    .entry(author.clone())
                    .or_default()
                    .push(id.clone());
            }
        } else {
            if let Some(ids) = self.category_index.get_mut(&prompt.category) {
                ids.retain(|i| i != id);
            }
            for tag in &prompt.tags {
                if let Some(ids) = self.tag_index.get_mut(tag) {
                    ids.retain(|i| i != id);
                }
            }
            if let Some(author) = &prompt.metadata.author {
                if let Some(ids) = self.author_index.get_mut(author) {
                    ids.retain(|i| i != id);
                }
            }
        }
    }

    fn remove_expired(&mut self) {
        let now = Instant::now();
        let expired: Vec<_> = self
            .prompts
            .iter()
            .filter(|(_, cached)| now.duration_since(cached.last_accessed) > self.ttl)
            .map(|(id, _)| id.clone())
            .collect();

        for id in expired {
            self.remove_prompt(&id);
        }
    }

    fn evict_if_needed(&mut self) {
        while self.prompts.len() > self.max_size {
            // Find least recently used
            if let Some(lru_id) = self
                .prompts
                .iter()
                .min_by_key(|(_, cached)| cached.last_accessed)
                .map(|(id, _)| id.clone())
            {
                self.remove_prompt(&lru_id);
            } else {
                break;
            }
        }
    }

    async fn load_from_storage(&mut self) -> Result<()> {
        // Placeholder for loading from persistent storage
        Ok(())
    }

    async fn save_to_storage(&self) -> Result<()> {
        // Placeholder for saving to persistent storage
        Ok(())
    }
}

impl Default for PromptCache {
    fn default() -> Self {
        Self::new()
    }
}
