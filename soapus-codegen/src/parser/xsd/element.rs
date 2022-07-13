//! Parsing of XSD element definitions

use crate::parser::xsd::{ComplexType, SchemaElement, Sequence};
use crate::parser::QName;
use quick_xml::events::{BytesStart, Event};
use std::error::Error;

use super::parser::SchemaParser;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse an <element> definition
    ///
    /// Elements can either reference an existing type or define an inline complexType.
    /// This function handles both cases:
    /// - External type reference: `<element name="Foo" type="xs:string"/>`
    /// - Inline complexType: `<element name="Foo"><complexType>...</complexType></element>`
    ///
    /// # Arguments
    ///
    /// * `e` - The element's start tag
    /// * `should_skip` - If true, this is a Start event and we need to parse content.
    ///   If false, this is an Empty event and there's no content.
    pub(super) fn parse_element(
        &mut self,
        e: &BytesStart,
        should_skip: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut element_name = None;
        let mut type_name = None;
        let mut nillable = false;

        for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            let key = attr.key.as_ref();
            let val = attr.unescape_value()?;
            if key == b"name" {
                element_name = Some(val.to_string());
            } else if key == b"type" {
                type_name = Some(QName(val.to_string()));
            } else if key == b"nillable" && val == "true" {
                nillable = true;
            }
        }

        // If this is a Start event (not Empty), check for inline complexType
        if should_skip {
            if let Some(ref name) = element_name {
                // Look for inline complexType definition
                let mut buf = Vec::new();
                let mut found_inline_complex_type = false;

                loop {
                    match self.reader.read_event_into(&mut buf)? {
                        Event::Start(e) if e.local_name().as_ref() == b"complexType" => {
                            // Parse inline complexType and add it to complex_types with element's name
                            let mut complex_type = ComplexType {
                                name: name.clone(),
                                ..Default::default()
                            };

                            let mut inner_buf = Vec::new();
                            loop {
                                match self.reader.read_event_into(&mut inner_buf)? {
                                    Event::Start(e) if e.local_name().as_ref() == b"sequence" => {
                                        complex_type.sequence = Some(self.parse_sequence()?);
                                    }
                                    Event::Empty(e) if e.local_name().as_ref() == b"sequence" => {
                                        complex_type.sequence = Some(Sequence::default());
                                    }
                                    Event::Start(e) if e.local_name().as_ref() == b"all" => {
                                        complex_type.sequence = Some(self.parse_all()?);
                                    }
                                    Event::Empty(e) if e.local_name().as_ref() == b"all" => {
                                        complex_type.sequence = Some(Sequence::default());
                                    }
                                    Event::End(e) if e.local_name().as_ref() == b"complexType" => {
                                        break
                                    }
                                    Event::Eof => break,
                                    _ => {}
                                }
                                inner_buf.clear();
                            }

                            self.model.complex_types.insert(name.clone(), complex_type);
                            found_inline_complex_type = true;
                        }
                        Event::End(e) if e.local_name().as_ref() == b"element" => break,
                        Event::Eof => break,
                        _ => {}
                    }
                    buf.clear();
                }

                // Only add to elements if it has an external type reference (not inline complexType)
                if !found_inline_complex_type && type_name.is_some() {
                    self.model.elements.insert(
                        name.clone(),
                        SchemaElement {
                            name: name.clone(),
                            type_: type_name.unwrap_or_default(),
                            nillable,
                            min_occurs: None,
                            max_occurs: None,
                        },
                    );
                }
            }
        } else {
            // Empty element - just add to elements if it has a name
            if let Some(name) = element_name {
                self.model.elements.insert(
                    name.clone(),
                    SchemaElement {
                        name,
                        type_: type_name.unwrap_or_default(),
                        nillable,
                        min_occurs: None,
                        max_occurs: None,
                    },
                );
            }
        }
        Ok(())
    }
}
