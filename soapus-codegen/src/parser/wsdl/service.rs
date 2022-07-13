//! Parsing of WSDL service elements

use crate::parser::QName;
use quick_xml::events::{BytesStart, Event};
use std::error::Error;
#[cfg(feature = "tracing")]
use tracing::warn;

use super::parser::WsdlParser;
use super::{Port, Service};

// Standard SOAP namespace URIs as defined by W3C WSDL specification
const SOAP_11_BINDING_NS: &str = "http://schemas.xmlsoap.org/wsdl/soap/";
const SOAP_12_BINDING_NS: &str = "http://schemas.xmlsoap.org/wsdl/soap12/";

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Parse a <service> element
    ///
    /// Services define the endpoints (URLs) where the web service can be accessed.
    /// Each service contains one or more ports, each specifying a binding and
    /// the actual network address.
    ///
    /// Example:
    /// ```xml
    /// <service name="Calculator">
    ///   <port name="CalculatorSoap" binding="tns:CalculatorSoap">
    ///     <soap:address location="http://www.dneonline.com/calculator.asmx"/>
    ///   </port>
    /// </service>
    /// ```
    pub(super) fn parse_service(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
        let mut service_name = None;
        let mut ports = Vec::new();

        for attr in ev.attributes().with_checks(false) {
            let attr = attr?;
            if attr.key.as_ref() == b"name" {
                service_name = Some(attr.unescape_value()?.to_string());
            }
        }

        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"port" => {
                    let mut port_name = None;
                    let mut binding = None;
                    let mut address = None;

                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        match attr.key.as_ref() {
                            b"name" => port_name = Some(attr.unescape_value()?.to_string()),
                            b"binding" => binding = Some(QName(attr.unescape_value()?.to_string())),
                            _ => {}
                        }
                    }

                    let mut port_buf = Vec::new();
                    loop {
                        match self.reader.read_event_into(&mut port_buf)? {
                            Event::Empty(e) | Event::Start(e)
                                if e.local_name().as_ref() == b"address" =>
                            {
                                // Only process SOAP address elements, not other address types
                                if let Some(ns_uri) = self.get_namespace_uri(e.name().as_ref()) {
                                    if ns_uri == SOAP_11_BINDING_NS || ns_uri == SOAP_12_BINDING_NS
                                    {
                                        for attr in e.attributes().with_checks(false) {
                                            let attr = attr?;
                                            if attr.key.as_ref() == b"location" {
                                                address = Some(attr.unescape_value()?.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            Event::End(e) if e.local_name().as_ref() == b"port" => break,
                            _ => {}
                        }
                        port_buf.clear();
                    }

                    let port_name = port_name.ok_or("port missing name")?;
                    let Some(binding) = binding else {
                        return Err(format!("Port '{}' missing 'binding' attribute.", port_name))?;
                    };
                    let Some(address) = address else {
                        warn!(
                            "Port '{}' missing address information (e.g., <soap:address location>).",
                            port_name
                        );
                        continue;
                    };

                    ports.push(Port {
                        name: port_name,
                        binding,
                        address,
                    });
                }
                Event::End(e) if e.local_name().as_ref() == b"service" => break,
                Event::Eof => break,
                _ => {}
            }

            buf.clear();
        }

        self.model.services.push(Service {
            name: service_name.ok_or("service missing name")?,
            ports,
        });

        Ok(())
    }
}
