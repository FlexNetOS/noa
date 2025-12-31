use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::conversational_ai::{PlatformTarget, UIChangeRequest, UIChangeType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UIComponent {
    pub id: String,
    pub component_type: ComponentType,
    pub properties: HashMap<String, serde_json::Value>,
    pub children: Vec<UIComponent>,
    pub platform_adaptations: HashMap<PlatformTarget, PlatformAdaptation>,
    pub generated_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComponentType {
    // Layout components
    Container,
    Flex,
    Grid,
    Stack,

    // Basic components
    Text,
    Button,
    Image,
    Icon,

    // Form components
    Input,
    TextArea,
    Select,
    Checkbox,
    Radio,

    // Navigation
    Navbar,
    Sidebar,
    Tabs,
    Menu,

    // Data display
    Table,
    List,
    Card,
    Badge,

    // Interactive
    Modal,
    Dropdown,
    Tooltip,
    Accordion,

    // Custom
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlatformAdaptation {
    pub styles: HashMap<String, String>,
    pub behavior: HashMap<String, String>,
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Interaction {
    pub event: String,
    pub action: String,
    pub target: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIGenerator {
    component_library: ComponentLibrary,
    code_templates: HashMap<String, String>,
    platform_targets: Vec<PlatformTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLibrary {
    pub components: HashMap<ComponentType, ComponentDefinition>,
    pub themes: HashMap<String, Theme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinition {
    pub name: String,
    pub description: String,
    pub properties: Vec<PropertyDefinition>,
    pub supported_platforms: Vec<PlatformTarget>,
    pub code_templates: HashMap<PlatformTarget, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    pub name: String,
    pub property_type: PropertyType,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Color,
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
    pub typography: Typography,
    pub spacing: Spacing,
    pub breakpoints: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub font_family: String,
    pub font_sizes: HashMap<String, String>,
    pub font_weights: HashMap<String, String>,
    pub line_heights: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub unit: f32,
    pub scale: Vec<f32>,
}

impl UIGenerator {
    pub fn new() -> Self {
        Self {
            component_library: Self::initialize_component_library(),
            code_templates: Self::initialize_code_templates(),
            platform_targets: vec![PlatformTarget::Universal],
        }
    }

    pub fn generate_component(&mut self, request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        match request.change_type {
            UIChangeType::CreateComponent => self.create_component(request),
            UIChangeType::ModifyComponent => self.modify_component(request),
            UIChangeType::DeleteComponent => self.delete_component(request),
            UIChangeType::RestructureLayout => self.restructure_layout(request),
            UIChangeType::ChangeStyling => self.change_styling(request),
            UIChangeType::AddInteraction => self.add_interaction(request),
            UIChangeType::UpdateContent => self.update_content(request),
        }
    }

    fn create_component(&mut self, request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Parse component type from description
        let component_type = self.infer_component_type(&request.description);

        let component = UIComponent {
            id: Uuid::new_v4().to_string(),
            component_type: component_type.clone(),
            properties: self.get_default_properties(&component_type),
            children: Vec::new(),
            platform_adaptations: HashMap::new(),
            generated_code: None,
        };

        // Generate code for the component
        let generated_code = self.generate_code(&component, &PlatformTarget::Universal)?;
        let mut component = component;
        component.generated_code = Some(generated_code);

        Ok(component)
    }

    fn modify_component(&mut self, _request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Find component by ID or description
        // Apply modifications
        // Regenerate code
        todo!()
    }

    fn delete_component(&mut self, _request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Find and remove component
        // Update parent references
        todo!()
    }

    fn restructure_layout(&mut self, _request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Reorganize component hierarchy
        // Update layout properties
        todo!()
    }

    fn change_styling(&mut self, _request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Parse styling changes
        // Update component properties
        // Apply platform-specific adaptations
        todo!()
    }

    fn add_interaction(&mut self, _request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Parse interaction requirements
        // Add event handlers and behaviors
        // Generate interaction code
        todo!()
    }

    fn update_content(&mut self, _request: UIChangeRequest) -> Result<UIComponent, UIGenError> {
        // Update text content, images, etc.
        // Regenerate affected components
        todo!()
    }

    fn infer_component_type(&self, description: &str) -> ComponentType {
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("button") {
            ComponentType::Button
        } else if desc_lower.contains("text") || desc_lower.contains("label") || desc_lower.contains("heading") {
            ComponentType::Text
        } else if desc_lower.contains("input") || desc_lower.contains("field") {
            ComponentType::Input
        } else if desc_lower.contains("image") || desc_lower.contains("picture") || desc_lower.contains("photo") {
            ComponentType::Image
        } else if desc_lower.contains("container") || desc_lower.contains("box") || desc_lower.contains("div") {
            ComponentType::Container
        } else if desc_lower.contains("flex") || desc_lower.contains("row") || desc_lower.contains("column") {
            ComponentType::Flex
        } else if desc_lower.contains("grid") || desc_lower.contains("table") {
            ComponentType::Grid
        } else if desc_lower.contains("navbar") || desc_lower.contains("header") || desc_lower.contains("navigation") {
            ComponentType::Navbar
        } else if desc_lower.contains("modal") || desc_lower.contains("popup") || desc_lower.contains("dialog") {
            ComponentType::Modal
        } else {
            ComponentType::Container // Default
        }
    }

    fn get_default_properties(&self, component_type: &ComponentType) -> HashMap<String, serde_json::Value> {
        let mut properties = HashMap::new();

        match component_type {
            ComponentType::Text => {
                properties.insert("text".to_string(), serde_json::Value::String("Sample text".to_string()));
                properties.insert("fontSize".to_string(), serde_json::Value::String("16px".to_string()));
                properties.insert("color".to_string(), serde_json::Value::String("#000000".to_string()));
            }
            ComponentType::Button => {
                properties.insert("text".to_string(), serde_json::Value::String("Click me".to_string()));
                properties.insert("variant".to_string(), serde_json::Value::String("primary".to_string()));
                properties.insert("size".to_string(), serde_json::Value::String("medium".to_string()));
            }
            ComponentType::Input => {
                properties.insert(
                    "placeholder".to_string(),
                    serde_json::Value::String("Enter text...".to_string()),
                );
                properties.insert("type".to_string(), serde_json::Value::String("text".to_string()));
            }
            ComponentType::Container => {
                properties.insert("padding".to_string(), serde_json::Value::String("16px".to_string()));
                properties.insert("display".to_string(), serde_json::Value::String("flex".to_string()));
            }
            _ => {}
        }

        properties
    }

    pub fn generate_code(
        &self,
        component: &UIComponent,
        platform: &PlatformTarget,
    ) -> Result<String, UIGenError> {
        let template = self.get_code_template(&component.component_type, platform)?;
        let code = self.fill_template(template, component)?;
        Ok(code)
    }

    fn get_code_template(
        &self,
        component_type: &ComponentType,
        platform: &PlatformTarget,
    ) -> Result<String, UIGenError> {
        // Get appropriate template based on component type and platform
        let template_key = format!("{:?}_{:?}", component_type, platform);

        match self.code_templates.get(&template_key) {
            Some(template) => Ok(template.clone()),
            None => self.get_default_template(component_type),
        }
    }

    fn get_default_template(&self, component_type: &ComponentType) -> Result<String, UIGenError> {
        let template = match component_type {
            ComponentType::Text => {
                r#"
                rsx! {{
                    p {{
                        class: \"{class_name}\",
                        \"{text}\"
                    }}
                }}
            "#
            }
            ComponentType::Button => {
                r#"
                rsx! {{
                    button {{
                        class: \"{class_name}\",
                        onclick: move |_| {{
                            {onclick_handler}
                        }},
                        \"{text}\"
                    }}
                }}
            "#
            }
            ComponentType::Input => {
                r#"
                rsx! {{
                    input {{
                        class: \"{class_name}\",
                        placeholder: \"{placeholder}\",
                        r#type: \"{input_type}\",
                        value: \"{value}\",
                        oninput: move |event| {{
                            {oninput_handler}
                        }}
                    }}
                }}
            "#
            }
            ComponentType::Container => {
                r#"
                rsx! {{
                    div {{
                        class: \"{class_name}\",
                        {children}
                    }}
                }}
            "#
            }
            _ => {
                r#"
                rsx! {{
                    div {{
                        class: \"{class_name}\",
                        \"Component: {component_type}\"
                    }}
                }}
            "#
            }
        };

        Ok(template.to_string())
    }

    fn fill_template(&self, template: String, component: &UIComponent) -> Result<String, UIGenError> {
        let mut filled = template;

        // Replace placeholders with actual values
        for (key, value) in &component.properties {
            let placeholder = format!("{{{}}}", key);
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => continue,
            };
            filled = filled.replace(&placeholder, &value_str);
        }

        // Replace component type placeholder
        filled = filled.replace("{component_type}", &format!("{:?}", component.component_type));

        // Replace class name placeholder
        let class_name = format!("component-{}-{:?}", component.id, component.component_type);
        filled = filled.replace("{class_name}", &class_name);

        Ok(filled)
    }

    fn initialize_component_library() -> ComponentLibrary {
        // Initialize with default components and themes
        ComponentLibrary {
            components: HashMap::new(),
            themes: HashMap::new(),
        }
    }

    fn initialize_code_templates() -> HashMap<String, String> {
        // Initialize with code templates for different platforms
        HashMap::new()
    }
}

impl Default for UIGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UIGenError {
    #[error("Template not found for component type: {0}")]
    TemplateNotFound(String),

    #[error("Invalid property value: {0}")]
    InvalidProperty(String),

    #[error("Code generation failed: {0}")]
    CodeGeneration(String),

    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
}
