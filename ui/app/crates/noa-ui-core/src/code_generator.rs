use crate::conversational_ai::PlatformTarget;
use crate::ui_generator::{ComponentType, UIComponent};
use std::collections::HashMap;

pub struct CodeGenerator {
    _templates: HashMap<String, String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            _templates: Self::initialize_templates(),
        }
    }

    pub fn generate_component_code(
        &self,
        component: &UIComponent,
        platform: PlatformTarget,
    ) -> Result<String, CodeGenError> {
        match platform {
            PlatformTarget::Web => self.generate_web_code(component),
            PlatformTarget::Desktop => self.generate_desktop_code(component),
            PlatformTarget::Mobile => self.generate_mobile_code(component),
            PlatformTarget::Universal => self.generate_universal_code(component),
        }
    }

    fn generate_web_code(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let mut code = String::new();

        // Generate component based on type
        match component.component_type {
            ComponentType::Button => {
                code.push_str(&self.generate_web_button(component)?);
            }
            ComponentType::Text => {
                code.push_str(&self.generate_web_text(component)?);
            }
            ComponentType::Input => {
                code.push_str(&self.generate_web_input(component)?);
            }
            ComponentType::Container => {
                code.push_str(&self.generate_web_container(component)?);
            }
            _ => {
                code.push_str(&self.generate_web_generic(component)?);
            }
        }

        Ok(code)
    }

    fn generate_desktop_code(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let mut code = String::new();

        // Generate Dioxus desktop component
        code.push_str(&format!(
            r#"
            #[component]
            pub fn {}() -> Element {{
                rsx! {{
            "#,
            self.get_component_name(component)
        ));

        match component.component_type {
            ComponentType::Button => {
                code.push_str(&self.generate_desktop_button(component)?);
            }
            ComponentType::Text => {
                code.push_str(&self.generate_desktop_text(component)?);
            }
            ComponentType::Input => {
                code.push_str(&self.generate_desktop_input(component)?);
            }
            ComponentType::Container => {
                code.push_str(&self.generate_desktop_container(component)?);
            }
            _ => {
                code.push_str(&self.generate_desktop_generic(component)?);
            }
        }

        code.push_str("        }\n    }\n");

        Ok(code)
    }

    fn generate_mobile_code(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        // Similar to desktop but with mobile-specific adaptations
        self.generate_desktop_code(component)
    }

    fn generate_universal_code(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        // Generate code that works across all platforms
        let mut code = String::new();

        code.push_str(&format!(
            r#"
            #[component]
            pub fn {}() -> Element {{
                rsx! {{
            "#,
            self.get_component_name(component)
        ));

        // Add platform-specific adaptations
        code.push_str(&self.generate_platform_adaptations(component)?);

        code.push_str("        }\n    }\n");

        Ok(code)
    }

    fn generate_web_button(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let text = self.get_property(component, "text", "Click me");
        let class_name = self.get_css_class(component);

        Ok(format!(
            r#"
            button {{
                class: \"{}\",
                onclick: move |_| {{
                    // Button click handler
                }},
                \"{}\"
            }}
        "#,
            class_name, text
        ))
    }

    fn generate_web_text(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let text = self.get_property(component, "text", "Sample text");
        let class_name = self.get_css_class(component);
        let tag = self.get_property(component, "tag", "p");

        Ok(format!(
            r#"
            {} {{
                class: \"{}\",
                \"{}\"
            }}
        "#,
            tag, class_name, text
        ))
    }

    fn generate_web_input(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let placeholder = self.get_property(component, "placeholder", "Enter text...");
        let input_type = self.get_property(component, "type", "text");
        let class_name = self.get_css_class(component);

        Ok(format!(
            r#"
            input {{
                class: \"{}\",
                placeholder: \"{}\",
                r#type: \"{}\",
                oninput: move |event| {{
                    // Input change handler
                }}
            }}
        "#,
            class_name, placeholder, input_type
        ))
    }

    fn generate_web_container(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let class_name = self.get_css_class(component);
        let children = self.generate_children_code(component)?;

        Ok(format!(
            r#"
            div {{
                class: \"{}\",
                {}
            }}
        "#,
            class_name, children
        ))
    }

    fn generate_web_generic(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let class_name = self.get_css_class(component);
        let children = self.generate_children_code(component)?;

        Ok(format!(
            r#"
            div {{
                class: \"{}\",
                \"{}\"
            }}
        "#,
            class_name, children
        ))
    }

    fn generate_desktop_button(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let text = self.get_property(component, "text", "Click me");

        Ok(format!(
            r#"
            button {{
                onclick: move |_| {{
                    // Button click handler
                }},
                \"{}\"
            }}
        "#,
            text
        ))
    }

    fn generate_desktop_text(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let text = self.get_property(component, "text", "Sample text");

        Ok(format!(
            r#"
            \"{}\"
        "#,
            text
        ))
    }

    fn generate_desktop_input(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let placeholder = self.get_property(component, "placeholder", "Enter text...");

        Ok(format!(
            r#"
            input {{
                placeholder: \"{}\",
                oninput: move |event| {{
                    // Input change handler
                }}
            }}
        "#,
            placeholder
        ))
    }

    fn generate_desktop_container(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let children = self.generate_children_code(component)?;

        Ok(format!(
            r#"
            div {{
                {}
            }}
        "#,
            children
        ))
    }

    fn generate_desktop_generic(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let children = self.generate_children_code(component)?;

        Ok(format!(
            r#"
            div {{
                {}
            }}
        "#,
            children
        ))
    }

    fn generate_platform_adaptations(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let mut adaptations = String::new();

        // Add responsive design adaptations
        adaptations.push_str(&format!(
            r#"
            // Platform-specific adaptations for {:?}
            div {{
                class: \"platform-adaptive {}\",
                
        "#,
            component.component_type,
            self.get_component_name(component)
        ));

        // Add children
        adaptations.push_str(&self.generate_children_code(component)?);
        adaptations.push_str("            }}\n");

        Ok(adaptations)
    }

    fn generate_children_code(&self, component: &UIComponent) -> Result<String, CodeGenError> {
        let mut children_code = String::new();

        for child in &component.children {
            let child_code = self.generate_component_code(child, PlatformTarget::Universal)?;
            children_code.push_str(&format!("{}\n", child_code));
        }

        Ok(children_code)
    }

    fn get_component_name(&self, component: &UIComponent) -> String {
        format!("Component_{}_{:?}", component.id, component.component_type)
    }

    fn get_property(&self, component: &UIComponent, key: &str, default: &str) -> String {
        component
            .properties
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    }

    fn get_css_class(&self, component: &UIComponent) -> String {
        format!("component-{}-{:?}", component.id, component.component_type)
    }

    fn initialize_templates() -> HashMap<String, String> {
        // NOTE: The original rust-lovable prototype used include_str! templates.
        // For the consolidated NOA workspace we keep this map empty until we
        // decide where templates should live under ui/app.
        HashMap::new()
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CodeGenError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Invalid component type: {0}")]
    InvalidComponentType(String),

    #[error("Property missing: {0}")]
    PropertyMissing(String),

    #[error("Code generation failed: {0}")]
    GenerationFailed(String),
}
