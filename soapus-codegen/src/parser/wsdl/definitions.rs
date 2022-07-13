//! Parsing of WSDL definitions element attributes

use quick_xml::events::BytesStart;
use std::error::Error;

use super::parser::WsdlParser;

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Parse attributes from the root <definitions> element
    ///
    /// Extracts:
    /// - targetNamespace
    /// - name attribute
    /// - xmlns:* namespace declarations
    pub(super) fn parse_definitions_attrs(&mut self, e: &BytesStart) -> Result<(), Box<dyn Error>> {
        for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            let key = attr.key.as_ref();
            let val = attr.unescape_value()?.to_string();

            if key == b"targetNamespace" {
                self.target_namespace = Some(val.clone());
            } else if key == b"name" {
                self.model.name = Some(val.clone());
            } else if key.starts_with(b"xmlns:") {
                let prefix = String::from_utf8_lossy(&key[6..]).to_string();
                self.namespaces.insert(prefix, val);
            }
        }
        Ok(())
    }
}
