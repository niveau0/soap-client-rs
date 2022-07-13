//! Type mapping from XSD types to Rust types

use crate::parser::QName;
use std::collections::HashMap;

/// Maps XML Schema types to Rust types
pub struct TypeMapper {
    /// Custom type mappings (QName -> Rust type)
    custom_mappings: HashMap<String, String>,
}

impl TypeMapper {
    /// Create a new type mapper with default XSD type mappings
    pub fn new() -> Self {
        Self {
            custom_mappings: HashMap::new(),
        }
    }

    /// Add a custom type mapping
    pub fn add_mapping(&mut self, xsd_type: impl Into<String>, rust_type: impl Into<String>) {
        self.custom_mappings
            .insert(xsd_type.into(), rust_type.into());
    }

    /// Map an XSD type to a Rust type
    pub fn map_type(&self, qname: &QName) -> String {
        // Check custom mappings first
        if let Some(rust_type) = self.custom_mappings.get(qname.as_str()) {
            return rust_type.clone();
        }

        // Map based on local name (ignoring prefix)
        let local_name = qname.local_name();

        match local_name {
            // String types
            "string" | "normalizedString" | "token" | "language" | "Name" | "NCName"
            | "NMTOKEN" | "NMTOKENS" | "ID" | "IDREF" | "IDREFS" | "ENTITY" | "ENTITIES"
            | "anyURI" | "QName" | "NOTATION" => "String".to_string(),

            // Integer types
            "int" | "integer" => "i32".to_string(),
            "long" => "i64".to_string(),
            "short" => "i16".to_string(),
            "byte" => "i8".to_string(),

            // Unsigned integer types
            "unsignedInt" => "u32".to_string(),
            "unsignedLong" => "u64".to_string(),
            "unsignedShort" => "u16".to_string(),
            "unsignedByte" => "u8".to_string(),

            // Arbitrary precision integers
            "positiveInteger" | "nonNegativeInteger" | "nonPositiveInteger" | "negativeInteger" => {
                "i64".to_string()
            }

            // Floating point types
            "float" => "f32".to_string(),
            "double" => "f64".to_string(),
            "decimal" => "f64".to_string(), // Could use rust_decimal crate instead

            // Boolean
            "boolean" => "bool".to_string(),

            // Date/Time types
            "dateTime" => "String".to_string(), // Could use chrono::DateTime
            "time" => "String".to_string(),     // Could use chrono::NaiveTime
            "date" => "String".to_string(),     // Could use chrono::NaiveDate
            "gYearMonth" => "String".to_string(),
            "gYear" => "String".to_string(),
            "gMonthDay" => "String".to_string(),
            "gDay" => "String".to_string(),
            "gMonth" => "String".to_string(),
            "duration" => "String".to_string(), // Could use chrono::Duration

            // Binary types
            "base64Binary" => "Vec<u8>".to_string(),
            "hexBinary" => "Vec<u8>".to_string(),

            // Other types
            "anyType" => "String".to_string(), // Generic fallback
            "anySimpleType" => "String".to_string(),

            // If not a built-in type, assume it's a custom type
            _ => {
                // Convert to PascalCase for custom types
                super::to_pascal_case(local_name)
            }
        }
    }

    /// Check if a type is optional based on minOccurs and nillable
    pub fn is_optional(&self, min_occurs: Option<u32>, nillable: bool) -> bool {
        nillable || min_occurs == Some(0) || min_occurs.is_none()
    }

    /// Check if a type is a collection based on maxOccurs
    pub fn is_collection(&self, max_occurs: &Option<String>) -> bool {
        match max_occurs {
            Some(max) => max == "unbounded" || max.parse::<u32>().unwrap_or(1) > 1,
            None => false,
        }
    }

    /// Wrap a Rust type with Option if it's optional
    pub fn wrap_optional(&self, rust_type: String, is_optional: bool) -> String {
        if is_optional {
            format!("Option<{}>", rust_type)
        } else {
            rust_type
        }
    }

    /// Wrap a Rust type with Vec if it's a collection
    pub fn wrap_collection(&self, rust_type: String, is_collection: bool) -> String {
        if is_collection {
            format!("Vec<{}>", rust_type)
        } else {
            rust_type
        }
    }

    /// Map a type with occurrence constraints
    pub fn map_type_with_occurs(
        &self,
        qname: &QName,
        min_occurs: Option<u32>,
        max_occurs: &Option<String>,
        nillable: bool,
    ) -> String {
        let base_type = self.map_type(qname);
        let is_optional = self.is_optional(min_occurs, nillable);
        let is_collection = self.is_collection(max_occurs);

        // Collection wraps before Option
        // e.g., Option<Vec<String>> not Vec<Option<String>>
        if is_collection {
            let vec_type = format!("Vec<{}>", base_type);
            self.wrap_optional(vec_type, is_optional)
        } else {
            self.wrap_optional(base_type, is_optional)
        }
    }

    /// Get the default value for a Rust type
    pub fn default_value(&self, rust_type: &str) -> Option<String> {
        match rust_type {
            "String" => Some("String::new()".to_string()),
            "bool" => Some("false".to_string()),
            t if t.starts_with("i") || t.starts_with("u") || t.starts_with("f") => {
                Some("0".to_string())
            }
            t if t.starts_with("Vec<") => Some("Vec::new()".to_string()),
            t if t.starts_with("Option<") => Some("None".to_string()),
            _ => None,
        }
    }

    /// Check if this is a built-in XSD type
    pub fn is_builtin_type(&self, qname: &QName) -> bool {
        let local_name = qname.local_name();
        matches!(
            local_name,
            "string"
                | "normalizedString"
                | "token"
                | "language"
                | "Name"
                | "NCName"
                | "NMTOKEN"
                | "NMTOKENS"
                | "ID"
                | "IDREF"
                | "IDREFS"
                | "ENTITY"
                | "ENTITIES"
                | "anyURI"
                | "QName"
                | "NOTATION"
                | "int"
                | "integer"
                | "long"
                | "short"
                | "byte"
                | "unsignedInt"
                | "unsignedLong"
                | "unsignedShort"
                | "unsignedByte"
                | "positiveInteger"
                | "nonNegativeInteger"
                | "nonPositiveInteger"
                | "negativeInteger"
                | "float"
                | "double"
                | "decimal"
                | "boolean"
                | "dateTime"
                | "time"
                | "date"
                | "gYearMonth"
                | "gYear"
                | "gMonthDay"
                | "gDay"
                | "gMonth"
                | "duration"
                | "base64Binary"
                | "hexBinary"
                | "anyType"
                | "anySimpleType"
        )
    }
}

impl Default for TypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_builtin_types() {
        let mapper = TypeMapper::new();

        assert_eq!(mapper.map_type(&QName::new("xs:string")), "String");
        assert_eq!(mapper.map_type(&QName::new("xs:int")), "i32");
        assert_eq!(mapper.map_type(&QName::new("xs:long")), "i64");
        assert_eq!(mapper.map_type(&QName::new("xs:boolean")), "bool");
        assert_eq!(mapper.map_type(&QName::new("xs:double")), "f64");
    }

    #[test]
    fn test_map_custom_type() {
        let mapper = TypeMapper::new();
        assert_eq!(
            mapper.map_type(&QName::new("tns:MyCustomType")),
            "MyCustomType"
        );
    }

    #[test]
    fn test_custom_mapping() {
        let mut mapper = TypeMapper::new();
        mapper.add_mapping("tns:SpecialType", "MySpecialRustType");

        assert_eq!(
            mapper.map_type(&QName::new("tns:SpecialType")),
            "MySpecialRustType"
        );
    }

    #[test]
    fn test_is_optional() {
        let mapper = TypeMapper::new();

        assert!(mapper.is_optional(Some(0), false));
        assert!(mapper.is_optional(Some(1), true));
        assert!(!mapper.is_optional(Some(1), false));
        assert!(mapper.is_optional(None, false));
    }

    #[test]
    fn test_is_collection() {
        let mapper = TypeMapper::new();

        assert!(mapper.is_collection(&Some("unbounded".to_string())));
        assert!(mapper.is_collection(&Some("5".to_string())));
        assert!(!mapper.is_collection(&Some("1".to_string())));
        assert!(!mapper.is_collection(&None));
    }

    #[test]
    fn test_wrap_optional() {
        let mapper = TypeMapper::new();

        assert_eq!(
            mapper.wrap_optional("String".to_string(), true),
            "Option<String>"
        );
        assert_eq!(mapper.wrap_optional("String".to_string(), false), "String");
    }

    #[test]
    fn test_wrap_collection() {
        let mapper = TypeMapper::new();

        assert_eq!(
            mapper.wrap_collection("String".to_string(), true),
            "Vec<String>"
        );
        assert_eq!(
            mapper.wrap_collection("String".to_string(), false),
            "String"
        );
    }

    #[test]
    fn test_map_type_with_occurs() {
        let mapper = TypeMapper::new();

        // Required single value
        assert_eq!(
            mapper.map_type_with_occurs(&QName::new("xs:string"), Some(1), &None, false),
            "String"
        );

        // Optional single value
        assert_eq!(
            mapper.map_type_with_occurs(&QName::new("xs:string"), Some(0), &None, false),
            "Option<String>"
        );

        // Required collection
        assert_eq!(
            mapper.map_type_with_occurs(
                &QName::new("xs:string"),
                Some(1),
                &Some("unbounded".to_string()),
                false
            ),
            "Vec<String>"
        );

        // Optional collection
        assert_eq!(
            mapper.map_type_with_occurs(
                &QName::new("xs:string"),
                Some(0),
                &Some("unbounded".to_string()),
                false
            ),
            "Option<Vec<String>>"
        );
    }

    #[test]
    fn test_is_builtin_type() {
        let mapper = TypeMapper::new();

        assert!(mapper.is_builtin_type(&QName::new("xs:string")));
        assert!(mapper.is_builtin_type(&QName::new("xsd:int")));
        assert!(mapper.is_builtin_type(&QName::new("boolean")));
        assert!(!mapper.is_builtin_type(&QName::new("MyCustomType")));
    }

    #[test]
    fn test_default_value() {
        let mapper = TypeMapper::new();

        assert_eq!(
            mapper.default_value("String"),
            Some("String::new()".to_string())
        );
        assert_eq!(mapper.default_value("bool"), Some("false".to_string()));
        assert_eq!(mapper.default_value("i32"), Some("0".to_string()));
        assert_eq!(
            mapper.default_value("Vec<String>"),
            Some("Vec::new()".to_string())
        );
        assert_eq!(
            mapper.default_value("Option<String>"),
            Some("None".to_string())
        );
    }
}
