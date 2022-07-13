//! Parsing of XSD sequence and all compositors

use crate::parser::xsd::{Sequence, SequenceElement};
use crate::parser::QName;
use quick_xml::events::{BytesStart, Event};
use std::error::Error;

use super::parser::SchemaParser;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse a <sequence> compositor
    ///
    /// Sequences define an ordered list of child elements.
    /// Each element can have minOccurs/maxOccurs attributes to control cardinality.
    ///
    /// Example:
    /// ```xml
    /// <sequence>
    ///   <element name="firstName" type="xs:string"/>
    ///   <element name="lastName" type="xs:string"/>
    ///   <element name="age" type="xs:int" minOccurs="0"/>
    /// </sequence>
    /// ```
    pub(super) fn parse_sequence(&mut self) -> Result<Sequence, Box<dyn Error>> {
        let mut sequence = Sequence::default();
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"element" => {
                    let elem = self.parse_sequence_element(&e, true)?;
                    sequence.elements.push(elem);
                }
                Event::Empty(e) if e.local_name().as_ref() == b"element" => {
                    // Empty element like <xs:element name="..." type="..."/>
                    let elem = self.parse_sequence_element(&e, false)?;
                    sequence.elements.push(elem);
                }
                Event::End(e) if e.local_name().as_ref() == b"sequence" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(sequence)
    }

    /// Parse an <all> compositor
    ///
    /// The <all> compositor is similar to sequence but elements can appear in any order.
    /// For code generation purposes, we treat it the same as sequence.
    ///
    /// Example:
    /// ```xml
    /// <all>
    ///   <element name="country" type="xs:string"/>
    ///   <element name="zipCode" type="xs:string"/>
    /// </all>
    /// ```
    pub(super) fn parse_all(&mut self) -> Result<Sequence, Box<dyn Error>> {
        // Parse <xs:all> compositor - treat as sequence for now
        // In the future, we might want to distinguish this for validation purposes
        let mut sequence = Sequence::default();
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"element" => {
                    let elem = self.parse_sequence_element(&e, true)?;
                    sequence.elements.push(elem);
                }
                Event::Empty(e) if e.local_name().as_ref() == b"element" => {
                    // Empty element like <xs:element name="..." type="..."/>
                    let elem = self.parse_sequence_element(&e, false)?;
                    sequence.elements.push(elem);
                }
                Event::End(e) if e.local_name().as_ref() == b"all" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(sequence)
    }

    /// Parse an element within a sequence or all compositor
    ///
    /// Extracts:
    /// - name - Element name
    /// - type - Element type (QName)
    /// - minOccurs - Minimum occurrences (default: 1)
    /// - maxOccurs - Maximum occurrences (default: 1, or "unbounded")
    /// - nillable - Whether the element can be nil/null
    ///
    /// # Arguments
    ///
    /// * `e` - The element's start tag
    /// * `should_skip` - If true, skip to the end of the element
    pub(super) fn parse_sequence_element(
        &mut self,
        e: &BytesStart,
        should_skip: bool,
    ) -> Result<SequenceElement, Box<dyn Error>> {
        let mut name = None;
        let mut type_name = None;
        let mut min_occurs = 1u32;
        let mut max_occurs = None;
        let mut nillable = false;

        for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            let key = attr.key.as_ref();
            let val = attr.unescape_value()?;

            match key {
                b"name" => name = Some(val.to_string()),
                b"type" => type_name = Some(QName(val.to_string())),
                b"minOccurs" => min_occurs = val.parse().unwrap_or(1),
                b"maxOccurs" => {
                    max_occurs = if val == "unbounded" {
                        Some("unbounded".to_string())
                    } else {
                        Some(val.to_string())
                    }
                }
                b"nillable" => nillable = val == "true",
                _ => {}
            }
        }

        // If this is a Start event, skip to the end of the element
        if should_skip {
            self.skip_element()?;
        }

        Ok(SequenceElement {
            name: name.unwrap_or_default(),
            type_: type_name.unwrap_or_default(),
            min_occurs,
            max_occurs,
            nillable,
        })
    }
}
