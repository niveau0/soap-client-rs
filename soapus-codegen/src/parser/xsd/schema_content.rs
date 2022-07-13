//! Parsing of XSD schema content (top-level elements)

use quick_xml::events::Event;
use std::error::Error;

use super::parser::SchemaParser;
use super::ComplexType;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse the content of the <schema> element
    ///
    /// This processes all top-level schema elements:
    /// - <element> - Top-level element definitions
    /// - <complexType> - Complex type definitions
    /// - <simpleType> - Simple type definitions
    pub(super) fn parse_schema_content(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();

        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) => match e.local_name().as_ref() {
                    b"element" => self.parse_element(&e, true)?,
                    b"complexType" => self.parse_complex_type(&e)?,
                    b"simpleType" => self.parse_simple_type(&e)?,
                    // Additional schema elements (attribute, group, etc.) can be added here if needed
                    _ => {} // Ignore unknown schema elements
                },
                Event::Empty(e) => match e.local_name().as_ref() {
                    b"element" => self.parse_element(&e, false)?,
                    b"complexType" => {
                        // Empty complex type (unusual but handle it)
                        let name = e
                            .try_get_attribute("name")?
                            .map(|a| a.unescape_value().unwrap().into_owned());
                        if let Some(n) = name {
                            self.model.complex_types.insert(
                                n.clone(),
                                ComplexType {
                                    name: n,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                    b"simpleType" => {
                        // Empty simple type - just skip
                    }
                    _ => {} // Ignoriere unbekannte Schema-Elemente
                },
                Event::End(e) if e.local_name().as_ref() == b"schema" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(())
    }
}
