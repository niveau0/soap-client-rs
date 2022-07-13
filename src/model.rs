pub const NAMESPACE_WSDL: &'static str = "http://schemas.xmlsoap.org/wsdl/";
pub const NAMESPACE_SOAP: &'static str = "http://schemas.xmlsoap.org/wsdl/soap/";
pub const NAMESPACE_HTTP: &'static str = "http://schemas.xmlsoap.org/wsdl/http/";
pub const NAMESPACE_MIME: &'static str = "http://schemas.xmlsoap.org/wsdl/mime/";
pub const NAMESPACE_SOAP_ENC: &'static str = "http://schemas.xmlsoap.org/soap/encoding/";
pub const NAMESPACE_SOAP_ENV: &'static str = "http://schemas.xmlsoap.org/soap/envelope/";
pub const NAMESPACE_XSI: &'static str = "http://www.w3.org/2000/10/XMLSchema-instance";
pub const NAMESPACE_XSD_2000_10: &'static str = "http://www.w3.org/2000/10/XMLSchema";
pub const NAMESPACE_XSD_2001: &'static str = "http://www.w3.org/2001/XMLSchema";

pub struct Address {
    location: String,
}
// w3.org: a container for data type definitions using some type system (such as XSD).
pub struct Type;
// w3.org: an abstract, typed definition of the data being communicated.
pub struct Message {
    name: String,
}
// w3.org: an abstract description of an action supported by the service.
pub struct Operation {
    name: String,
    action: String,
}
// w3.org: an abstract set of operations supported by one or more endpoints.
pub struct PortType {
    operation: Operation,
}
// w3.org: a concrete protocol and data format specification for a particular port type.
pub struct Binding {
    style: String,
    transport: String,
    operation: Operation,
}
// w3.org: a single endpoint defined as a combination of a binding and a network address.
pub struct Port {
    name: String,
    binding: Binding,
    address: Address,
}

// w3.org: a collection of related endpoints.
pub struct Service {
    name: String,
    documentation: String,
    port: Port,
}

pub struct Wsdl {
    types: Vec<Type>,
}
