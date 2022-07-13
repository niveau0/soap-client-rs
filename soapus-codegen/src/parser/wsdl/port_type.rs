//! Parsing of WSDL portType elements

use crate::parser::QName;
use quick_xml::events::{BytesStart, Event};
use std::error::Error;
#[cfg(feature = "tracing")]
use tracing::warn;

use super::parser::WsdlParser;
use super::{Fault, PortType, PortTypeOperation};

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Parse a <portType> element
    ///
    /// PortTypes define abstract operations and their input/output messages.
    /// They describe what a web service can do without specifying the protocol
    /// or data format (that's defined in the binding).
    ///
    /// Example:
    /// ```xml
    /// <portType name="CalculatorSoap">
    ///   <operation name="Add">
    ///     <input message="tns:AddSoapIn"/>
    ///     <output message="tns:AddSoapOut"/>
    ///   </operation>
    /// </portType>
    /// ```
    pub(super) fn parse_port_type(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
        let mut name = None;
        for attr in ev.attributes().with_checks(false) {
            let attr = attr?;
            if attr.key.as_ref() == b"name" {
                name = Some(attr.unescape_value()?.to_string());
            }
        }

        let name = name.ok_or("portType missing name")?;
        let mut operations = Vec::new();
        let mut current_op_name = None;
        let mut current_input = None;
        let mut current_output = None;
        let mut current_documentation = None;

        let mut buf = Vec::new();
        let mut current_faults = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"operation" => {
                    current_op_name = None;
                    current_input = None;
                    current_output = None;
                    current_documentation = None;

                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            current_op_name = Some(attr.unescape_value()?.to_string());
                        }
                    }
                }
                Event::Start(e) if e.local_name().as_ref() == b"documentation" => {
                    // Read the text content of <documentation> element
                    let mut doc_text = String::new();
                    loop {
                        match self.reader.read_event_into(&mut buf)? {
                            Event::Text(e) => {
                                doc_text.push_str(e.unescape()?.trim());
                            }
                            Event::End(e) if e.local_name().as_ref() == b"documentation" => break,
                            Event::Eof => break,
                            _ => {}
                        }
                    }
                    if !doc_text.is_empty() {
                        current_documentation = Some(doc_text);
                    }
                }
                Event::Empty(e) | Event::Start(e) if e.local_name().as_ref() == b"input" => {
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        if attr.key.as_ref() == b"message" {
                            current_input = Some(QName(attr.unescape_value()?.to_string()));
                        }
                    }
                }
                Event::Empty(e) | Event::Start(e) if e.local_name().as_ref() == b"output" => {
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        if attr.key.as_ref() == b"message" {
                            current_output = Some(QName(attr.unescape_value()?.to_string()));
                        }
                    }
                }
                Event::Empty(e) | Event::Start(e) if e.local_name().as_ref() == b"fault" => {
                    let mut fault_name = None;
                    let mut fault_message = None;
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        match attr.key.as_ref() {
                            b"name" => fault_name = Some(attr.unescape_value()?.to_string()),
                            b"message" => {
                                fault_message = Some(QName(attr.unescape_value()?.to_string()))
                            }
                            _ => {}
                        }
                    }
                    while let Ok(Event::End(ref e)) = self.reader.read_event_into(&mut buf) {
                        if e.local_name().as_ref() == b"fault" {
                            break;
                        }
                    }
                    if let (Some(name), Some(message)) = (fault_name, fault_message) {
                        current_faults.push(Fault { name, message });
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"operation" => {
                    if current_input.is_none() && current_output.is_none() {
                        warn!(
                            "Operation '{}' in portType '{}' has no input or output message.",
                            current_op_name.as_ref().unwrap(),
                            name
                        );
                    }

                    let faults = current_faults;
                    current_faults = Vec::new();
                    if let Some(name) = current_op_name.take() {
                        operations.push(PortTypeOperation {
                            name,
                            input: current_input.take(),
                            output: current_output.take(),
                            faults,
                            documentation: current_documentation.take(),
                        });
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"portType" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear()
        }

        self.model.port_types.push(PortType { name, operations });
        Ok(())
    }
}
