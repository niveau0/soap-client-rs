//! Parsing of WSDL binding elements

use quick_xml::events::{BytesStart, Event};
use std::error::Error;
#[cfg(feature = "tracing")]
use tracing::warn;

use crate::parser::QName;

use super::parser::WsdlParser;
use super::{Binding, BindingOperation};

// Standard SOAP namespace URIs as defined by W3C WSDL specification
const SOAP_11_BINDING_NS: &str = "http://schemas.xmlsoap.org/wsdl/soap/";
const SOAP_12_BINDING_NS: &str = "http://schemas.xmlsoap.org/wsdl/soap12/";

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Parse a <binding> element
    ///
    /// Bindings specify the concrete protocol and data format for a portType.
    /// They define how SOAP operations are transmitted (HTTP, transport details)
    /// and include the SOAPAction header for each operation.
    ///
    /// Example:
    /// ```xml
    /// <binding name="CalculatorSoap" type="tns:CalculatorSoap">
    ///   <soap:binding transport="http://schemas.xmlsoap.org/soap/http"/>
    ///   <operation name="Add">
    ///     <soap:operation soapAction="http://tempuri.org/Add" style="document"/>
    ///     <input>
    ///       <soap:body use="literal"/>
    ///     </input>
    ///     <output>
    ///       <soap:body use="literal"/>
    ///     </output>
    ///   </operation>
    /// </binding>
    /// ```
    pub(super) fn parse_binding(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
        let mut name = None;
        let mut type_ = None;
        let mut transport = None;
        let mut soap_version = None;
        let mut is_soap_binding = false;
        let mut operations = Vec::new();

        for attr in ev.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
                b"name" => name = Some(attr.unescape_value()?.to_string()),
                b"type" => type_ = Some(QName(attr.unescape_value()?.to_string())),
                b"xmlns:soap" => {
                    soap_version = Some(attr.unescape_value()?.to_string());
                    is_soap_binding = true;
                }
                _ => {}
            }
        }

        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                // SOAP Binding Element
                Event::Empty(e) | Event::Start(e)
                    if e.local_name().as_ref().ends_with(b"binding") =>
                {
                    // Check namespace URI to determine SOAP version
                    // The namespace URI is defined by W3C WSDL specification and is standard
                    if let Some(ns_uri) = self.get_namespace_uri(e.name().as_ref()) {
                        if ns_uri == SOAP_11_BINDING_NS {
                            is_soap_binding = true;
                            if soap_version.is_none() {
                                soap_version = Some("1.1".to_string());
                            }
                        } else if ns_uri == SOAP_12_BINDING_NS {
                            is_soap_binding = true;
                            soap_version = Some("1.2".to_string());
                        }
                    }
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        match attr.key.as_ref() {
                            b"transport" => transport = Some(attr.unescape_value()?.to_string()),
                            b"style" => {}
                            b"version" => {
                                // Allow explicit version attribute to override namespace detection
                                if let Some(ns_uri) = self.get_namespace_uri(e.name().as_ref()) {
                                    if ns_uri == SOAP_11_BINDING_NS || ns_uri == SOAP_12_BINDING_NS
                                    {
                                        soap_version = Some(attr.unescape_value()?.to_string());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Operation Start
                Event::Start(e) if e.local_name().as_ref() == b"operation" => {
                    let mut op_name = None;
                    let mut soap_action = None;
                    let mut style = None;

                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            op_name = Some(attr.unescape_value()?.to_string());
                        }
                    }

                    loop {
                        match self.reader.read_event_into(&mut buf)? {
                            // <soap:operation> or <soap12:operation>
                            Event::Empty(e) | Event::Start(e)
                                if e.local_name().as_ref() == b"operation" =>
                            {
                                // Only process SOAP operations, not WSDL operations
                                if let Some(ns_uri) = self.get_namespace_uri(e.name().as_ref()) {
                                    if ns_uri == SOAP_11_BINDING_NS || ns_uri == SOAP_12_BINDING_NS
                                    {
                                        for attr in e.attributes().with_checks(false) {
                                            let attr = attr?;
                                            match attr.key.as_ref() {
                                                b"soapAction" => {
                                                    soap_action =
                                                        Some(attr.unescape_value()?.to_string())
                                                }
                                                b"style" => {
                                                    style = Some(attr.unescape_value()?.to_string())
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                            Event::End(e) if e.local_name().as_ref() == b"operation" => break,
                            _ => {}
                        }
                        buf.clear()
                    }

                    if let Some(name) = op_name {
                        if is_soap_binding && soap_action.is_none() {
                            warn!(
                                "SOAP operation '{}' in binding '{}' missing 'soapAction'",
                                name,
                                self.model
                                    .bindings
                                    .last()
                                    .map(|b| &b.name)
                                    .unwrap_or(&String::new())
                            );
                        }
                        // 'style'-attribute might be in <binding> as in <operation>.
                        // Note: SOAP fault parsing is handled at runtime, not in WSDL binding
                        operations.push(BindingOperation {
                            name,
                            soap_action,
                            style,
                        });
                    }
                }

                Event::End(e) if e.local_name().as_ref() == b"binding" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear()
        }

        let name = name.ok_or("binding missing name")?;
        let type_ = type_.ok_or("binding missing type")?;

        if is_soap_binding {
            let Some(transport) = transport else {
                return Err(
                    format!("SOAP binding '{}' missing 'transport' attribute.", name).into(),
                );
            };
            let soap_version = soap_version.unwrap_or_else(|| {
                warn!(
                    "SOAP binding '{}' missing SOAP version information. Assuming default (1.1)",
                    name
                );
                "1.1".to_string()
            });

            self.model.bindings.push(Binding {
                name,
                type_,
                transport,
                soap_version,
                operations,
            });
        }

        Ok(())
    }
}
