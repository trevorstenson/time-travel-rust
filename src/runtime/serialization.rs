use deno_core::{v8, serde_json};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;

/// Represents a serialized JavaScript value that can be stored and reconstructed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JSValue {
    // Primitive types
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),
    String(String),
    BigInt(String), // Store as string to avoid precision issues
    Symbol(String), // Store symbol description
    
    // Object types
    Object(HashMap<String, JSValue>),
    Array(Vec<JSValue>),
    Function {
        name: String,
        source: Option<String>,
        location: Option<String>,
    },
    
    // Special object types
    Date(String), // ISO string representation
    RegExp {
        pattern: String,
        flags: String,
    },
    Map(Vec<(JSValue, JSValue)>), // Key-value pairs
    Set(Vec<JSValue>),
    
    // Error and circular reference handling
    Error(String),
    CircularReference(String), // Reference ID for circular structures
}

impl JSValue {
    /// Convert a V8 value to JSValue for serialization
    pub fn from_v8_value(
        scope: &mut v8::HandleScope,
        value: v8::Local<v8::Value>,
        max_depth: usize,
        circular_refs: &mut HashMap<usize, String>,
    ) -> Result<Self> {
        Self::from_v8_value_internal(scope, value, max_depth, 0, circular_refs)
    }

    fn from_v8_value_internal(
        scope: &mut v8::HandleScope,
        value: v8::Local<v8::Value>,
        max_depth: usize,
        current_depth: usize,
        circular_refs: &mut HashMap<usize, String>,
    ) -> Result<Self> {
        // Prevent infinite recursion
        if current_depth > max_depth {
            return Ok(JSValue::Error("Max depth exceeded".to_string()));
        }

        // Handle null and undefined
        if value.is_null() {
            return Ok(JSValue::Null);
        }
        if value.is_undefined() {
            return Ok(JSValue::Undefined);
        }

        // Handle primitives
        if value.is_boolean() {
            let bool_val = value.boolean_value(scope);
            return Ok(JSValue::Boolean(bool_val));
        }

        if value.is_number() {
            let num_val = value.number_value(scope).unwrap_or(0.0);
            return Ok(JSValue::Number(num_val));
        }

        if value.is_string() {
            let string_val = value.to_rust_string_lossy(scope);
            return Ok(JSValue::String(string_val));
        }

        if value.is_big_int() {
            // BigInt requires special handling
            if let Ok(bigint) = v8::Local::<v8::BigInt>::try_from(value) {
                let bigint_str = bigint.to_string(scope)
                    .map(|s| s.to_rust_string_lossy(scope))
                    .unwrap_or_else(|| "0n".to_string());
                return Ok(JSValue::BigInt(bigint_str));
            }
        }

        if value.is_symbol() {
            if let Ok(symbol) = v8::Local::<v8::Symbol>::try_from(value) {
                let desc_value = symbol.description(scope);
                let description = if desc_value.is_undefined() {
                    "Symbol()".to_string()
                } else {
                    desc_value.to_rust_string_lossy(scope)
                };
                return Ok(JSValue::Symbol(description));
            }
        }

        // Handle Date objects
        if value.is_date() {
            if let Ok(date) = v8::Local::<v8::Date>::try_from(value) {
                let timestamp = date.value_of();
                // Convert timestamp to ISO string
                let iso_string = format!("{}", chrono::DateTime::<chrono::Utc>::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_default()
                    .format("%Y-%m-%dT%H:%M:%S%.3fZ"));
                return Ok(JSValue::Date(iso_string));
            }
        }

        // Handle RegExp objects
        if value.is_reg_exp() {
            if let Ok(regexp) = v8::Local::<v8::RegExp>::try_from(value) {
                // For RegExp, we'll use toString() to get both pattern and flags
                let regexp_string = regexp.to_string(scope)
                    .map(|s| s.to_rust_string_lossy(scope))
                    .unwrap_or_else(|| "/unknown/".to_string());
                
                // Simple parsing of pattern and flags from string representation
                let (pattern, flags) = if regexp_string.starts_with('/') && regexp_string.len() > 2 {
                    if let Some(last_slash) = regexp_string.rfind('/') {
                        if last_slash > 0 {
                            let pattern = regexp_string[1..last_slash].to_string();
                            let flags = regexp_string[last_slash + 1..].to_string();
                            (pattern, flags)
                        } else {
                            (regexp_string, String::new())
                        }
                    } else {
                        (regexp_string, String::new())
                    }
                } else {
                    (regexp_string, String::new())
                };
                
                return Ok(JSValue::RegExp { pattern, flags });
            }
        }

        // Handle functions
        if value.is_function() {
            if let Ok(function) = v8::Local::<v8::Function>::try_from(value) {
                let name_value = function.get_name(scope);
                let name = if name_value.is_undefined() || name_value.to_rust_string_lossy(scope).is_empty() {
                    "anonymous".to_string()
                } else {
                    name_value.to_rust_string_lossy(scope)
                };
                
                // Try to get function source (this might not always work depending on V8 settings)
                let source = function.to_string(scope)
                    .map(|s| s.to_rust_string_lossy(scope));

                return Ok(JSValue::Function {
                    name,
                    source,
                    location: None, // Could be enhanced with source location info
                });
            }
        }

        // Handle arrays
        if value.is_array() {
            if let Ok(array) = v8::Local::<v8::Array>::try_from(value) {
                let length = array.length();
                let mut elements = Vec::with_capacity(length as usize);

                for i in 0..length {
                    if let Some(element) = array.get_index(scope, i) {
                        let serialized_element = Self::from_v8_value_internal(
                            scope, 
                            element, 
                            max_depth, 
                            current_depth + 1, 
                            circular_refs
                        )?;
                        elements.push(serialized_element);
                    } else {
                        elements.push(JSValue::Undefined);
                    }
                }

                return Ok(JSValue::Array(elements));
            }
        }

        // Handle Map objects
        if value.is_map() {
            if let Ok(map) = v8::Local::<v8::Map>::try_from(value) {
                let array = map.as_array(scope);
                let length = array.length();
                let mut entries = Vec::new();

                // Map.prototype.as_array() returns [key1, value1, key2, value2, ...]
                for i in (0..length).step_by(2) {
                    if let (Some(key), Some(value)) = (array.get_index(scope, i), array.get_index(scope, i + 1)) {
                        let serialized_key = Self::from_v8_value_internal(
                            scope, key, max_depth, current_depth + 1, circular_refs
                        )?;
                        let serialized_value = Self::from_v8_value_internal(
                            scope, value, max_depth, current_depth + 1, circular_refs
                        )?;
                        entries.push((serialized_key, serialized_value));
                    }
                }

                return Ok(JSValue::Map(entries));
            }
        }

        // Handle Set objects
        if value.is_set() {
            if let Ok(set) = v8::Local::<v8::Set>::try_from(value) {
                let array = set.as_array(scope);
                let length = array.length();
                let mut elements = Vec::new();

                for i in 0..length {
                    if let Some(element) = array.get_index(scope, i) {
                        let serialized_element = Self::from_v8_value_internal(
                            scope, element, max_depth, current_depth + 1, circular_refs
                        )?;
                        elements.push(serialized_element);
                    }
                }

                return Ok(JSValue::Set(elements));
            }
        }

        // Handle generic objects
        if value.is_object() {
            if let Ok(object) = v8::Local::<v8::Object>::try_from(value) {
                // Check for circular references
                let object_id = object.get_identity_hash();
                let object_id_key = object_id.get() as usize;
                if let Some(ref_id) = circular_refs.get(&object_id_key) {
                    return Ok(JSValue::CircularReference(ref_id.clone()));
                }

                // Mark this object in circular reference tracking
                let ref_id = format!("ref_{}", object_id);
                circular_refs.insert(object_id_key, ref_id.clone());

                let mut properties = HashMap::new();
                
                // Get object's own property names
                if let Some(property_names) = object.get_own_property_names(scope, v8::GetPropertyNamesArgs::default()) {
                    let length = property_names.length();
                    
                    for i in 0..length {
                        if let Some(key) = property_names.get_index(scope, i) {
                            let key_string = key.to_rust_string_lossy(scope);
                            
                            if let Some(property_value) = object.get(scope, key) {
                                let serialized_value = Self::from_v8_value_internal(
                                    scope, 
                                    property_value, 
                                    max_depth, 
                                    current_depth + 1, 
                                    circular_refs
                                )?;
                                properties.insert(key_string, serialized_value);
                            }
                        }
                    }
                }

                return Ok(JSValue::Object(properties));
            }
        }

        // Fallback for unknown types
        Ok(JSValue::Error(format!("Unsupported value type: {}", value.type_repr())))
    }

    /// Convert JSValue back to a JSON representation for display/debugging
    pub fn to_json_value(&self) -> serde_json::Value {
        match self {
            JSValue::Null => serde_json::Value::Null,
            JSValue::Undefined => serde_json::json!({ "type": "undefined" }),
            JSValue::Boolean(b) => serde_json::Value::Bool(*b),
            JSValue::Number(n) => {
                if n.is_finite() {
                    serde_json::json!(n)
                } else if n.is_infinite() {
                    if *n > 0.0 {
                        serde_json::json!({ "type": "number", "value": "Infinity" })
                    } else {
                        serde_json::json!({ "type": "number", "value": "-Infinity" })
                    }
                } else {
                    serde_json::json!({ "type": "number", "value": "NaN" })
                }
            },
            JSValue::String(s) => serde_json::Value::String(s.clone()),
            JSValue::BigInt(s) => serde_json::json!({ "type": "bigint", "value": s }),
            JSValue::Symbol(s) => serde_json::json!({ "type": "symbol", "description": s }),
            JSValue::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, value) in obj {
                    map.insert(key.clone(), value.to_json_value());
                }
                serde_json::Value::Object(map)
            },
            JSValue::Array(arr) => {
                let json_arr: Vec<serde_json::Value> = arr.iter()
                    .map(|v| v.to_json_value())
                    .collect();
                serde_json::Value::Array(json_arr)
            },
            JSValue::Function { name, source, location } => {
                serde_json::json!({
                    "type": "function",
                    "name": name,
                    "source": source,
                    "location": location
                })
            },
            JSValue::Date(iso) => serde_json::json!({ "type": "date", "value": iso }),
            JSValue::RegExp { pattern, flags } => {
                serde_json::json!({ "type": "regexp", "pattern": pattern, "flags": flags })
            },
            JSValue::Map(entries) => {
                let json_entries: Vec<serde_json::Value> = entries.iter()
                    .map(|(k, v)| serde_json::json!([k.to_json_value(), v.to_json_value()]))
                    .collect();
                serde_json::json!({ "type": "map", "entries": json_entries })
            },
            JSValue::Set(elements) => {
                let json_elements: Vec<serde_json::Value> = elements.iter()
                    .map(|v| v.to_json_value())
                    .collect();
                serde_json::json!({ "type": "set", "values": json_elements })
            },
            JSValue::Error(msg) => serde_json::json!({ "type": "error", "message": msg }),
            JSValue::CircularReference(ref_id) => {
                serde_json::json!({ "type": "circular_ref", "ref": ref_id })
            },
        }
    }

    /// Get a human-readable string representation of the value
    pub fn to_display_string(&self) -> String {
        match self {
            JSValue::Null => "null".to_string(),
            JSValue::Undefined => "undefined".to_string(),
            JSValue::Boolean(b) => b.to_string(),
            JSValue::Number(n) => {
                if n.is_finite() {
                    n.to_string()
                } else if n.is_infinite() {
                    if *n > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
                } else {
                    "NaN".to_string()
                }
            },
            JSValue::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            JSValue::BigInt(s) => format!("{}n", s),
            JSValue::Symbol(s) => format!("Symbol({})", s),
            JSValue::Object(obj) => {
                if obj.is_empty() {
                    "{}".to_string()
                } else {
                    let props: Vec<String> = obj.iter()
                        .take(3) // Limit for display
                        .map(|(k, v)| format!("{}: {}", k, v.to_display_string()))
                        .collect();
                    if obj.len() > 3 {
                        format!("{{ {}, ... }}", props.join(", "))
                    } else {
                        format!("{{ {} }}", props.join(", "))
                    }
                }
            },
            JSValue::Array(arr) => {
                if arr.is_empty() {
                    "[]".to_string()
                } else {
                    let elements: Vec<String> = arr.iter()
                        .take(3)
                        .map(|v| v.to_display_string())
                        .collect();
                    if arr.len() > 3 {
                        format!("[{}, ...]", elements.join(", "))
                    } else {
                        format!("[{}]", elements.join(", "))
                    }
                }
            },
            JSValue::Function { name, .. } => format!("function {}()", name),
            JSValue::Date(iso) => format!("Date({})", iso),
            JSValue::RegExp { pattern, flags } => format!("/{}/{}", pattern, flags),
            JSValue::Map(entries) => format!("Map({} entries)", entries.len()),
            JSValue::Set(elements) => format!("Set({} values)", elements.len()),
            JSValue::Error(msg) => format!("Error: {}", msg),
            JSValue::CircularReference(ref_id) => format!("[Circular: {}]", ref_id),
        }
    }
}

/// Configuration for value serialization
#[derive(Debug, Clone)]
pub struct SerializationConfig {
    pub max_depth: usize,
    pub max_string_length: usize,
    pub max_array_length: usize,
    pub max_object_properties: usize,
    pub capture_function_source: bool,
}

impl Default for SerializationConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            max_string_length: 1000,
            max_array_length: 100,
            max_object_properties: 50,
            capture_function_source: true,
        }
    }
}

/// Main serialization context that manages the conversion process
#[derive(Debug)]
pub struct SerializationContext {
    config: SerializationConfig,
    circular_refs: HashMap<usize, String>,
}

impl SerializationContext {
    pub fn new(config: SerializationConfig) -> Self {
        Self {
            config,
            circular_refs: HashMap::new(),
        }
    }

    /// Serialize a V8 value using this context
    pub fn serialize_value(
        &mut self,
        scope: &mut v8::HandleScope,
        value: v8::Local<v8::Value>,
    ) -> Result<JSValue> {
        self.circular_refs.clear(); // Reset circular reference tracking
        JSValue::from_v8_value(scope, value, self.config.max_depth, &mut self.circular_refs)
    }

    /// Serialize multiple values (e.g., function arguments)
    pub fn serialize_values(
        &mut self,
        scope: &mut v8::HandleScope,
        values: &[v8::Local<v8::Value>],
    ) -> Result<Vec<JSValue>> {
        self.circular_refs.clear();
        let mut results = Vec::with_capacity(values.len());
        
        for value in values {
            let serialized = JSValue::from_v8_value(
                scope, 
                *value, 
                self.config.max_depth, 
                &mut self.circular_refs
            )?;
            results.push(serialized);
        }
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsvalue_to_display_string() {
        assert_eq!(JSValue::Null.to_display_string(), "null");
        assert_eq!(JSValue::Undefined.to_display_string(), "undefined");
        assert_eq!(JSValue::Boolean(true).to_display_string(), "true");
        assert_eq!(JSValue::Number(42.0).to_display_string(), "42");
        assert_eq!(JSValue::String("hello".to_string()).to_display_string(), "\"hello\"");
    }

    #[test]
    fn test_jsvalue_to_json_value() {
        let val = JSValue::Object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), JSValue::String("test".to_string()));
            map.insert("age".to_string(), JSValue::Number(25.0));
            map
        });

        let json = val.to_json_value();
        assert!(json.is_object());
    }
} 