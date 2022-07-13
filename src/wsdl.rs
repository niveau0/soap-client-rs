// Specification:
// https://www.w3.org/TR/2001/NOTE-wsdl-20010315#:~:text=WSDL%20includes%20a%20binding%20for,the%20HTTP%20binding%20of%20SOAP

use crate::model::{self, Error, Message, Part, Type, Wsdl, PortType, Operation};
use xmltree::{Element, Namespace};

struct WsdlNamespace<'a> {
    prefix: &'a String,
    uri: &'a String,
}
impl Wsdl {
    pub(crate) fn new(root: xmltree::Element) -> Result<Self, Error> {
        let namespaces = root.namespaces.as_ref().expect("Missing namespaces");
        let wsdl_ns = wsdl_namespace(namespaces).expect("Missing wsdl namespaces");
        let wsdl_ns = WsdlNamespace {
            prefix: wsdl_ns.0,
            uri: wsdl_ns.1,
        };
        let target_namespace = target_namespace(&root);

        let types = collect_types(&root, &wsdl_ns)?;
        let messages = collect_messages(&root, &wsdl_ns)?;
        let port_types = collect_port_types(&root, &wsdl_ns)?;
        dbg!(&port_types);

        Ok(Wsdl {
            target_namespace,
            types,
            messages,
        })
    }
}

fn target_namespace(element: &Element) -> String {
    let target_namespace = element
        .attributes
        .get("targetNamespace")
        .expect("Failed to get target namespace")
        .to_owned();
    target_namespace
}

fn wsdl_namespace(namespaces: &Namespace) -> Option<(&String, &String)> {
    namespaces
        .0
        .iter()
        .find(|elem| elem.1 == model::NAMESPACE_WSDL)
}

/// Types– a container for data type definitions using some type system (such as XSD).
// <wsdl:types> ?
//   <wsdl:documentation .... />?
//   <xsd:schema .... />*
//   <-- extensibility element --> *
// </wsdl:types>
fn collect_types(root: &Element, namespace: &WsdlNamespace) -> Result<Vec<Type>, Error> {
    let types = match namespace.prefix.is_empty() {
        true => root.get_child(("types", namespace.prefix.as_str())),
        false => root.get_child("types"),
    };
    let Some(types_root) = types else {
        return Ok(vec![])
    };

    Ok(types_root
        .children
        .iter()
        .filter_map(|c| c.as_element())
        .filter(|e| e.name == "schema")
        .flat_map(|schema| {
            // TODO switch on different XSD namespace versions?
            schema
                .children
                .iter()
                .filter_map(|c| c.as_element())
                .filter(|e| e.name == "element")
                .map(|e| Type {
                    namespace: target_namespace(schema),
                    element: e.clone(),
                })
                .collect::<Vec<Type>>()
        })
        .collect::<Vec<Type>>())
}

/// Message– an abstract, typed definition of the data being communicated.
// <wsdl:message name="nmtoken"> *
//   <wsdl:documentation .... />?
//   <part name="nmtoken" element="qname"? type="qname"?/> *
// </wsdl:message>
fn collect_messages(root: &Element, namespace: &WsdlNamespace) -> Result<Vec<Message>, Error> {
    Ok(root
        .children
        .iter()
        .filter_map(|c| c.as_element())
        .filter(|e| e.name == "message" && matches!(&e.namespace, Some(ns) if ns == namespace.uri))
        .map(|msg| {
            let parts = msg
                .children
                .iter()
                .filter_map(|c| c.as_element())
                .filter(|e| e.name == "part")
                .filter(|e| {
                    e.attributes.contains_key("name") && e.attributes.contains_key("element")
                })
                .map(|e| Part {
                    name: e.attributes.get("name").unwrap().to_owned(),
                    element_ref: e.attributes.get("element").unwrap().to_owned(),
                })
                .collect::<Vec<Part>>();

            Message {
                name: msg.name.to_owned(),
                parts,
            }
        })
        .collect::<Vec<Message>>())
}

/// Operation– an abstract description of an action supported by the service.
fn collect_operations(container: &Element, namespace: &WsdlNamespace) -> Result<Vec<Operation>, Error> {
    Ok(container
        .children
        .iter()
        .filter_map(|c| c.as_element())
        .filter(|e| e.name == "operation" && matches!(&e.namespace, Some(ns) if ns == namespace.uri))
        .map(|msg| {
            Operation {
                name: msg.name.to_owned(),
                parts,
            }
        })
        .collect::<Vec<Operation>>())
}

/// Port Type–an abstract set of operations supported by one or more endpoints.
// <wsdl:portType name="nmtoken">*
//   <wsdl:documentation .... />?
//   <wsdl:operation name="nmtoken">*
//      <wsdl:documentation .... /> ?
//      <wsdl:input name="nmtoken"? message="qname">?
//        <wsdl:documentation .... /> ?
//      </wsdl:input>
//      <wsdl:output name="nmtoken"? message="qname">?
//        <wsdl:documentation .... /> ?
//      </wsdl:output>
//      <wsdl:fault name="nmtoken" message="qname"> *
//        <wsdl:documentation .... /> ?
//      </wsdl:fault>
//   </wsdl:operation>
// </wsdl:portType>
fn collect_port_types(root: &Element, namespace: &WsdlNamespace) -> Result<Vec<PortType>, Error> {
    Ok(root
        .children
        .iter()
        .filter_map(|c| c.as_element())
        .filter(|e| e.name == "portType" && matches!(&e.namespace, Some(ns) if ns == namespace.uri))
        .map(|msg| {
            
            let parts = msg
                .children
                .iter()
                .filter_map(|c| c.as_element())
                .filter(|e| e.name == "part")
                .filter(|e| {
                    e.attributes.contains_key("name") && e.attributes.contains_key("element")
                })
                .map(|e| Part {
                    name: e.attributes.get("name").unwrap().to_owned(),
                    element_ref: e.attributes.get("element").unwrap().to_owned(),
                })
                .collect::<Vec<Part>>();

            Message {
                name: msg.name.to_owned(),
                parts,
            }
        })
        .collect::<Vec<PortType>>())
}

/// Binding– a concrete protocol and data format specification for a particular port type.
// <wsdl:binding name="nmtoken" type="qname">*
//     <wsdl:documentation .... />?
//     <-- extensibility element --> *
//     <wsdl:operation name="nmtoken">*
//         <wsdl:documentation .... /> ?
//         <-- extensibility element --> *
//         <wsdl:input> ?
//             <wsdl:documentation .... /> ?
//             <-- extensibility element -->
//         </wsdl:input>
//         <wsdl:output> ?
//             <wsdl:documentation .... /> ?
//             <-- extensibility element --> *
//         </wsdl:output>
//         <wsdl:fault name="nmtoken"> *
//             <wsdl:documentation .... /> ?
//             <-- extensibility element --> *
//         </wsdl:fault>
//     </wsdl:operation>
// </wsdl:binding>
fn collect_bindings() {}

/// Service– a collection of related endpoints.
// <wsdl:service name="nmtoken"> *
//   <wsdl:documentation .... />?
//   <wsdl:port name="nmtoken" binding="qname"> *
//    <wsdl:documentation .... /> ?
//    <-- extensibility element -->
//   </wsdl:port>
//   <-- extensibility element -->
// </wsdl:service>
fn collect_services() {}

#[cfg(test)]
mod tests {
    use super::*;

    const WSDL: &[u8] = include_bytes!("../example.wsdl");

    #[test]
    fn wdsl() {
        let root = Element::parse(&WSDL[..]).expect("Failed to parse xml");
        Wsdl::new(root).expect("Failed to parse WSDL");
    }
}
