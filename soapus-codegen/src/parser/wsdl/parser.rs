//! WSDL parser orchestration
//!
//! This module contains the main `WsdlParser` struct and orchestrates
//! the parsing of WSDL documents. The actual parsing logic for each
//! WSDL element type is implemented in separate modules:
//!
//! - `definitions` - Root element attributes and namespaces
//! - `types` - XSD schema extraction
//! - `message` - Message definitions
//! - `port_type` - PortType and operation definitions
//! - `binding` - SOAP binding and operation details
//! - `service` - Service endpoints and ports

use super::WsdlModel;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::error::Error;

#[cfg(feature = "tracing")]
use tracing::{debug, info};

/// Parse a WSDL XML string into a structured model
///
/// # Arguments
///
/// * `xml` - The WSDL document as a string
///
/// # Returns
///
/// A `WsdlModel` containing all parsed WSDL elements
///
/// # Errors
///
/// Returns an error if the XML is malformed or the WSDL is invalid
pub fn parse_wsdl(xml: &str) -> Result<WsdlModel, Box<dyn Error>> {
    #[cfg(feature = "tracing")]
    info!(xml_size = xml.len(), "Starting WSDL parsing");

    let reader = Reader::from_str(xml);
    let result = WsdlParser::new(reader).parse();

    #[cfg(feature = "tracing")]
    match &result {
        Ok(model) => info!(
            service_count = model.services.len(),
            message_count = model.messages.len(),
            port_type_count = model.port_types.len(),
            "WSDL parsing completed successfully"
        ),
        Err(e) => tracing::error!(error = %e, "WSDL parsing failed"),
    }

    result
}

/// WSDL parser state
///
/// This struct maintains the parsing state while traversing the WSDL document.
/// It is used by the parsing functions in the submodules.
pub struct WsdlParser<B: std::io::BufRead> {
    pub(super) reader: Reader<B>,
    pub(super) namespaces: HashMap<String, String>,
    pub(super) target_namespace: Option<String>,
    pub(super) model: WsdlModel,
}

impl<B: std::io::BufRead> WsdlParser<B> {
    /// Create a new WSDL parser
    pub fn new(reader: Reader<B>) -> Self {
        Self {
            reader,
            namespaces: HashMap::new(),
            target_namespace: None,
            model: WsdlModel::default(),
        }
    }

    /// Resolve a namespace prefix to its URI
    #[allow(dead_code)]
    pub fn resolve_prefix(&self, prefix: &str) -> Option<&String> {
        self.namespaces.get(prefix)
    }

    /// Get namespace URI from an element name
    ///
    /// For example, "soap:binding" returns the namespace URI for the "soap" prefix
    pub(super) fn get_namespace_uri(&self, element_name: &[u8]) -> Option<&String> {
        // Find the prefix (everything before ':')
        if let Some(colon_pos) = element_name.iter().position(|&b| b == b':') {
            let prefix = String::from_utf8_lossy(&element_name[..colon_pos]);
            self.namespaces.get(prefix.as_ref())
        } else {
            None
        }
    }

    /// Parse the WSDL document
    ///
    /// This orchestrates parsing of all WSDL elements by dispatching
    /// to the appropriate parsing functions in submodules.
    pub fn parse(mut self) -> Result<WsdlModel, Box<dyn Error>> {
        #[cfg(feature = "tracing")]
        debug!("Starting WSDL document traversal");

        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ev) => match ev.local_name().as_ref() {
                    b"definitions" => {
                        #[cfg(feature = "tracing")]
                        debug!("Parsing definitions element");
                        self.parse_definitions_attrs(&ev)?
                    }
                    b"types" => {
                        #[cfg(feature = "tracing")]
                        debug!("Parsing types element");
                        self.parse_types()?
                    }
                    b"message" => {
                        #[cfg(feature = "tracing")]
                        debug!("Parsing message element");
                        self.parse_message(&ev)?
                    }
                    b"portType" => {
                        #[cfg(feature = "tracing")]
                        debug!("Parsing portType element");
                        self.parse_port_type(&ev)?
                    }
                    b"binding" => {
                        #[cfg(feature = "tracing")]
                        debug!("Parsing binding element");
                        self.parse_binding(&ev)?
                    }
                    b"service" => {
                        #[cfg(feature = "tracing")]
                        debug!("Parsing service element");
                        self.parse_service(&ev)?
                    }
                    _ => {}
                },
                Event::Eof => break,
                _ => {}
            }
            buf.clear()
        }

        self.model.target_namespace = self.target_namespace;
        self.model.namespaces = self.namespaces;
        Ok(self.model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_wsdl() {
        let wsdl = r#"<?xml version="1.0" encoding="utf-8"?>
<definitions xmlns="http://schemas.xmlsoap.org/wsdl/"
             xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap/"
             xmlns:tns="http://tempuri.org/"
             targetNamespace="http://tempuri.org/"
             name="Calculator">
  <types>
    <schema xmlns="http://www.w3.org/2001/XMLSchema"
            targetNamespace="http://tempuri.org/">
      <element name="Add">
        <complexType>
          <sequence>
            <element name="intA" type="int"/>
            <element name="intB" type="int"/>
          </sequence>
        </complexType>
      </element>
    </schema>
  </types>

  <message name="AddSoapIn">
    <part name="parameters" element="tns:Add"/>
  </message>

  <message name="AddSoapOut">
    <part name="parameters" element="tns:AddResponse"/>
  </message>

  <portType name="CalculatorSoap">
    <operation name="Add">
      <input message="tns:AddSoapIn"/>
      <output message="tns:AddSoapOut"/>
    </operation>
  </portType>

  <binding name="CalculatorSoap" type="tns:CalculatorSoap">
    <soap:binding transport="http://schemas.xmlsoap.org/soap/http"/>
    <operation name="Add">
      <soap:operation soapAction="http://tempuri.org/Add" style="document"/>
      <input>
        <soap:body use="literal"/>
      </input>
      <output>
        <soap:body use="literal"/>
      </output>
    </operation>
  </binding>

  <service name="Calculator">
    <port name="CalculatorSoap" binding="tns:CalculatorSoap">
      <soap:address location="http://www.dneonline.com/calculator.asmx"/>
    </port>
  </service>
</definitions>"#;

        let result = parse_wsdl(wsdl);
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(model.name, Some("Calculator".to_string()));
        assert_eq!(
            model.target_namespace,
            Some("http://tempuri.org/".to_string())
        );
        assert_eq!(model.messages.len(), 2);
        assert_eq!(model.port_types.len(), 1);
        assert_eq!(model.bindings.len(), 1);
        assert_eq!(model.services.len(), 1);
    }

    #[test]
    fn parses_calculator_wsdl() {
        let wsdl = include_str!("../../../../testdata/wsdl/calculator.wsdl");
        let model = parse_wsdl(wsdl).unwrap();

        // Check service
        assert_eq!(model.service_name(), Some("Calculator"));
        assert_eq!(model.services.len(), 1);
        let service = &model.services[0];
        assert_eq!(service.name, "Calculator");
        assert_eq!(service.ports.len(), 2); // SOAP 1.1 and 1.2

        // Check first port (SOAP 1.1)
        let port1 = &service.ports[0];
        assert_eq!(port1.name, "CalculatorSoap");
        assert_eq!(port1.address, "http://www.dneonline.com/calculator.asmx");

        // Check bindings
        assert_eq!(model.bindings.len(), 2); // SOAP 1.1 and 1.2 bindings
        let binding = &model.bindings[0];
        assert_eq!(binding.name, "CalculatorSoap");
        assert_eq!(binding.soap_version, "1.1");
        assert_eq!(binding.transport, "http://schemas.xmlsoap.org/soap/http");

        // Check operations in binding
        assert_eq!(binding.operations.len(), 4); // Add, Subtract, Multiply, Divide
        let add_op = binding
            .operations
            .iter()
            .find(|op| op.name == "Add")
            .unwrap();
        assert_eq!(
            add_op.soap_action,
            Some("http://tempuri.org/Add".to_string())
        );
        assert_eq!(add_op.style, Some("document".to_string()));

        // Check port types
        assert_eq!(model.port_types.len(), 1);
        let port_type = &model.port_types[0];
        assert_eq!(port_type.name, "CalculatorSoap");
        assert_eq!(port_type.operations.len(), 4);

        // Check messages
        assert!(model.messages.len() >= 8); // 4 operations * 2 messages (in/out)
        let add_in = model.find_message(&"AddSoapIn".into());
        assert!(add_in.is_some());
        assert_eq!(add_in.unwrap().parts.len(), 1);

        // Check schema
        assert!(model.schema().is_some());
        let schema = model.schema().unwrap();
        assert_eq!(
            schema.target_namespace,
            Some("http://tempuri.org/".to_string())
        );
        // Schema should have either elements or complex types
        assert!(!schema.elements.is_empty() || !schema.complex_types.is_empty());
    }

    #[test]
    fn parses_countryinfo_wsdl() {
        let wsdl = include_str!("../../../../testdata/wsdl/countryinfo.wsdl");
        let model = parse_wsdl(wsdl).unwrap();

        // Check service
        assert_eq!(model.service_name(), Some("CountryInfoService"));
        assert_eq!(model.services.len(), 1);
        let service = &model.services[0];
        assert_eq!(service.name, "CountryInfoService");

        // CountryInfo has both SOAP 1.1 and 1.2 ports
        assert!(!service.ports.is_empty());

        // Check bindings
        assert!(!model.bindings.is_empty());

        // Check port types
        assert!(!model.port_types.is_empty());

        // CountryInfo has many operations
        let port_type = &model.port_types[0];
        assert!(port_type.operations.len() > 5);

        // Check for specific operations
        let has_list_of_countries = port_type
            .operations
            .iter()
            .any(|op| op.name == "ListOfCountryNamesByName");
        assert!(has_list_of_countries);

        // Check schema
        assert!(model.schema().is_some());
        let schema = model.schema().unwrap();
        assert!(schema.target_namespace.is_some());
    }

    #[test]
    fn parses_numberconversion_wsdl() {
        let wsdl = include_str!("../../../../testdata/wsdl/numberconversion.wsdl");
        let model = parse_wsdl(wsdl).unwrap();

        // Check service
        assert_eq!(model.service_name(), Some("NumberConversion"));
        assert_eq!(model.services.len(), 1);
        let service = &model.services[0];
        assert_eq!(service.name, "NumberConversion");

        // NumberConversion has both SOAP 1.1 and 1.2 ports
        assert!(!service.ports.is_empty());

        // Check bindings
        assert!(!model.bindings.is_empty());

        // Check port types
        assert_eq!(model.port_types.len(), 1);
        let port_type = &model.port_types[0];
        assert_eq!(port_type.name, "NumberConversionSoapType");

        // Check operations
        assert!(port_type.operations.len() >= 2);
        let has_number_to_words = port_type
            .operations
            .iter()
            .any(|op| op.name == "NumberToWords");
        let has_number_to_dollars = port_type
            .operations
            .iter()
            .any(|op| op.name == "NumberToDollars");
        assert!(has_number_to_words);
        assert!(has_number_to_dollars);

        // Check schema
        assert!(model.schema().is_some());
    }
}
