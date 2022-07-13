pub mod wsdl;
mod schema;

use std::collections::HashMap;
use crate::codegen::{CodeGenEndpoint, CodeGenModel, CodeGenOperation};

#[derive(Default, Debug, Clone)]
pub struct QName(String);

impl QName {
    pub fn prefix(&self) -> Option<&str> {
        self.0.split(':').next()
    }
}

#[derive(Default, Debug)]
pub struct WsdlModel {
    name: Option<String>,
    target_namespace: Option<String>,
    namespaces: HashMap<String, String>,
    messages: Vec<Message>,
    port_types: Vec<PortType>,
    bindings: Vec<Binding>,
    services: Vec<Service>,
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
}

#[derive(Debug)]
pub struct Binding {
    pub name: String,
    pub type_: QName,
    pub transport: String, // e.g. "http://schemas.xmlsoap.org/soap/http"
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
    pub fn into_codegen_model(&self) -> CodeGenModel {
        todo!()
        // let mut services = Vec::new();
        //
        // for service in &self.services {
        //     for port in &service.ports {
        //         let mut operations = Vec::new();
        //
        //         let Some(address) = &port.address else {
        //             //warn!("Missing port address");
        //             continue;
        //         };
        //         let binding = match self.bindings.iter().find(|b| b.name == port.binding) {
        //             Some(b) => b,
        //             None => continue,
        //         };
        //
        //         if binding.soap_version.is_none() {
        //             continue;
        //         }
        //
        //         let port_type_map: HashMap<_, _> = self
        //             .port_types
        //             .iter()
        //             .map(|pt| (pt.name.clone(), pt))
        //             .collect();
        //
        //         let port_type = match port_type_map.get(&binding.type_) {
        //             Some(pt) => pt,
        //             None => continue,
        //         };
        //
        //         let binding_ops: HashMap<_, _> = binding
        //             .operations
        //             .iter()
        //             .map(|bo| (bo.name.clone(), bo))
        //             .collect();
        //
        //         for pt_op in &port_type.operations {
        //             let bo = binding_ops.get(&pt_op.name);
        //
        //             let input_msg = pt_op
        //                 .input
        //                 .as_ref()
        //                 .and_then(|name| self.messages.iter().find(|m| m.name == *name));
        //             let output_msg = pt_op
        //                 .output
        //                 .as_ref()
        //                 .and_then(|name| self.messages.iter().find(|m| m.name == *name));
        //
        //             operations.push(CodeGenOperation {
        //                 name: pt_op.name.clone(),
        //                 soap_action: bo.and_then(|b| b.soap_action.clone()),
        //                 style: bo.and_then(|b| b.style.clone()),
        //                 input: input_msg.cloned(),
        //                 output: output_msg.cloned(),
        //             });
        //         }
        //
        //         services.push(CodeGenEndpoint {
        //             name: service.name.clone(),
        //             port_name: port.name.clone(),
        //             address: address.clone(),
        //             operations,
        //         });
        //     }
        // }
        //
        // CodeGenModel {
        //     services,
        //     messages: self
        //         .messages
        //         .iter()
        //         .map(|m| (m.name.clone(), m.clone()))
        //         .collect(),
        // }
    }
}
