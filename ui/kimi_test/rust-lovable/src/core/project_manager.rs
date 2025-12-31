use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub settings: ProjectSettings,
    pub pages: Vec<Page>,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub platform_targets: Vec<PlatformTarget>,
    pub ui_framework: UIFramework,
    pub theme: String,
    pub build_targets: Vec<BuildTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlatformTarget {
    Web,
    Desktop,
    Mobile,
    Universal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UIFramework {
    Dioxus,
    React,
    Vue,
    Svelte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTarget {
    pub platform: PlatformTarget,
    pub output_path: PathBuf,
    pub configuration: BuildConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfiguration {
    pub minify: bool,
    pub source_map: bool,
    pub optimization_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub path: String,
    pub components: Vec<UIComponent>,
    pub metadata: PageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub viewport: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub size: u64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Font,
    Stylesheet,
    Script,
    Document,
    Other,
}

pub struct ProjectManager {
    projects_dir: PathBuf,
    current_project: Option<Project>,
}

impl ProjectManager {
    pub fn new(projects_dir: PathBuf) -> Self {
        Self {
            projects_dir,
            current_project: None,
        }
    }
    
    pub fn create_project(&mut self, name: String, description: String) -> Result<Project, ProjectError> {
        let project = Project {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            settings: ProjectSettings::default(),
            pages: vec![self.create_default_page()],
            assets: Vec::new(),
        };
        
        self.save_project(&project)?;
        self.current_project = Some(project.clone());
        
        Ok(project)
    }
    
    pub fn load_project(&mut self, project_id: &str) -> Result<Project, ProjectError> {
        let project_path = self.projects_dir.join(project_id).join("project.json");
        let project_json = std::fs::read_to_string(project_path)?;
        let project: Project = serde_json::from_str(&project_json)?;
        
        self.current_project = Some(project.clone());
        Ok(project)
    }
    
    pub fn save_project(&self, project: &Project) -> Result<(), ProjectError> {
        let project_dir = self.projects_dir.join(&project.id);
        std::fs::create_dir_all(&project_dir)?;
        
        let project_json = serde_json::to_string_pretty(project)?;
        std::fs::write(project_dir.join("project.json"), project_json)?;
        
        Ok(())
    }
    
    pub fn list_projects(&self) -> Result<Vec<ProjectSummary>, ProjectError> {
        let mut projects = Vec::new();
        
        for entry in std::fs::read_dir(&self.projects_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let project_path = entry.path().join("project.json");
                if project_path.exists() {
                    let project_json = std::fs::read_to_string(project_path)?;
                    let project: Project = serde_json::from_str(&project_json)?;
                    
                    projects.push(ProjectSummary {
                        id: project.id,
                        name: project.name,
                        description: project.description,
                        last_modified: project.updated_at,
                    });
                }
            }
        }
        
        projects.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
        Ok(projects)
    }
    
    pub fn add_page(&mut self, name: String, path: String) -> Result<Page, ProjectError> {
        let project = self.current_project.as_mut().ok_or(ProjectError::NoProjectOpen)?;
        
        let page = Page {
            id: Uuid::new_v4().to_string(),
            name,
            path,
            components: Vec::new(),
            metadata: PageMetadata::default(),
        };
        
        project.pages.push(page.clone());
        project.updated_at = chrono::Utc::now();
        
        self.save_project(project)?;
        Ok(page)
    }
    
    pub fn add_component(&mut self, page_id: &str, component: UIComponent) -> Result<(), ProjectError> {
        let project = self.current_project.as_mut().ok_or(ProjectError::NoProjectOpen)?;
        
        if let Some(page) = project.pages.iter_mut().find(|p| p.id == page_id) {
            page.components.push(component);
            project.updated_at = chrono::Utc::now();
            self.save_project(project)?;
        }
        
        Ok(())
    }
    
    pub fn export_project(&self, project_id: &str, format: ExportFormat) -> Result<PathBuf, ProjectError> {
        let project = self.load_project(project_id)?;
        let export_dir = self.projects_dir.join("exports").join(project_id);
        std::fs::create_dir_all(&export_dir)?;
        
        match format {
            ExportFormat::Zip => self.export_as_zip(&project, &export_dir),
            ExportFormat::GitHub => self.export_to_github(&project, &export_dir),
            ExportFormat::Vercel => self.export_to_vercel(&project, &export_dir),
        }
    }
    
    fn create_default_page(&self) -> Page {
        Page {
            id: Uuid::new_v4().to_string(),
            name: "Home".to_string(),
            path: "/".to_string(),
            components: Vec::new(),
            metadata: PageMetadata::default(),
        }
    }
    
    fn export_as_zip(&self, project: &Project, export_dir: &PathBuf) -> Result<PathBuf, ProjectError> {
        let zip_path = export_dir.join(format!("{}.zip", project.name));
        // Implementation for creating ZIP file
        Ok(zip_path)
    }
    
    fn export_to_github(&self, project: &Project, export_dir: &PathBuf) -> Result<PathBuf, ProjectError> {
        // Implementation for GitHub export
        Ok(export_dir.clone())
    }
    
    fn export_to_vercel(&self, project: &Project, export_dir: &PathBuf) -> Result<PathBuf, ProjectError> {
        // Implementation for Vercel export
        Ok(export_dir.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Zip,
    GitHub,
    Vercel,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            platform_targets: vec![PlatformTarget::Universal],
            ui_framework: UIFramework::Dioxus,
            theme: "default".to_string(),
            build_targets: vec![],
        }
    }
}

impl Default for PageMetadata {
    fn default() -> Self {
        Self {
            title: "Untitled Page".to_string(),
            description: String::new(),
            keywords: vec![],
            viewport: "width=device-width, initial-scale=1.0".to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("No project is currently open")]
    NoProjectOpen,
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
}