use model::{Type, Wsdl};
use xmltree::{Element, Namespace};

mod model;
mod wsdl;

fn main() {
    env_logger::init();

    let v = std::fs::read("example.wsdl").expect("Failed to read path");
    let root = Element::parse(&v[..]).expect("Failed to parse xml");

    Wsdl::new(root).expect("Failed to parse Wsdl");
    // let target_namespace = namespaces
    //     .get("tns")
    //     .expect("Failed to get target namespace");

    // let wsdl_prefix = wsdl::find_namespace(namespaces);
    // let types = find_types(&root, wsdl_prefix).expect("Failed to get types");
}

// fn find_types<'a>(root: &'a Element, maybe_namespace: Option<&str>) -> Result<&'a Element, Error> {
//     let types = match maybe_namespace {
//         Some(ns) => root.get_child(("types", ns)),
//         None => root.get_child("types"),
//     };
//     let types_root = types.ok_or(Error::MissingTypes)?;
//     let schema = types_root
//         .get_child("schema")
//         .ok_or(Error::MissingTypesSchema)?;

//     // TODO switch on different XSD namespace versions?

//     let elements: Vec<&Element> = schema
//         .children
//         .iter()
//         .filter_map(|c| c.as_element())
//         .filter(|e| e.name == "element")
//         .collect();
//     dbg!(&elements);
//     Ok(types_root)
// }

/// Parse endpoints.
fn find_endpoints() {}
// fn parse_type(elem: &Element) -> Result<Type, Error> {
//     let name = elem.attributes.get("name").ok_or(Error::MissingTypeName)?;

//     // sometimes we have <element name="TypeName"><complexType>...</complexType></element>,
//     // sometimes we have <complexType name="TypeName">...</complexType>
//     //let current_child = elem.children.get(0).ok_or(WsdlError::Empty)?
//     //    .as_element().ok_or(WsdlError::NotAnElement)?;

//     let child = if elem.name == "complexType" {
//         elem
//     } else {
//         continue;
//         // dbg!(&elem.name);
//         // elem.children
//         //     .get(0)
//         //     .ok_or(WsdlError::Empty)?
//         //     .as_element()
//         //     .ok_or(WsdlError::NotAnElement)?
//     };

//     if child.name == "complexType" {
//         let mut fields = HashMap::new();
//         for field in child
//             .children
//             .get(0)
//             .ok_or(WsdlError::Empty)?
//             .as_element()
//             .ok_or(WsdlError::NotAnElement)?
//             .children
//             .iter()
//             .filter_map(|c| c.as_element())
//         {
//             let field_name = field
//                 .attributes
//                 .get("name")
//                 .ok_or(WsdlError::AttributeNotFound("name (field)"))?;
//             let field_type = field
//                 .attributes
//                 .get("type")
//                 .ok_or(WsdlError::AttributeNotFound("type (field)"))?;
//             let nillable = match field.attributes.get("nillable").map(|s| s.as_str()) {
//                 Some("true") => true,
//                 Some("false") => false,
//                 _ => false,
//             };

//             let min_occurs = match field.attributes.get("minOccurs").map(|s| s.as_str()) {
//                 None => None,
//                 Some("unbounded") => Some(Occurence::Unbounded),
//                 Some(n) => Some(Occurence::Num(
//                     n.parse().expect("occurence should be a number"),
//                 )),
//             };
//             let max_occurs = match field.attributes.get("maxOccurs").map(|s| s.as_str()) {
//                 None => None,
//                 Some("unbounded") => Some(Occurence::Unbounded),
//                 Some(n) => Some(Occurence::Num(
//                     n.parse().expect("occurence should be a number"),
//                 )),
//             };
//             trace!("field {:?} -> {:?}", field_name, field_type);
//             let type_attributes = TypeAttribute {
//                 nillable,
//                 min_occurs,
//                 max_occurs,
//             };

//             let simple_type = match split_namespace(field_type.as_str()) {
//                 "boolean" => SimpleType::Boolean,
//                 "string" => SimpleType::String,
//                 "int" => SimpleType::Int,
//                 "float" => SimpleType::Float,
//                 "dateTime" => SimpleType::DateTime,
//                 s => SimpleType::Complex(s.to_string()),
//             };
//             fields.insert(field_name.to_string(), (type_attributes, simple_type));
//         }

//         types.insert(name.to_string(), Type::Complex(ComplexType { fields }));
//     } else {
//         trace!("child {:#?}", child);
//         unimplemented!("not a complex type");
//     }
// }
