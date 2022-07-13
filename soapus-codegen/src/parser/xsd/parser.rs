//! XSD schema parser orchestration
//!
//! This module contains the main `SchemaParser` struct and orchestrates
//! the parsing of XML Schema documents. The actual parsing logic for each
//! XSD element type is implemented in separate modules:
//!
//! - `schema_attributes` - Schema element attributes
//! - `schema_content` - Top-level schema elements
//! - `element` - Element definitions
//! - `complex_type` - ComplexType definitions
//! - `sequence` - Sequence and all compositors
//! - `simple_type` - SimpleType definitions and utilities

use crate::parser::xsd::XmlSchema;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::error::Error;

/// Parse an XML Schema string into a structured model
///
/// # Arguments
///
/// * `xml` - The XML Schema document as a string
///
/// # Returns
///
/// An `XmlSchema` containing all parsed schema elements
///
/// # Errors
///
/// Returns an error if the XML is malformed or the schema is invalid
pub fn parse_schema(xml: &str) -> Result<XmlSchema, Box<dyn std::error::Error>> {
    let reader = Reader::from_str(xml);
    SchemaParser::new(reader).parse()
}

/// XSD schema parser state
///
/// This struct maintains the parsing state while traversing the schema document.
/// It is used by the parsing functions in the submodules.
pub struct SchemaParser<B: std::io::BufRead> {
    pub(super) reader: Reader<B>,
    pub(super) namespaces: HashMap<String, String>,
    #[allow(dead_code)]
    pub(super) target_namespace: Option<String>,
    pub(super) model: XmlSchema,
}

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Create a new schema parser
    pub fn new(reader: Reader<B>) -> Self {
        Self {
            reader,
            namespaces: HashMap::new(),
            target_namespace: None,
            model: XmlSchema::default(),
        }
    }

    /// Resolve a namespace prefix to its URI
    #[allow(dead_code)]
    pub fn resolve_prefix(&self, prefix: &str) -> Option<&String> {
        self.namespaces.get(prefix)
    }

    /// Parse the XML Schema document
    ///
    /// This orchestrates parsing of the schema by finding the root <schema>
    /// element and dispatching to the appropriate parsing functions.
    pub fn parse(mut self) -> Result<XmlSchema, Box<dyn Error>> {
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ev) => {
                    if ev.local_name().as_ref() == b"schema" {
                        self.parse_schema_attributes(&ev)?;
                        self.parse_schema_content()?;
                        break;
                    }
                }
                Event::End(ev) if ev.local_name().as_ref() == b"schema" => {
                    break;
                }
                Event::Eof => return Err("Unexpected EOF".into()),
                _ => {}
            }
            buf.clear();
        }
        Ok(self.model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_calculator_schema() {
        let schema = r#"
        <schema xmlns="http://www.w3.org/2001/XMLSchema"
                targetNamespace="http://tempuri.org/"
                elementFormDefault="qualified">
            <element name="Add">
                <complexType>
                    <sequence>
                        <element name="intA" type="int"/>
                        <element name="intB" type="int"/>
                    </sequence>
                </complexType>
            </element>
            <element name="AddResponse">
                <complexType>
                    <sequence>
                        <element name="AddResult" type="int"/>
                    </sequence>
                </complexType>
            </element>
        </schema>
        "#;

        let result = parse_schema(schema);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(
            model.target_namespace,
            Some("http://tempuri.org/".to_string())
        );
        assert_eq!(model.element_form_default, Some("qualified".to_string()));

        // Check that Add complexType was created from inline definition
        assert!(model.complex_types.contains_key("Add"));
        let add_type = &model.complex_types["Add"];
        assert!(add_type.sequence.is_some());
        let sequence = add_type.sequence.as_ref().unwrap();
        assert_eq!(sequence.elements.len(), 2);
        assert_eq!(sequence.elements[0].name, "intA");
        assert_eq!(sequence.elements[1].name, "intB");

        // Check AddResponse
        assert!(model.complex_types.contains_key("AddResponse"));
        let response_type = &model.complex_types["AddResponse"];
        assert!(response_type.sequence.is_some());
        let resp_sequence = response_type.sequence.as_ref().unwrap();
        assert_eq!(resp_sequence.elements.len(), 1);
        assert_eq!(resp_sequence.elements[0].name, "AddResult");
    }

    #[test]
    fn parses_sequence_with_empty_elements() {
        let schema = r#"
        <schema xmlns="http://www.w3.org/2001/XMLSchema">
            <complexType name="TestType">
                <sequence>
                    <element name="field1" type="string"/>
                    <element name="field2" type="int" minOccurs="0"/>
                    <element name="field3" type="boolean" maxOccurs="unbounded"/>
                </sequence>
            </complexType>
        </schema>
        "#;

        let result = parse_schema(schema);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert!(model.complex_types.contains_key("TestType"));

        let test_type = &model.complex_types["TestType"];
        assert!(test_type.sequence.is_some());
        let sequence = test_type.sequence.as_ref().unwrap();
        assert_eq!(sequence.elements.len(), 3);

        // Check minOccurs/maxOccurs
        assert_eq!(sequence.elements[1].min_occurs, 0);
        assert_eq!(
            sequence.elements[2].max_occurs,
            Some("unbounded".to_string())
        );
    }
}
