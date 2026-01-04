use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::conversational_ai::PlatformTarget;
use crate::ui_generator::UIComponent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPlatformAdapter {
    platform_targets: Vec<PlatformTarget>,
    adaptations: HashMap<PlatformTarget, PlatformAdaptations>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformAdaptations {
    pub styles: HashMap<String, StyleAdaptation>,
    pub components: HashMap<String, ComponentAdaptation>,
    pub interactions: HashMap<String, InteractionAdaptation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleAdaptation {
    pub property: String,
    pub values: HashMap<PlatformTarget, String>,
    pub conditions: Vec<AdaptationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentAdaptation {
    pub component_type: String,
    pub platform_specific: HashMap<PlatformTarget, ComponentOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOverride {
    pub tag: Option<String>,
    pub attributes: HashMap<String, String>,
    pub styles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionAdaptation {
    pub event: String,
    pub handlers: HashMap<PlatformTarget, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationCondition {
    pub platform: PlatformTarget,
    pub condition: String,
    pub value: String,
}

impl CrossPlatformAdapter {
    pub fn new() -> Self {
        Self {
            platform_targets: vec![PlatformTarget::Web, PlatformTarget::Desktop, PlatformTarget::Mobile],
            adaptations: Self::initialize_adaptations(),
        }
    }

    pub fn adapt_component(&self, component: &mut UIComponent, target_platform: PlatformTarget) {
        // Apply platform-specific adaptations to the component
        if let Some(adaptations) = self.adaptations.get(&target_platform) {
            self.apply_style_adaptations(component, adaptations, target_platform);
            self.apply_component_adaptations(component, adaptations, target_platform);
            self.apply_interaction_adaptations(component, adaptations);
        }
    }

    pub fn generate_responsive_code(&self, component: &UIComponent) -> String {
        let mut code = String::new();

        // Generate CSS media queries for responsive design
        code.push_str(&self.generate_media_queries(component));

        // Generate platform-specific conditional rendering
        code.push_str(&self.generate_conditional_rendering(component));

        code
    }

    fn apply_style_adaptations(
        &self,
        component: &mut UIComponent,
        adaptations: &PlatformAdaptations,
        target_platform: PlatformTarget,
    ) {
        for (property, adaptation) in &adaptations.styles {
            let value = adaptation
                .values
                .get(&target_platform)
                .or_else(|| adaptation.values.get(&PlatformTarget::Universal));

            if let Some(value) = value {
                component
                    .properties
                    .insert(property.clone(), serde_json::Value::String(value.clone()));
            }
        }
    }

    fn apply_component_adaptations(
        &self,
        component: &mut UIComponent,
        adaptations: &PlatformAdaptations,
        target_platform: PlatformTarget,
    ) {
        // Apply component-level adaptations
        let component_key = format!("{:?}", component.component_type);
        if let Some(component_adaptation) = adaptations.components.get(&component_key) {
            // Apply universal overrides first, then platform-specific overrides.
            if let Some(override_configs) = component_adaptation
                .platform_specific
                .get(&PlatformTarget::Universal)
            {
                self.apply_component_override(component, override_configs);
            }
            if let Some(override_configs) = component_adaptation.platform_specific.get(&target_platform) {
                self.apply_component_override(component, override_configs);
            }
        }
    }

    fn apply_component_override(&self, component: &mut UIComponent, override_configs: &ComponentOverride) {
        // Tag overrides are consumed by the code generator via the `tag` property.
        if let Some(tag) = &override_configs.tag {
            component
                .properties
                .insert("tag".to_string(), serde_json::Value::String(tag.clone()));
        }

        // Treat attributes as plain string properties.
        for (attr, value) in &override_configs.attributes {
            component
                .properties
                .insert(attr.clone(), serde_json::Value::String(value.clone()));
        }

        // Merge styles into a nested `styles` object property.
        if !override_configs.styles.is_empty() {
            let mut merged = match component.properties.remove("styles") {
                Some(serde_json::Value::Object(map)) => map,
                _ => serde_json::Map::new(),
            };

            for (k, v) in &override_configs.styles {
                merged.insert(k.clone(), serde_json::Value::String(v.clone()));
            }

            component
                .properties
                .insert("styles".to_string(), serde_json::Value::Object(merged));
        }
    }

    fn apply_interaction_adaptations(&self, _component: &mut UIComponent, _adaptations: &PlatformAdaptations) {
        // Apply interaction adaptations
        // This would modify the component's event handlers based on platform
    }

    fn generate_media_queries(&self, component: &UIComponent) -> String {
        let mut css = String::new();

        // Generate responsive breakpoints
        css.push_str(&format!(
            r#"
            @media (max-width: 768px) {{
                .component-{0} {{
                    /* Mobile styles */
                }}
            }}
            
            @media (min-width: 769px) and (max-width: 1024px) {{
                .component-{0} {{
                    /* Tablet styles */
                }}
            }}
            
            @media (min-width: 1025px) {{
                .component-{0} {{
                    /* Desktop styles */
                }}
            }}
        "#,
            component.id
        ));

        css
    }

    fn generate_conditional_rendering(&self, component: &UIComponent) -> String {
        let mut code = String::new();

        // Generate platform-specific conditional rendering
        code.push_str(&format!(
            r#"
            #[component]
            pub fn {}() -> Element {{
                let platform = use_platform(); // Custom hook to detect platform
                
                rsx! {{
                    match platform {{
                        Platform::Web => {{
                            // Web-specific rendering
                        }}
                        Platform::Desktop => {{
                            // Desktop-specific rendering
                        }}
                        Platform::Mobile => {{
                            // Mobile-specific rendering
                        }}
                    }}
                }}
            }}
        "#,
            self.get_component_name(component)
        ));

        code
    }

    fn get_component_name(&self, component: &UIComponent) -> String {
        format!("Component_{}_{:?}", component.id, component.component_type)
    }

    fn initialize_adaptations() -> HashMap<PlatformTarget, PlatformAdaptations> {
        let mut adaptations = HashMap::new();

        // Web adaptations
        adaptations.insert(
            PlatformTarget::Web,
            PlatformAdaptations {
                styles: Self::initialize_web_styles(),
                components: Self::initialize_web_components(),
                interactions: Self::initialize_web_interactions(),
            },
        );

        // Desktop adaptations
        adaptations.insert(
            PlatformTarget::Desktop,
            PlatformAdaptations {
                styles: Self::initialize_desktop_styles(),
                components: Self::initialize_desktop_components(),
                interactions: Self::initialize_desktop_interactions(),
            },
        );

        // Mobile adaptations
        adaptations.insert(
            PlatformTarget::Mobile,
            PlatformAdaptations {
                styles: Self::initialize_mobile_styles(),
                components: Self::initialize_mobile_components(),
                interactions: Self::initialize_mobile_interactions(),
            },
        );

        adaptations
    }

    fn initialize_web_styles() -> HashMap<String, StyleAdaptation> {
        let mut styles = HashMap::new();

        styles.insert(
            "touch_target_size".to_string(),
            StyleAdaptation {
                property: "min-height".to_string(),
                values: Self::initialize_web_touch_targets(),
                conditions: vec![],
            },
        );

        styles
    }

    fn initialize_web_components() -> HashMap<String, ComponentAdaptation> {
        HashMap::new()
    }

    fn initialize_web_interactions() -> HashMap<String, InteractionAdaptation> {
        HashMap::new()
    }

    fn initialize_desktop_styles() -> HashMap<String, StyleAdaptation> {
        HashMap::new()
    }

    fn initialize_desktop_components() -> HashMap<String, ComponentAdaptation> {
        HashMap::new()
    }

    fn initialize_desktop_interactions() -> HashMap<String, InteractionAdaptation> {
        HashMap::new()
    }

    fn initialize_mobile_styles() -> HashMap<String, StyleAdaptation> {
        HashMap::new()
    }

    fn initialize_mobile_components() -> HashMap<String, ComponentAdaptation> {
        HashMap::new()
    }

    fn initialize_mobile_interactions() -> HashMap<String, InteractionAdaptation> {
        HashMap::new()
    }

    fn initialize_web_touch_targets() -> HashMap<PlatformTarget, String> {
        let mut targets = HashMap::new();
        targets.insert(PlatformTarget::Web, "44px".to_string());
        targets.insert(PlatformTarget::Desktop, "32px".to_string());
        targets.insert(PlatformTarget::Mobile, "48px".to_string());
        targets
    }
}

impl Default for CrossPlatformAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CrossPlatformError {
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    #[error("Adaptation failed: {0}")]
    AdaptationFailed(String),
}
