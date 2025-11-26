//! Rust code generator from OpenAPI schemas

use crate::parser::{OpenApiSpec, Schema};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Generate Rust types from OpenAPI spec
pub fn generate(spec: &OpenApiSpec, output_dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(output_dir)?;

    // Collect all definitions
    let definitions = collect_definitions(spec);

    // Group definitions by category
    let grouped = group_definitions(&definitions);

    // Generate module files
    let mut modules = Vec::new();

    for (category, defs) in &grouped {
        let module_name = category.to_snake_case();
        let file_path = output_dir.join(format!("{}.rs", module_name));

        let tokens = generate_module(category, defs)?;
        let formatted = format_code(tokens)?;

        fs::write(&file_path, formatted)?;
        modules.push(module_name);
        println!(
            "  Generated: {}.rs ({} types)",
            category.to_snake_case(),
            defs.len()
        );
    }

    // Generate mod.rs
    let mod_rs = generate_mod_rs(&modules, &definitions);
    fs::write(output_dir.join("mod.rs"), format_code(mod_rs)?)?;

    Ok(())
}

/// Collect all definitions from spec
fn collect_definitions(spec: &OpenApiSpec) -> BTreeMap<String, Schema> {
    let mut defs = spec.definitions.clone();

    // Also check components/schemas for OpenAPI 3.0
    if let Some(components) = &spec.components {
        for (name, schema) in &components.schemas {
            defs.insert(name.clone(), schema.clone());
        }
    }

    defs
}

/// Group definitions by category based on naming patterns
fn group_definitions(
    definitions: &BTreeMap<String, Schema>,
) -> BTreeMap<String, Vec<(String, Schema)>> {
    let mut grouped: BTreeMap<String, Vec<(String, Schema)>> = BTreeMap::new();

    for (name, schema) in definitions {
        let category = categorize_type(name);
        grouped
            .entry(category)
            .or_default()
            .push((name.clone(), schema.clone()));
    }

    grouped
}

/// Categorize a type by its name prefix
fn categorize_type(name: &str) -> String {
    // Common prefixes to categorize by
    let prefixes = [
        ("Instance", "instances"),
        ("Image", "images"),
        ("Network", "networks"),
        ("Storage", "storage"),
        ("Cluster", "cluster"),
        ("Certificate", "certificates"),
        ("Operation", "operations"),
        ("Profile", "profiles"),
        ("Project", "projects"),
        ("Server", "server"),
        ("Warning", "warnings"),
        ("Auth", "auth"),
        ("Identity", "auth"),
        ("Permission", "auth"),
        ("Placement", "placement"),
        ("Resource", "resources"),
        ("Metrics", "metrics"),
    ];

    for (prefix, category) in prefixes {
        if name.starts_with(prefix) {
            return category.to_string();
        }
    }

    "common".to_string()
}

/// Check if any definitions in a category use BTreeMap
fn uses_btreemap(definitions: &[(String, Schema)]) -> bool {
    for (_, schema) in definitions {
        if check_schema_uses_btreemap(schema) {
            return true;
        }
    }
    false
}

fn check_schema_uses_btreemap(schema: &Schema) -> bool {
    // Check if any property is a map/object with additionalProperties
    if schema.additional_properties.is_some() {
        return true;
    }

    // Check properties for nested maps
    for prop_schema in schema.properties.values() {
        if check_schema_uses_btreemap(prop_schema) {
            return true;
        }
    }

    // Check items for array types
    if let Some(items) = &schema.items {
        if check_schema_uses_btreemap(items) {
            return true;
        }
    }

    false
}

/// Escape HTML-like tags in documentation to prevent rustdoc warnings
fn escape_html_tags(doc: &str) -> String {
    // Replace angle brackets that look like HTML tags with backtick-wrapped versions
    // This handles patterns like <uefi-variable-name> or <UUID>
    let mut result = doc.to_string();

    // Find and escape patterns like <word> or <word-word>
    let re = regex::Regex::new(r"<([a-zA-Z][a-zA-Z0-9_-]*)>").unwrap();
    result = re.replace_all(&result, "`<$1>`").to_string();

    result
}

/// Generate a module file for a category
fn generate_module(
    category: &str,
    definitions: &[(String, Schema)],
) -> anyhow::Result<TokenStream> {
    let mut type_tokens = Vec::new();

    for (name, schema) in definitions {
        let type_def = generate_type_definition(name, schema)?;
        type_tokens.push(type_def);
    }

    let doc = format!("//! {} types for LXD API", category.to_upper_camel_case());

    // Only include BTreeMap import if actually used
    let btreemap_import = if uses_btreemap(definitions) {
        quote! { use std::collections::BTreeMap; }
    } else {
        quote! {}
    };

    Ok(quote! {
        #![doc = #doc]
        #![allow(clippy::derive_partial_eq_without_eq)]

        use serde::{Deserialize, Serialize};
        #btreemap_import

        // Import types from sibling modules
        #[allow(unused_imports)]
        use super::*;

        #(#type_tokens)*
    })
}

/// Generate a Rust type definition from a schema
fn generate_type_definition(name: &str, schema: &Schema) -> anyhow::Result<TokenStream> {
    let type_name = format_ident!("{}", name);
    let doc = escape_html_tags(schema.description.as_deref().unwrap_or("").trim());

    // Handle enum types
    if !schema.enum_values.is_empty() {
        return generate_enum_type(name, schema);
    }

    // Handle object types (structs)
    if schema.schema_type.as_deref() == Some("object") || !schema.properties.is_empty() {
        return generate_struct_type(name, schema);
    }

    // Handle allOf (composition)
    if !schema.all_of.is_empty() {
        return generate_all_of_type(name, schema);
    }

    // Default: generate type alias for simple types
    let rust_type = schema_to_rust_type(schema);

    // Only add doc comment if there's actual content
    let doc_attr = if doc.is_empty() {
        quote! {}
    } else {
        quote! { #[doc = #doc] }
    };

    Ok(quote! {
        #doc_attr
        pub type #type_name = #rust_type;
    })
}

/// Generate an enum type
fn generate_enum_type(name: &str, schema: &Schema) -> anyhow::Result<TokenStream> {
    let type_name = format_ident!("{}", name);
    let doc = escape_html_tags(schema.description.as_deref().unwrap_or("").trim());

    let variants: Vec<_> = schema
        .enum_values
        .iter()
        .filter_map(|v| {
            v.as_str().map(|s| {
                let variant_name = format_ident!("{}", s.to_upper_camel_case());
                let serde_rename = s;
                quote! {
                    #[serde(rename = #serde_rename)]
                    #variant_name
                }
            })
        })
        .collect();

    // Only add doc comment if there's actual content
    let doc_attr = if doc.is_empty() {
        quote! {}
    } else {
        quote! { #[doc = #doc] }
    };

    if variants.is_empty() {
        // If no string variants, just make it a string type alias
        return Ok(quote! {
            #doc_attr
            pub type #type_name = String;
        });
    }

    Ok(quote! {
        #doc_attr
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub enum #type_name {
            #(#variants),*
        }
    })
}

/// Generate a struct type
fn generate_struct_type(name: &str, schema: &Schema) -> anyhow::Result<TokenStream> {
    let type_name = format_ident!("{}", name);
    let doc = escape_html_tags(schema.description.as_deref().unwrap_or("").trim());

    let required_fields: BTreeSet<_> = schema.required.iter().cloned().collect();

    let fields: Vec<_> = schema
        .properties
        .iter()
        .map(|(field_name, field_schema)| {
            let rust_field_name = sanitize_field_name(field_name);
            let field_ident = format_ident!("{}", rust_field_name);
            let field_doc = escape_html_tags(field_schema.description.as_deref().unwrap_or("").trim());
            let is_required = required_fields.contains(field_name);

            let field_type = if is_required {
                schema_to_rust_type(field_schema)
            } else {
                let inner = schema_to_rust_type(field_schema);
                quote! { Option<#inner> }
            };

            // Handle serde rename if field name differs
            let serde_attr = if rust_field_name != *field_name {
                quote! { #[serde(rename = #field_name)] }
            } else {
                quote! {}
            };

            // Add default for optional fields
            let default_attr = if !is_required {
                quote! { #[serde(default, skip_serializing_if = "Option::is_none")] }
            } else {
                quote! {}
            };

            // Only add doc comment if there's actual content
            let doc_attr = if field_doc.is_empty() {
                quote! {}
            } else {
                quote! { #[doc = #field_doc] }
            };

            quote! {
                #doc_attr
                #serde_attr
                #default_attr
                pub #field_ident: #field_type
            }
        })
        .collect();

    // Only add doc comment if there's actual content
    let doc_attr = if doc.trim().is_empty() {
        quote! {}
    } else {
        quote! { #[doc = #doc] }
    };

    Ok(quote! {
        #doc_attr
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct #type_name {
            #(#fields),*
        }
    })
}

/// Generate type for allOf composition
fn generate_all_of_type(name: &str, schema: &Schema) -> anyhow::Result<TokenStream> {
    // Merge all properties from allOf schemas
    let mut merged_props: BTreeMap<String, Schema> = BTreeMap::new();
    let mut merged_required: Vec<String> = schema.required.clone();

    for sub_schema in &schema.all_of {
        if let Some(ref_name) = sub_schema.ref_name() {
            // Reference to another type - we'll handle this by embedding
            // For now, just note it
            println!("    Note: {} includes reference to {}", name, ref_name);
        }

        for (prop_name, prop_schema) in &sub_schema.properties {
            merged_props.insert(prop_name.clone(), prop_schema.clone());
        }

        merged_required.extend(sub_schema.required.clone());
    }

    // Also include direct properties
    for (prop_name, prop_schema) in &schema.properties {
        merged_props.insert(prop_name.clone(), prop_schema.clone());
    }

    let merged = Schema {
        properties: merged_props,
        required: merged_required,
        description: schema.description.clone(),
        ..Default::default()
    };

    generate_struct_type(name, &merged)
}

/// Convert an OpenAPI schema to a Rust type
fn schema_to_rust_type(schema: &Schema) -> TokenStream {
    // Handle references first
    if let Some(ref_name) = schema.ref_name() {
        let type_name = format_ident!("{}", ref_name);
        return quote! { #type_name };
    }

    match schema.schema_type.as_deref() {
        Some("string") => {
            // Could use chrono::DateTime<Utc> with feature flag for date-time format
            quote! { String }
        }
        Some("integer") => match schema.format.as_deref() {
            Some("int32") => quote! { i32 },
            Some("int64") => quote! { i64 },
            _ => quote! { i64 },
        },
        Some("number") => match schema.format.as_deref() {
            Some("float") => quote! { f32 },
            Some("double") => quote! { f64 },
            _ => quote! { f64 },
        },
        Some("boolean") => quote! { bool },
        Some("array") => {
            if let Some(items) = &schema.items {
                let item_type = schema_to_rust_type(items);
                quote! { Vec<#item_type> }
            } else {
                quote! { Vec<serde_json::Value> }
            }
        }
        Some("object") => {
            if let Some(additional) = &schema.additional_properties {
                let value_type = schema_to_rust_type(additional);
                quote! { BTreeMap<String, #value_type> }
            } else if schema.properties.is_empty() {
                quote! { BTreeMap<String, serde_json::Value> }
            } else {
                // This is a nested object - for now, use Value
                quote! { serde_json::Value }
            }
        }
        None if schema.additional_properties.is_some() => {
            let additional = schema.additional_properties.as_ref().unwrap();
            let value_type = schema_to_rust_type(additional);
            quote! { BTreeMap<String, #value_type> }
        }
        _ => quote! { serde_json::Value },
    }
}

/// Sanitize a field name to be a valid Rust identifier
fn sanitize_field_name(name: &str) -> String {
    let snake = name.to_snake_case();

    // Handle Rust keywords
    match snake.as_str() {
        "type" => "kind".to_string(),
        "self" => "self_".to_string(),
        "ref" => "ref_".to_string(),
        "mod" => "mod_".to_string(),
        "use" => "use_".to_string(),
        "fn" => "fn_".to_string(),
        "async" => "async_".to_string(),
        "await" => "await_".to_string(),
        "loop" => "loop_".to_string(),
        "move" => "move_".to_string(),
        _ => snake,
    }
}

/// Generate the mod.rs file
fn generate_mod_rs(modules: &[String], definitions: &BTreeMap<String, Schema>) -> TokenStream {
    let module_idents: Vec<_> = modules.iter().map(|m| format_ident!("{}", m)).collect();

    // Re-export all types
    let reexports: Vec<_> = modules
        .iter()
        .map(|m| {
            let mod_ident = format_ident!("{}", m);
            quote! { pub use #mod_ident::*; }
        })
        .collect();

    let type_count = definitions.len();
    let doc = format!(
        "Generated LXD API types\n\nThis module contains Rust type definitions generated from the LXD Swagger 2.0 specification.\nTotal types generated: {}",
        type_count
    );

    quote! {
        #![doc = #doc]

        #(pub mod #module_idents;)*

        #(#reexports)*
    }
}

/// Format generated code using rustfmt-style formatting
fn format_code(tokens: TokenStream) -> anyhow::Result<String> {
    let file = syn::parse_file(&tokens.to_string());
    match file {
        Ok(parsed) => Ok(prettyplease::unparse(&parsed)),
        Err(_) => {
            // If parsing fails, return the raw tokens
            Ok(tokens.to_string())
        }
    }
}
