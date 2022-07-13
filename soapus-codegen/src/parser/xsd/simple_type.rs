//! Parsing of XSD simpleType definitions and utility functions

use quick_xml::events::{BytesStart, Event};
use std::error::Error;

use super::parser::SchemaParser;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse a <simpleType> definition
    ///
    /// SimpleTypes define restrictions on built-in types, such as enumerations,
    /// patterns, or value ranges. Currently, we primarily support enumerations
    /// which are used to generate Rust enums.
    ///
    /// Other simpleType restrictions (pattern, minLength, etc.) are handled
    /// by the type mapper which maps them to appropriate Rust types.
    ///
    /// Example:
    /// ```xml
    /// <simpleType name="ColorType">
    ///   <restriction base="xs:string">
    ///     <enumeration value="Red"/>
    ///     <enumeration value="Green"/>
    ///     <enumeration value="Blue"/>
    ///   </restriction>
    /// </simpleType>
    /// ```
    pub(super) fn parse_simple_type(&mut self, _e: &BytesStart) -> Result<(), Box<dyn Error>> {
        // SimpleType parsing is implemented in parse_restriction for enumerations
        // Other simple types are handled by the type mapper
        self.skip_element()?;
        Ok(())
    }

    /// Skip the current element and all its content
    ///
    /// This is used when we encounter elements we don't need to parse in detail,
    /// such as annotations, documentation, or unsupported simpleType restrictions.
    ///
    /// The function maintains a depth counter to correctly handle nested elements
    /// with the same name.
    pub(super) fn skip_element(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();
        let mut depth = 1;
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(_) => depth += 1,
                Event::End(_) => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(())
    }
}
