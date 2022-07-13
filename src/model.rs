pub const NAMESPACE_WSDL: &str = "http://schemas.xmlsoap.org/wsdl/";
pub const NAMESPACE_SOAP: &str = "http://schemas.xmlsoap.org/wsdl/soap/";
pub const NAMESPACE_HTTP: &str = "http://schemas.xmlsoap.org/wsdl/http/";
pub const NAMESPACE_MIME: &str = "http://schemas.xmlsoap.org/wsdl/mime/";
pub const NAMESPACE_SOAP_ENC: &str = "http://schemas.xmlsoap.org/soap/encoding/";
pub const NAMESPACE_SOAP_ENV: &str = "http://schemas.xmlsoap.org/soap/envelope/";
pub const NAMESPACE_XSI: &str = "http://www.w3.org/2000/10/XMLSchema-instance";
pub const NAMESPACE_XSD_2000_10: &str = "http://www.w3.org/2000/10/XMLSchema";
pub const NAMESPACE_XSD_2001: &str = "http://www.w3.org/2001/XMLSchema";

#[derive(Debug)]
pub enum Error {
    MissingTypes,
    MissingTypesChildren,
    MissingTypesSchema,
    MissingTypeName,
}

#[derive(Debug)]
pub struct Address {
    pub location: String,
}
// w3.org: a container for data type definitions using some type system (such as XSD).
#[derive(Debug)]
pub struct Type {
    pub namespace: String,
    pub element: xmltree::Element,
}

// w3.org: an abstract, typed definition of the data being communicated.
#[derive(Debug)]
pub struct Part {
    pub name: String,
    pub element_ref: String,
}
#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub parts: Vec<Part>,
}
// w3.org: an abstract description of an action supported by the service.
#[derive(Debug)]
pub struct Operation {
    pub name: String,
    pub action: String,
}
// w3.org: an abstract set of operations supported by one or more endpoints.
#[derive(Debug)]
pub struct PortType {
    pub operation: Operation,
}
// w3.org: a concrete protocol and data format specification for a particular port type.
#[derive(Debug)]
pub struct Binding {
    style: String,
    transport: String,
    operation: Operation,
}
// w3.org: a single endpoint defined as a combination of a binding and a network address.
#[derive(Debug)]
pub struct Port {
    pub name: String,
    pub binding: Binding,
    pub address: Address,
}

// w3.org: a collection of related endpoints.
#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub documentation: String,
    pub port: Port,
}

#[derive(Debug)]
pub struct Wsdl {
    pub(crate) target_namespace: String,
    pub(crate) types: Vec<Type>,
    pub(crate) messages: Vec<Message>,
}
