//! Parsing of WSDL message elements

use crate::parser::QName;
use quick_xml::events::{BytesStart, Event};
use std::error::Error;

use super::parser::WsdlParser;
use super::{Message, MessagePart};

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Parse a <message> element
    ///
    /// Messages define the data being communicated in SOAP operations.
    /// Each message consists of one or more parts, which reference
    /// either an element or a type from the schema.
    ///
    /// Example:
    /// ```xml
    /// <message name="AddSoapIn">
    ///   <part name="parameters" element="tns:Add"/>
    /// </message>
    /// ```
    pub(super) fn parse_message(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
        let mut name = None;
        for attr in ev.attributes().with_checks(false) {
            let attr = attr?;
            if attr.key.as_ref() == b"name" {
                name = Some(attr.unescape_value()?.to_string());
            }
        }

        let name = name.ok_or("message missing name")?;
        let mut parts = Vec::new();
        let mut buf = Vec::new();

        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) | Event::Empty(e) if e.local_name().as_ref() == b"part" => {
                    let mut part_name = None;
                    let mut part_element = None;
                    let mut part_type = None;

                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        let key = attr.key.as_ref();
                        let val = attr.unescape_value()?;
                        match key {
                            b"name" => part_name = Some(val),
                            b"element" => part_element = Some(QName(val.to_string())),
                            b"type" => part_type = Some(QName(val.to_string())),
                            _ => {}
                        }
                    }

                    if let Some(part_name) = part_name {
                        if part_element.is_none() && part_type.is_none() {
                            return Err(format!(
                                "Part '{}' in message '{}' must have either 'element' or 'type' attribute.",
                                part_name, name
                            ).into());
                        }
                        parts.push(MessagePart {
                            name: part_name.to_string(),
                            element: part_element,
                            type_: part_type,
                        });
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"message" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear()
        }

        self.model.messages.push(Message { name, parts });
        Ok(())
    }
}
