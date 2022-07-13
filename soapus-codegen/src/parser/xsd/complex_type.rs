//! Parsing of XSD complexType definitions

use crate::parser::xsd::{ComplexType, Sequence};
use quick_xml::events::{BytesStart, Event};
use std::error::Error;

use super::parser::SchemaParser;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse a <complexType> definition
    ///
    /// ComplexTypes define structured types with child elements.
    /// They can contain:
    /// - <sequence> - Ordered sequence of elements
    /// - <all> - Unordered collection of elements
    /// - <choice> - Alternative elements (not yet fully supported)
    ///
    /// Example:
    /// ```xml
    /// <complexType name="Person">
    ///   <sequence>
    ///     <element name="firstName" type="xs:string"/>
    ///     <element name="lastName" type="xs:string"/>
    ///   </sequence>
    /// </complexType>
    /// ```
    pub(super) fn parse_complex_type(&mut self, e: &BytesStart) -> Result<(), Box<dyn Error>> {
        let name = e
            .try_get_attribute("name")?
            .map(|a| a.unescape_value().unwrap().into_owned());
        let mut complex_type = ComplexType::default();
        if let Some(n) = name {
            complex_type.name = n;
        }

        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"sequence" => {
                    complex_type.sequence = Some(self.parse_sequence()?);
                }
                Event::Empty(e) if e.local_name().as_ref() == b"sequence" => {
                    // Empty sequence like <xs:sequence/>
                    complex_type.sequence = Some(Sequence::default());
                }
                Event::Start(e) if e.local_name().as_ref() == b"all" => {
                    // <xs:all> - treat as sequence for now
                    complex_type.sequence = Some(self.parse_all()?);
                }
                Event::Empty(e) if e.local_name().as_ref() == b"all" => {
                    // Empty all like <xs:all/>
                    complex_type.sequence = Some(Sequence::default());
                }
                Event::End(e) if e.local_name().as_ref() == b"complexType" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        if !complex_type.name.is_empty() {
            self.model
                .complex_types
                .insert(complex_type.name.clone(), complex_type);
        }
        Ok(())
    }
}
