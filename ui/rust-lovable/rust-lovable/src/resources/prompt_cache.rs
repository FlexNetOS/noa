use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub version: String,
    pub metadata: PromptMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub usage_stats: UsageStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub author: Option<String>,
    pub description: Option<String>,
    pub parameters: HashMap<String, String>,
    pub examples: Vec<PromptExample>,
    pub validation_rules: Vec<ValidationRule>,
    pub access_level: AccessLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptExample {
    pub input: String,
    pub output: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub pattern: Option<String>,
    pub required: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessLevel {
    Public,
    Internal,
    Private,
    Restricted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub usage_count: u64,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub average_rating: Option<f64>,
    pub total_ratings: u64,
}

#[derive(Debug, Clone)]
pub struct CachedPrompt {
    pub prompt: Prompt,
    pub last_accessed: Instant,
    pub access_count: u64,
}

pub struct PromptCache {
    prompts: HashMap<String, CachedPrompt>,
    category_index: HashMap<String, Vec<String>>,
    tag_index: HashMap<String, Vec<String>>,
    author_index: HashMap<String, Vec<String>>,
    max_size: usize,
    ttl: Duration,
}

impl PromptCache {
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

    pub async fn initialize(&mut self) -> Result<()> {
        // Load prompts from storage if available
        self.load_from_storage().await?;
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        // Remove expired prompts
        self.remove_expired();
        
        // Save prompts to storage
        self.save_to_storage().await?;
        Ok(())
    }

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

    pub fn get_prompt(&mut self, id: &str) -> Option<Prompt> {
        if let Some(cached) = self.prompts.get_mut(id) {
            cached.last_accessed = Instant::now();
            cached.access_count += 1;
            Some(cached.prompt.clone())
        } else {
            None
        }
    }

    pub fn get_prompts_by_category(&mut self, category: &str) -> Vec<Prompt> {
        let ids: Vec<String> = self.category_index
            .get(category)
            .cloned()
            .unwrap_or_default();
        ids.iter()
            .filter_map(|id| self.get_prompt(id))
            .collect()
    }

    pub fn get_prompts_by_tag(&mut self, tag: &str) -> Vec<Prompt> {
        let ids: Vec<String> = self.tag_index
            .get(tag)
            .cloned()
            .unwrap_or_default();
        ids.iter()
            .filter_map(|id| self.get_prompt(id))
            .collect()
    }

    pub fn get_prompts_by_author(&mut self, author: &str) -> Vec<Prompt> {
        let ids: Vec<String> = self.author_index
            .get(author)
            .cloned()
            .unwrap_or_default();
        ids.iter()
            .filter_map(|id| self.get_prompt(id))
            .collect()
    }

    pub fn search_prompts(&self, query: &str, category: Option<&str>, tags: Option<Vec<&str>>) -> Vec<Prompt> {
        let query_lower = query.to_lowercase();
        
        self.prompts.values()
            .filter(|cached| {
                let prompt = &cached.prompt;
                
                // Text search
                let text_match = prompt.content.to_lowercase().contains(&query_lower) ||
                    prompt.metadata.description.as_ref()
                        .map(|desc| desc.to_lowercase().contains(&query_lower))
                        .unwrap_or(false);
                
                // Category filter
                let category_match = category.map_or(true, |cat| prompt.category == cat);
                
                // Tags filter
                let tags_match = tags.as_ref().map_or(true, |search_tags| {
                    search_tags.iter().all(|tag| prompt.tags.contains(&tag.to_string()))
                });
                
                text_match && category_match && tags_match
            })
            .map(|cached| cached.prompt.clone())
            .collect()
    }

    pub fn update_prompt(&mut self, id: &str, updates: PromptUpdates) -> Result<()> {
        if let Some(cached) = self.prompts.get_mut(id) {
            let prompt = &mut cached.prompt;
            
            if let Some(content) = updates.content {
                prompt.content = content;
            }
            if let Some(category) = updates.category {
                // Remove from old category index
                if let Some(old_category) = self.category_index.get_mut(&prompt.category) {
                    old_category.retain(|prompt_id| prompt_id != id);
                }
                
                prompt.category = category;
                // Add to new category index
                self.category_index
                    .entry(prompt.category.clone())
                    .or_default()
                    .push(id.to_string());
            }
            if let Some(tags) = updates.tags {
                // Update tag indices
                for tag in &prompt.tags {
                    if let Some(tag_index) = self.tag_index.get_mut(tag) {
                        tag_index.retain(|prompt_id| prompt_id != id);
                    }
                }
                
                prompt.tags = tags;
                for tag in &prompt.tags {
                    self.tag_index
                        .entry(tag.clone())
                        .or_default()
                        .push(id.to_string());
                }
            }
            
            prompt.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Prompt not found"))
        }
    }

    pub fn remove_prompt(&mut self, id: &str) -> Result<()> {
        if let Some(cached) = self.prompts.remove(id) {
            // Remove from indices
            self.update_indices(&cached.prompt, false);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Prompt not found"))
        }
    }

    pub fn rate_prompt(&mut self, id: &str, rating: f64) -> Result<()> {
        if let Some(cached) = self.prompts.get_mut(id) {
            let stats = &mut cached.prompt.usage_stats;
            let total_ratings = stats.total_ratings as f64;
            let current_avg = stats.average_rating.unwrap_or(0.0);
            
            stats.average_rating = Some((current_avg * total_ratings + rating) / (total_ratings + 1.0));
            stats.total_ratings += 1;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Prompt not found"))
        }
    }

    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            total_prompts: self.prompts.len(),
            categories: self.category_index.len(),
            tags: self.tag_index.len(),
            authors: self.author_index.len(),
        }
    }

    fn update_indices(&mut self, prompt: &Prompt, add: bool) {
        let id = &prompt.id;
        
        // Category index
        if add {
            self.category_index
                .entry(prompt.category.clone())
                .or_default()
                .push(id.clone());
        } else {
            if let Some(index) = self.category_index.get_mut(&prompt.category) {
                index.retain(|prompt_id| prompt_id != id);
            }
        }
        
        // Tag index
        for tag in &prompt.tags {
            if add {
                self.tag_index
                    .entry(tag.clone())
                    .or_default()
                    .push(id.clone());
            } else {
                if let Some(index) = self.tag_index.get_mut(tag) {
                    index.retain(|prompt_id| prompt_id != id);
                }
            }
        }
        
        // Author index
        if let Some(author) = &prompt.metadata.author {
            if add {
                self.author_index
                    .entry(author.clone())
                    .or_default()
                    .push(id.clone());
            } else {
                if let Some(index) = self.author_index.get_mut(author) {
                    index.retain(|prompt_id| prompt_id != id);
                }
            }
        }
    }

    fn remove_expired(&mut self) {
        let now = Instant::now();
        let expired_ids: Vec<String> = self.prompts
            .iter()
            .filter(|(_, cached)| now.duration_since(cached.last_accessed) > self.ttl)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired_ids {
            let _ = self.remove_prompt(&id);
        }
    }

    fn evict_if_needed(&mut self) {
        if self.prompts.len() > self.max_size {
            // Find the least recently used prompt
            let lru_id = self.prompts
                .iter()
                .min_by_key(|(_, cached)| cached.last_accessed)
                .map(|(id, _)| id.clone());
            
            if let Some(id) = lru_id {
                let _ = self.remove_prompt(&id);
            }
        }
    }

    async fn load_from_storage(&mut self) -> Result<()> {
        // Implementation would load from configured storage backend
        Ok(())
    }

    async fn save_to_storage(&mut self) -> Result<()> {
        // Implementation would save to configured storage backend
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PromptUpdates {
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_prompts: usize,
    pub categories: usize,
    pub tags: usize,
    pub authors: usize,
}

impl Prompt {
    pub fn new(
        content: String,
        category: String,
        tags: Vec<String>,
        metadata: PromptMetadata,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            category,
            tags,
            version: "1.0".to_string(),
            metadata,
            created_at: now,
            updated_at: now,
            usage_stats: UsageStats {
                usage_count: 0,
                last_used: None,
                average_rating: None,
                total_ratings: 0,
            },
        }
    }
}