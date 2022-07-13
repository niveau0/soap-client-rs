use crate::parser::Message;
use std::collections::HashMap;

pub mod generator;

pub struct CodeGenModel {
    pub services: Vec<CodeGenEndpoint>,
    pub messages: HashMap<String, Message>,
}

pub struct CodeGenEndpoint {
    pub name: String,
    pub port_name: String,
    pub address: String,
    pub operations: Vec<CodeGenOperation>,
}

pub struct CodeGenOperation {
    pub name: String,
    pub soap_action: Option<String>,
    pub style: Option<String>,
    pub input: Option<Message>,
    pub output: Option<Message>,
}
