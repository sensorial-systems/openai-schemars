//! A Rust utility to generate JSON Schemas compatible with OpenAI's function calling API, using [schemars](https://docs.rs/schemars/) and [serde_json](https://docs.rs/serde_json/).
//!
//! ## Example
//!
//! ```rust
//! use openai_schemars::{Schema, schemars};
//! use schemars::JsonSchema;
//!
//! #[derive(JsonSchema)]
//! struct MyStruct {
//!     /// The name of the user
//!     name: String,
//! }
//!
//! let schema = Schema::new::<MyStruct>().expect("Failed to generate schema");
//! println!("{}", serde_json::to_string_pretty(&schema.value).expect("Failed to serialize schema"));
//! ```

pub use schemars;

use schemars::schema_for;

/// A JSON Schema that is compatible with OpenAI's function calling API.
pub struct Schema {
    pub value: serde_json::Value,
}

impl Schema {
    pub fn new<T: schemars::JsonSchema>() -> Result<Self, serde_json::Error> {
        let schema = serde_json::to_value(schema_for!(T))?;
        let mut json_schema = Self { value: schema };
        json_schema.enforce_openai_subset();
        Ok(json_schema)
    }

    fn enforce_openai_subset(&mut self) {
        Self::remove_property_format_value_from_json(&mut self.value);
        Self::replace_one_of_by_any_of(&mut self.value);
        Self::set_additional_properties_to_false(&mut self.value);
        Self::enforce_all_required_properties(&mut self.value);
    }

    fn set_additional_properties_to_false(object: &mut serde_json::Value) {
        match object {
            serde_json::Value::Object(object) => {
                if object.get("type") == Some(&serde_json::Value::String("object".into())) {
                    object.insert("additionalProperties".into(), serde_json::Value::Bool(false));
                }
                for value in object.values_mut() {
                    Self::set_additional_properties_to_false(value);
                }
            }
            serde_json::Value::Array(array) => {
                for value in array.iter_mut() {
                    Self::set_additional_properties_to_false(value);
                }
            }
            _ => {}
        }
    }
    
    fn enforce_all_required_properties(object: &mut serde_json::Value) {
        match object {
            serde_json::Value::Object(object) => {
                let properties = object
                    .get_mut("properties")
                    .and_then(|properties| properties.as_object())
                    .map(|properties|
                        properties
                            .keys()
                            .map(|key| serde_json::Value::String(key.to_string()))
                            .collect::<Vec<_>>()
                    );
                if let (Some(required), Some(properties)) = (object.get_mut("required"), properties) {
                    if let Some(required) = required.as_array_mut() {
                        for property in properties {
                            if !required.contains(&property) {
                                required.push(property);
                            }
                        }
                    }
                }
                for value in object.values_mut() {
                    Self::enforce_all_required_properties(value);
                }
            },
            serde_json::Value::Array(array) => {
                for value in array.iter_mut() {
                    Self::enforce_all_required_properties(value);
                }
            }
            _ => {}
        }
    }
    
    fn replace_one_of_by_any_of(object: &mut serde_json::Value) {
        match object {
            serde_json::Value::Object(object) => {
                for key in ["oneOf", "allOf"] {
                    if object.contains_key(key) {
                        if let Some(value) = object.remove(key) {
                            object.insert("anyOf".into(), value);
                        }
                    }    
                }
                for value in object.values_mut() {
                    Self::replace_one_of_by_any_of(value);
                }
            }
            serde_json::Value::Array(array) => {
                for value in array.iter_mut() {
                    Self::replace_one_of_by_any_of(value);
                }
            }
            _ => {}
        }
    }
    
    fn remove_property_format_value_from_json(object: &mut serde_json::Value) {
        match object {
            serde_json::Value::Object(object) => {
                for key in ["minLength", "maxLength", "pattern", "format", "minimum", "maximum", "multipleOf", "patternProperties", "unevaluatedProperties", "propertyNames", "minProperties", "maxProperties", "unevaluatedItems", "contains", "minContains", "maxContains", "minItems", "maxItems", "uniqueItems"] {
                    object.remove(key);
                }
                for value in object.values_mut() {
                    Self::remove_property_format_value_from_json(value);
                }
            },
            serde_json::Value::Array(array) => {
                for value in array.iter_mut() {
                    Self::remove_property_format_value_from_json(value);
                }
            },
            _ => {}
        }
    }
}