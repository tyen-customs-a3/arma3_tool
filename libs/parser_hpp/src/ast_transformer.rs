use gamedata_scanner_models::{ClassProperty, GameClass, PropertyValue};
use hemtt_config::{Class, Config as HemttConfig, Item, Number as HemttNumber, Property, Value};
use hemtt_workspace::reporting::Processed;
use log::trace;
use std::path::Path;

pub fn transform_config_to_game_classes(
    config_ast: &HemttConfig,
    processed_output: &Processed,
    _original_file_path: &Path, // Keep for now, might remove if unused
    project_root_dir: &Path,
) -> Vec<GameClass> {
    let mut classes = Vec::new();
    Transformer::new(_original_file_path, processed_output, project_root_dir)
        .extract_classes_from_config(config_ast, &mut classes);
    classes
}

struct Transformer<'a> {
    original_file_path: &'a Path, // Fallback or initial context
    processed_output: &'a Processed,
    project_root_dir: &'a Path,
}

impl<'a> Transformer<'a> {
    fn new(original_file_path: &'a Path, processed_output: &'a Processed, project_root_dir: &'a Path) -> Self {
        Self {
            original_file_path,
            processed_output,
            project_root_dir
        }
    }

    fn extract_classes_from_config(&self, config: &HemttConfig, classes: &mut Vec<GameClass>) {
        // First pass: Process all forward declarations and base classes (no parent)
        for property in config.0.iter() {
            if let Property::Class(class) = property {
                match class {
                    Class::External { name, .. } => {
                        if !classes.iter().any(|c| c.name == name.as_str()) {
                            let name_span = name.span();
                            let file_path_for_external = self.processed_output
                                .mapping(name_span.start)
                                .map(|mapping| {
                                    let original_wpath = mapping.original().path();
                                    let wpath_str = original_wpath.as_str();
                                    let clean_path = if wpath_str.starts_with('/') {
                                        &wpath_str[1..] // Remove leading slash
                                    } else {
                                        wpath_str
                                    };
                                    self.project_root_dir.join(clean_path)
                                })
                                .unwrap_or_else(|| {
                                    log::warn!(
                                        "Could not find mapping for external class '{}' (span {:?}), using initial file path: {}",
                                        name.as_str(), name_span, self.original_file_path.display()
                                    );
                                    self.original_file_path.to_path_buf()
                                });

                            classes.push(GameClass {
                                name: name.as_str().to_string(),
                                parent: None,
                                properties: Vec::new(),
                                container_class: None,
                                file_path: file_path_for_external,
                                is_forward_declaration: true,
                            });
                        }
                    }
                    Class::Local {
                        name,
                        parent,
                        properties,
                        ..
                    } => {
                        if parent.is_none() {
                            // Base classes
                            let class_def = self.create_game_class(
                                name,
                                None,
                                properties,
                                classes,
                                None, // No container for top-level base
                            );
                            // Add if not exists (or decide on update strategy for re-definitions)
                            if !classes
                                .iter()
                                .any(|c| c.name == class_def.name && c.container_class.is_none())
                            {
                                classes.push(class_def);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Second pass: Process classes with inheritance and all nested classes
        for property in config.0.iter() {
            if let Property::Class(class) = property {
                match class {
                    Class::Local {
                        name,
                        parent,
                        properties,
                        ..
                    } => {
                        let game_class = self.create_game_class(
                            name,
                            parent.as_ref(),
                            properties,
                            classes,
                            None, // Top-level class, no container
                        );

                        // Update if exists, else add
                        if let Some(idx) = classes
                            .iter()
                            .position(|c| c.name == name.as_str() && c.container_class.is_none())
                        {
                            classes[idx] = game_class;
                        } else {
                            classes.push(game_class);
                        }
                    }
                    Class::Root { properties, .. } => {
                        // Root classes like CfgPatches, CfgVehicles directly contain other classes
                        for prop in properties.iter() {
                            if let Property::Class(Class::Local {
                                name,
                                parent,
                                properties: local_props,
                                ..
                            }) = prop
                            {
                                // This 'name' is a CfgPatches, CfgVehicles, etc.
                                // It acts as a top-level class itself.
                                let root_item_class = self.create_game_class(
                                    name,
                                    parent.as_ref(),
                                    local_props,
                                    classes,
                                    None, // This is a top-level item like CfgVehicles
                                );
                                if !classes.iter().any(|c| {
                                    c.name == name.as_str() && c.container_class.is_none()
                                }) {
                                    classes.push(root_item_class);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn create_game_class(
        &self,
        name_ident: &hemtt_config::Ident, // Changed from &str
        parent_name_ident: Option<&hemtt_config::Ident>, // Changed from Option<&str>
        hemtt_properties: &[Property],
        all_classes_vec: &mut Vec<GameClass>, // Used to add nested classes directly
        container_class_name: Option<&str>,
    ) -> GameClass {
        let name = name_ident.as_str(); // Get string name
        let parent_name = parent_name_ident.map(|p| p.as_str()); // Get string parent name

        let name_span = name_ident.span();
        trace!(
            "Creating game class: {}, span: {:?}, initial file: {}",
            name,
            name_span,
            self.original_file_path.display()
        );

        let file_path_for_class = self.processed_output
            .mapping(name_span.start)
            .map(|mapping| {
                let original_wpath = mapping.original().path();
                trace!("  Class '{}' mapped to original file: {}", name, original_wpath.as_str());
                // Handle the case where WorkspacePath might start with '/' which would be treated as absolute
                let wpath_str = original_wpath.as_str();
                let clean_path = if wpath_str.starts_with('/') {
                    &wpath_str[1..] // Remove leading slash
                } else {
                    wpath_str
                };
                self.project_root_dir.join(clean_path)
            })
            .unwrap_or_else(|| {
                log::warn!(
                    "Could not find mapping for class '{}' (name span {:?}), using initial file path: {}",
                    name, name_span, self.original_file_path.display()
                );
                self.original_file_path.to_path_buf()
            });
        trace!("  Final file_path for class '{}': {}", name, file_path_for_class.display());

        let mut game_class = GameClass {
            name: name.to_string(),
            parent: parent_name.map(String::from),
            properties: Vec::new(),
            container_class: container_class_name.map(String::from),
            file_path: file_path_for_class, // Use resolved path
            is_forward_declaration: false, // This function creates full definitions
        };

        for prop in hemtt_properties {
            match prop {
                Property::Entry {
                    name: prop_name,
                    value,
                    ..
                } => {
                    trace!("  Adding property to {}: {}", name, prop_name.as_str());
                    game_class.properties.push(ClassProperty {
                        name: prop_name.as_str().to_string(),
                        value: self.convert_hemtt_value_to_property_value(value),
                    });
                }
                Property::Class(nested_hemtt_class) => {
                    match nested_hemtt_class {
                        Class::Local {
                            name: nested_name_ident,
                            parent: nested_parent_ident,
                            properties: nested_props,
                            ..
                        } => {
                            let nested_game_class = self.create_game_class(
                                nested_name_ident,
                                nested_parent_ident.as_ref(),
                                nested_props,
                                all_classes_vec,
                                Some(name), // The current class 'name' is the container
                            );

                            // Add nested class as a property of its container
                            game_class.properties.push(ClassProperty {
                                name: nested_game_class.name.clone(),
                                value: PropertyValue::Class(Box::new(nested_game_class.clone())),
                            });

                            // Also add the nested class to the global list if not already present with this container
                            let nested_name_str = nested_name_ident.as_str();
                            if !all_classes_vec.iter().any(|c| {
                                c.name == nested_name_str
                                    && c.container_class.as_deref() == Some(name)
                            }) {
                                all_classes_vec.push(nested_game_class);
                            }
                        }
                        Class::External {
                            name: nested_external_name_ident,
                            ..
                        } => {
                            // This is a nested forward declaration, e.g., `class CfgAmmo { class BulletBase; };`
                            let fwd_decl_span = nested_external_name_ident.span();
                            let fwd_decl_file_path = self.processed_output
                                .mapping(fwd_decl_span.start)
                                .map(|mapping| {
                                    let original_wpath = mapping.original().path();
                                    let wpath_str = original_wpath.as_str();
                                    let clean_path = if wpath_str.starts_with('/') {
                                        &wpath_str[1..] // Remove leading slash
                                    } else {
                                        wpath_str
                                    };
                                    self.project_root_dir.join(clean_path)
                                })
                                .unwrap_or_else(|| {
                                    log::warn!(
                                        "Could not find mapping for forward decl class '{}' (span {:?}), using initial file path: {}",
                                        nested_external_name_ident.as_str(), fwd_decl_span, self.original_file_path.display()
                                    );
                                    self.original_file_path.to_path_buf()
                                });

                            let nested_forward_decl_game_class = GameClass {
                                name: nested_external_name_ident.as_str().to_string(),
                                parent: None,
                                properties: Vec::new(),
                                container_class: Some(name.to_string()), // `name` is the current class (e.g. "CfgAmmo")
                                file_path: fwd_decl_file_path,
                                is_forward_declaration: true,
                            };

                            game_class.properties.push(ClassProperty {
                                name: nested_forward_decl_game_class.name.clone(),
                                value: PropertyValue::Class(Box::new(
                                    nested_forward_decl_game_class.clone(),
                                )),
                            });

                            // Also add to the global list
                            let nested_fwd_name_str = nested_external_name_ident.as_str();
                            if !all_classes_vec.iter().any(|c| {
                                c.name == nested_fwd_name_str
                                    && c.container_class.as_deref() == Some(name)
                            }) {
                                all_classes_vec.push(nested_forward_decl_game_class);
                            }
                        }
                        // Other hemtt_config::ast::Class variants (Root, Unknown) are not expected
                        // as direct results of `Property::Class` inside another class's properties array.
                        // If they were, they'd need handling here.
                        _ => {
                            // Simplified to remove trace macro for debugging syntax issues
                        }
                    }
                }
                _ => {} // Handle Property::Delete, Property::MissingSemicolon, Property::Enum as needed or ignore
            } // Closes match prop
        } // Closes for prop in hemtt_properties
        game_class // Return game_class
    } // Closes fn create_game_class

    fn convert_hemtt_value_to_property_value(&self, value: &Value) -> PropertyValue {
        match value {
            Value::Str(s) => PropertyValue::String(s.value().to_string()),
            Value::Number(n) => match n {
                HemttNumber::Int32 { value, .. } => PropertyValue::Number(*value as i64),
                HemttNumber::Int64 { value, .. } => PropertyValue::Number(*value),
                HemttNumber::Float32 { value, .. } => PropertyValue::Number(*value as i64), // Consider if f32 should be stored differently
            },
            Value::Array(arr) => PropertyValue::Array(
                arr.items
                    .iter()
                    .map(|item| self.convert_hemtt_item_to_string(item))
                    .collect(),
            ),
            Value::Expression(e) => PropertyValue::String(format!("EXPRESSION: {:?}", e)), // Represent expression
            Value::Macro(m) => PropertyValue::String(m.to_string()), // Use MacroExpression's to_string
            Value::UnexpectedArray(arr) => {
                log::warn!(
                    "Encountered UnexpectedArray, converting to string array: {:?}",
                    arr.span
                );
                PropertyValue::Array(
                    arr.items
                        .iter()
                        .map(|item| self.convert_hemtt_item_to_string(item))
                        .collect(),
                )
            }
            Value::Invalid(range) => {
                log::warn!("Encountered Invalid Hemtt Value at range {:?}, representing as empty string", range);
                PropertyValue::String("INVALID_HEMTT_VALUE".to_string()) // Or a more descriptive placeholder
            }
        }
    }

    fn convert_hemtt_item_to_string(&self, item: &Item) -> String {
        match item {
            Item::Str(s) => s.value().to_string(),
            Item::Number(n) => n.to_string(),
            Item::Macro(m) => m.to_string(),
            Item::Array(items_vec) => {
                // Recursively convert nested arrays, e.g., "{elem1, elem2, {sub1, sub2}}"
                let inner_items: Vec<String> = items_vec
                    .iter()
                    .map(|sub_item| self.convert_hemtt_item_to_string(sub_item))
                    .collect();
                format!("{{{}}}", inner_items.join(", "))
            }
            Item::Invalid(range) => {
                log::warn!(
                    "Encountered Invalid Hemtt Item at range {:?}, representing as placeholder",
                    range
                );
                "INVALID_HEMTT_ITEM".to_string()
            }
        }
    }
}