//! Parsing of XSD schema element attributes

use quick_xml::events::BytesStart;
use std::error::Error;

use super::parser::SchemaParser;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse attributes from the root <schema> element
    ///
    /// Extracts:
    /// - targetNamespace
    /// - attributeFormDefault
    /// - elementFormDefault
    /// - version
    /// - xmlns:* namespace declarations
    pub(super) fn parse_schema_attributes(&mut self, e: &BytesStart) -> Result<(), Box<dyn Error>> {
        for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            let key = attr.key.as_ref();
            let val = attr.unescape_value()?.to_string();

            if key == b"targetNamespace" {
                self.model.target_namespace = Some(val.clone());
            } else if key == b"attributeFormDefault" {
                self.model.attribute_form_default = Some(val.clone());
            } else if key == b"elementFormDefault" {
                self.model.element_form_default = Some(val.clone());
            } else if key == b"version" {
                self.model.version = Some(val.clone());
            } else if key.starts_with(b"xmlns:") {
                let prefix = String::from_utf8_lossy(&key[6..]).to_string();
                self.namespaces.insert(prefix.clone(), val.clone());
                self.model.namespaces.insert(prefix, val);
            }
        }
        Ok(())
    }
}
