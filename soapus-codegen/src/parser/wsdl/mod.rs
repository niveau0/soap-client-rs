pub mod parser;

// Parser sub-modules for different WSDL elements
mod binding;
mod definitions;
mod message;
mod port_type;
mod service;
mod types;

use crate::parser::QName;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct WsdlModel {
    name: Option<String>,
    target_namespace: Option<String>,
    namespaces: HashMap<String, String>,
    messages: Vec<Message>,
    port_types: Vec<PortType>,
    bindings: Vec<Binding>,
    services: Vec<Service>,
    schema: Option<crate::parser::XmlSchema>,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub name: String,
    pub parts: Vec<MessagePart>,
}

#[derive(Clone, Debug)]
pub struct MessagePart {
    pub name: String,
    pub element: Option<QName>,
    pub type_: Option<QName>,
}

#[derive(Debug)]
pub struct PortType {
    pub name: String,
    pub operations: Vec<PortTypeOperation>,
}

#[derive(Debug)]
pub struct Fault {
    pub name: String,
    pub message: QName,
}

#[derive(Debug)]
pub struct PortTypeOperation {
    pub name: String,
    pub input: Option<QName>,
    pub output: Option<QName>,
    pub faults: Vec<Fault>,
    /// Documentation from WSDL <wsdl:documentation> element
    pub documentation: Option<String>,
}

#[derive(Debug)]
pub struct Binding {
    pub name: String,
    pub type_: QName,
    pub transport: String,    // e.g. "http://schemas.xmlsoap.org/soap/http"
    pub soap_version: String, // e.g. für <soap:binding style="..."> or xmlns:soap="..."
    pub operations: Vec<BindingOperation>,
}

#[derive(Debug)]
pub struct BindingOperation {
    pub name: String,
    pub soap_action: Option<String>,
    pub style: Option<String>, // "document" or "rpc" for SOAP Binding
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub ports: Vec<Port>,
}

#[derive(Debug)]
pub struct Port {
    pub name: String,
    pub binding: QName,
    pub address: String,
}

impl WsdlModel {
    /// Get the service name (first service if multiple exist)
    pub fn service_name(&self) -> Option<&str> {
        self.services.first().map(|s| s.name.as_str())
    }

    /// Get an iterator over all operations
    pub fn operations(&self) -> impl Iterator<Item = &PortTypeOperation> {
        self.port_types.iter().flat_map(|pt| pt.operations.iter())
    }

    /// Get the embedded XSD schema (if any)
    pub fn schema(&self) -> Option<&crate::parser::XmlSchema> {
        self.schema.as_ref()
    }

    /// Set the XSD schema
    pub(crate) fn set_schema(&mut self, schema: crate::parser::XmlSchema) {
        self.schema = Some(schema);
    }

    /// Find a message by QName
    pub fn find_message(&self, qname: &QName) -> Option<&Message> {
        self.messages.iter().find(|m| m.name == qname.local_name())
    }

    /// Get all services
    pub fn services(&self) -> &[Service] {
        &self.services
    }

    /// Get all bindings
    pub fn bindings(&self) -> &[Binding] {
        &self.bindings
    }

    /// Get all port types
    pub fn port_types(&self) -> &[PortType] {
        &self.port_types
    }

    /// Get all messages
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Get the target namespace
    pub fn target_namespace(&self) -> Option<&str> {
        self.target_namespace.as_deref()
    }

    /// Find a binding by name
    pub fn find_binding(&self, name: &str) -> Option<&Binding> {
        self.bindings.iter().find(|b| b.name == name)
    }

    /// Find a port type by name
    pub fn find_port_type(&self, name: &str) -> Option<&PortType> {
        self.port_types.iter().find(|pt| pt.name == name)
    }

    /// Get the first service (convenience method)
    pub fn first_service(&self) -> Option<&Service> {
        self.services.first()
    }

    /// Get the endpoint URL from the first service's first port
    pub fn endpoint_url(&self) -> Option<&str> {
        self.services
            .first()
            .and_then(|s| s.ports.first())
            .map(|p| p.address.as_str())
    }

    /// Find the SOAPAction for a given operation name
    ///
    /// Searches through all bindings to find the SOAPAction header value
    /// for the specified operation.
    pub fn find_soap_action(&self, operation_name: &str) -> Option<&str> {
        for binding in &self.bindings {
            for op in &binding.operations {
                if op.name == operation_name {
                    return op.soap_action.as_deref();
                }
            }
        }
        None
    }
}
