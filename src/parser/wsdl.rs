use super::{
    Binding, BindingOperation, Fault, Message, MessagePart, Port, PortType, PortTypeOperation,
    QName, Service, WsdlModel,
};
use log::warn;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::HashMap;
use std::error::Error;

pub fn parse_wsdl(xml: &str) -> Result<WsdlModel, Box<dyn std::error::Error>> {
    let reader = Reader::from_str(xml);
    WsdlParser::new(reader).parse()
}

pub struct WsdlParser<B: std::io::BufRead> {
    reader: Reader<B>,
    namespaces: HashMap<String, String>,
    target_namespace: Option<String>,
    model: WsdlModel,
}

fn resolve_qname(qname: &str, namespaces: &HashMap<String, String>) -> Option<(String, String)> {
    if let Some((prefix, local)) = qname.split_once(':') {
        if let Some(ns_uri) = namespaces.get(prefix) {
            return Some((ns_uri.clone(), local.to_string()));
        }
    }
    None
}
impl<B: std::io::BufRead> WsdlParser<B> {
    pub fn new(mut reader: Reader<B>) -> Self {
        reader.trim_text(true);
        Self {
            reader,
            namespaces: HashMap::new(),
            target_namespace: None,
            model: WsdlModel::default(),
        }
    }

    pub fn resolve_prefix(&self, prefix: &str) -> Option<&String> {
        self.namespaces.get(prefix)
    }

    pub fn parse(mut self) -> Result<WsdlModel, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(ev) => match ev.local_name().as_ref() {
                    b"definitions" => self.parse_definitions_attrs(&ev)?,
                    b"message" => self.parse_message(&ev)?,
                    b"portType" => self.parse_port_type(&ev)?,
                    b"binding" => self.parse_binding(&ev)?,
                    b"service" => self.parse_service(&ev)?,

                    _ => {
                        dbg!(ev.name());
                    }
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

    fn parse_definitions_attrs(&mut self, e: &BytesStart) -> Result<(), Box<dyn Error>> {
        for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            let key = attr.key.as_ref();
            let val = attr.unescape_value()?.to_string();
            if key == b"targetNamespace" {
                self.target_namespace = Some(val.clone());
            } else if key == b"name" {
                self.model.name = Some(val.clone());
            } else if key.starts_with(b"xmlns:") {
                let prefix = String::from_utf8_lossy(&key[6..]).to_string();
                self.namespaces.insert(prefix, val);
            }
        }
        Ok(())
    }

    fn parse_message(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
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
                            return Err(format!("Part '{}' in message '{}' must have either 'element' or 'type' attribute.", part_name, name).into());
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

    fn parse_port_type(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
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

        let mut buf = Vec::new();
        let mut current_faults = Vec::new();
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"operation" => {
                    current_op_name = None;
                    current_input = None;
                    current_output = None;

                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            current_op_name = Some(attr.unescape_value()?.to_string());
                        }
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
                            faults: faults,
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

    fn parse_binding(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
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
                    if String::from_utf8_lossy(e.name().as_ref()).starts_with("soap:") {
                        is_soap_binding = true;
                    }
                    for attr in e.attributes().with_checks(false) {
                        let attr = attr?;
                        match attr.key.as_ref() {
                            b"transport" => transport = Some(attr.unescape_value()?.to_string()),
                            b"style" => {}
                            b"version"
                                if String::from_utf8_lossy(e.name().as_ref())
                                    .starts_with("soap:") =>
                            {
                                soap_version = Some(attr.unescape_value()?.to_string());
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
                            // <soap:operation>
                            Event::Empty(e) | Event::Start(e)
                                if e.local_name().as_ref().ends_with(b"operation") =>
                            {
                                for attr in e.attributes().with_checks(false) {
                                    let attr = attr?;
                                    match attr.key.as_ref() {
                                        b"soapAction" => {
                                            soap_action = Some(attr.unescape_value()?.to_string())
                                        }
                                        b"style" => {
                                            style = Some(attr.unescape_value()?.to_string())
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Event::End(e) if e.local_name().as_ref() == b"operation" => break,
                            _ => {}
                        }
                        buf.clear()
                    }

                    if let Some(name) = op_name {
                        if is_soap_binding {
                            if soap_action.is_none() {
                                eprintln!(
                                    "Warning: SOAP operation '{}' in binding '{}' missing 'soapAction'.",
                                    name,
                                    self.model
                                        .bindings
                                        .last()
                                        .map(|b| &b.name)
                                        .unwrap_or(&String::new())
                                );
                            }
                            // 'style'-attribute might be in <binding> as in <operation>.
                        }
                        // TODO parse operation faults
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
            let soap_version= soap_version.unwrap_or_else(|| {
                eprintln!(
                    "Warning: SOAP binding '{}' missing SOAP version information. Assuming default (e.g., 1.1).",
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

    fn parse_service(&mut self, ev: &BytesStart) -> Result<(), Box<dyn Error>> {
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

                    loop {
                        match self.reader.read_event_into(&mut buf)? {
                            Event::Empty(e) | Event::Start(e)
                                if e.name().as_ref().ends_with(b"address") =>
                            {
                                for attr in e.attributes().with_checks(false) {
                                    let attr = attr?;
                                    if attr.key.as_ref() == b"location" {
                                        address = Some(attr.unescape_value()?.to_string());
                                    }
                                }
                            }
                            Event::End(e) if e.local_name().as_ref() == b"port" => break,
                            _ => {}
                        }
                        buf.clear();
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

#[test]
fn parses_wsdl() {
    let wsdl = r#"
    <wsdl:definitions name="MultiService"
                 targetNamespace="http://example.com/wsdl"
                 xmlns:tns="http://example.com/wsdl"
                 xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap/"
                 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                 xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/">

      <wsdl:message name="SayHelloRequest">
        <wsdl:part name="helloRequestName" element="tns:SayHello"/>
      </wsdl:message>

      <wsdl:message name="SayHelloResponse">
        <wsdl:part name="helloResponseName" type="xsd:string"/>
      </wsdl:message>

      <wsdl:portType name="HelloPortType">
        <wsdl:operation name="sayHello">
          <wsdl:input message="tns:SayHelloRequest"/>
          <wsdl:output message="tns:SayHelloResponse"/>
        </wsdl:operation>
        <wsdl:operation name="sayGoodbye">
             <wsdl:input message="tns:SayGoodbyeRequest"/>
             <wsdl:output message="tns:SayGoodbyeResponse"/>
         </wsdl:operation>
      </wsdl:portType>

      <wsdl:portType name="IgnoredHttpPortType">
        <wsdl:operation name="ignoredOp">
          <wsdl:input message="tns:HelloRequest"/>
          <wsdl:output message="tns:HelloResponse"/>
        </wsdl:operation>
      </wsdl:portType>

      <wsdl:binding name="HelloBinding" type="tns:HelloPortType">
        <soap:binding transport="http://schemas.xmlsoap.org/soap/http" style="document" version="1.1"/>
        <wsdl:operation name="sayHello">
          <soap:operation soapAction="sayHelloAction" style="document"/>
        </wsdl:operation>
      </wsdl:binding>

      <wsdl:binding name="HttpBinding" type="tns:IgnoredHttpPortType">
        <!-- intentionally no soap:binding -->
        <wsdl:operation name="ignoredOp"/>
      </wsdl:binding>

      <wsdl:service name="ServiceOne">
        <wsdl:port name="HelloPort" binding="tns:HelloBinding">
          <soap:address location="http://example.com/hello1"/>
        </wsdl:port>
        <wsdl:port name="HttpPort" binding="tns:HttpBinding">
          <!-- not a SOAP port -->
        </wsdl:port>
      </wsdl:service>

      <wsdl:service name="ServiceTwo">
        <wsdl:port name="HelloPort2" binding="tns:HelloBinding">
          <soap:address location="http://example.com/hello2"/>
        </wsdl:port>
      </wsdl:service>

    </wsdl:definitions>
"#;

    let parser = WsdlParser::new(Reader::from_str(wsdl));
    let wsdl_model = parser.parse().unwrap();

    assert_eq!(wsdl_model.messages.len(), 2);

    let request_message = &wsdl_model.messages[0];
    assert_eq!(request_message.name, "SayHelloRequest");
    assert_eq!(request_message.parts.len(), 1);
    assert_eq!(request_message.parts[0].name, "helloRequestName");
    assert_eq!(
        request_message.parts[0]
            .element
            .as_ref()
            .map(|q| q.0.as_str()),
        Some("tns:SayHello")
    );

    let response_message = &wsdl_model.messages[1];
    assert_eq!(response_message.name, "SayHelloResponse");
    assert_eq!(response_message.parts.len(), 1);
    assert_eq!(response_message.parts[0].name, "helloResponseName");
    assert_eq!(
        response_message.parts[0]
            .type_
            .as_ref()
            .map(|q| q.0.as_str()),
        Some("xsd:string")
    );

    assert_eq!(wsdl_model.port_types.len(), 2);
    let port_type = &wsdl_model.port_types[0];
    assert_eq!(port_type.name, "HelloPortType");
    assert_eq!(port_type.operations.len(), 2);

    let first_op = &port_type.operations[0];
    assert_eq!(first_op.name, "sayHello");
    assert_eq!(
        first_op.input.as_ref().map(|q| q.0.as_str()),
        Some("tns:SayHelloRequest")
    );
    assert_eq!(
        first_op.output.as_ref().map(|q| q.0.as_str()),
        Some("tns:SayHelloResponse")
    );

    let second_op = &port_type.operations[1];
    assert_eq!(second_op.name, "sayGoodbye");
    assert_eq!(
        second_op.input.as_ref().map(|q| q.0.as_str()),
        Some("tns:SayGoodbyeRequest")
    );
    assert_eq!(
        second_op.output.as_ref().map(|q| q.0.as_str()),
        Some("tns:SayGoodbyeResponse")
    );

    assert_eq!(wsdl_model.bindings.len(), 1);
    let binding = &wsdl_model.bindings[0];
    assert_eq!(binding.name, "HelloBinding");
    assert_eq!(binding.type_.0.as_str(), "tns:HelloPortType");
    assert_eq!(binding.transport, "http://schemas.xmlsoap.org/soap/http");
    assert_eq!(binding.operations.len(), 1);

    let op = &binding.operations[0];
    assert_eq!(op.name, "sayHello");
    assert_eq!(op.soap_action.as_deref(), Some("sayHelloAction"));
    assert_eq!(op.style.as_deref(), Some("document"));

    assert_eq!(wsdl_model.services.len(), 2);
    let service = &wsdl_model.services[0];
    assert_eq!(service.name, "ServiceOne");
    assert_eq!(service.ports.len(), 1);

    let port = &service.ports[0];
    assert_eq!(port.name, "HelloPort");
    assert_eq!(port.binding.0.as_str(), "tns:HelloBinding");
    assert_eq!(port.address, "http://example.com/hello1");
}
