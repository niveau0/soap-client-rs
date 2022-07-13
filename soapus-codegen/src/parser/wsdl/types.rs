//! Parsing of WSDL types element and embedded XSD schema

use crate::parser::parse_schema;
use quick_xml::events::Event;
use std::error::Error;

use super::parser::WsdlParser;

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Parse the <types> section and extract embedded XSD schema
    ///
    /// The types section contains XML Schema definitions that define
    /// the data types used in SOAP messages. This function extracts
    /// the embedded schema and parses it using the XSD parser.
    pub(super) fn parse_types(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();
        let mut schema_xml = String::new();
        let mut in_schema = false;
        let mut _depth = 0;

        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"schema" => {
                    in_schema = true;
                    _depth = 1;
                    schema_xml.push_str("<schema");
                    // Add attributes
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        let key = std::str::from_utf8(attr.key.as_ref())?;
                        let value = attr.unescape_value()?;
                        schema_xml.push_str(&format!(" {}=\"{}\"", key, value));
                    }
                    schema_xml.push('>');
                }
                Event::Start(e) if in_schema => {
                    _depth += 1;
                    schema_xml.push('<');
                    schema_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        let key = std::str::from_utf8(attr.key.as_ref())?;
                        let value = attr.unescape_value()?;
                        schema_xml.push_str(&format!(" {}=\"{}\"", key, value));
                    }
                    schema_xml.push('>');
                }
                Event::End(e) if in_schema && e.local_name().as_ref() == b"schema" => {
                    schema_xml.push_str("</schema>");

                    // Parse the extracted schema
                    if let Ok(schema) = parse_schema(&schema_xml) {
                        self.model.set_schema(schema);
                    }

                    break;
                }
                Event::End(e) if in_schema => {
                    _depth -= 1;
                    schema_xml.push_str("</");
                    schema_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                    schema_xml.push('>');
                }
                Event::Text(e) if in_schema => {
                    let text = e.unescape()?;
                    schema_xml.push_str(&text);
                }
                Event::Empty(e) if in_schema => {
                    schema_xml.push('<');
                    schema_xml.push_str(std::str::from_utf8(e.name().as_ref())?);
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        let key = std::str::from_utf8(attr.key.as_ref())?;
                        let value = attr.unescape_value()?;
                        schema_xml.push_str(&format!(" {}=\"{}\"", key, value));
                    }
                    schema_xml.push_str("/>");
                }
                Event::End(e) if e.local_name().as_ref() == b"types" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(())
    }
}
