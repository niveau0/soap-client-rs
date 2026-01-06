//! Parsing of XSD simpleType definitions and utility functions

use quick_xml::events::{BytesStart, Event};
use std::error::Error;

use super::parser::SchemaParser;
use super::{Restriction, SimpleType};
use crate::parser::QName;

impl<B: std::io::BufRead> SchemaParser<B> {
    /// Parse a <simpleType> definition
    ///
    /// SimpleTypes define restrictions on built-in types, such as enumerations,
    /// patterns, or value ranges. Currently, we primarily support enumerations
    /// which are used to generate Rust enums.
    ///
    /// Other simpleType restrictions (pattern, minLength, etc.) are handled
    /// by the type mapper which maps them to appropriate Rust types.
    ///
    /// Example:
    /// ```xml
    /// <simpleType name="ColorType">
    ///   <restriction base="xs:string">
    ///     <enumeration value="Red"/>
    ///     <enumeration value="Green"/>
    ///     <enumeration value="Blue"/>
    ///   </restriction>
    /// </simpleType>
    /// ```
    pub(super) fn parse_simple_type(&mut self, e: &BytesStart) -> Result<(), Box<dyn Error>> {
        // Extract the name attribute
        let name = e
            .try_get_attribute("name")?
            .map(|a| a.unescape_value().unwrap().into_owned());

        let mut simple_type: Option<SimpleType> = None;
        let mut buf = Vec::new();

        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(e) if e.local_name().as_ref() == b"restriction" => {
                    simple_type = Some(self.parse_restriction(&e)?);
                }
                Event::Empty(e) if e.local_name().as_ref() == b"restriction" => {
                    // Empty restriction - extract base type
                    let base = e
                        .try_get_attribute("base")?
                        .map(|a| QName(a.unescape_value().unwrap().into_owned()))
                        .unwrap_or_else(|| QName("xs:string".to_string()));
                    simple_type = Some(SimpleType::Restriction {
                        base,
                        restrictions: vec![],
                    });
                }
                Event::Start(e) if e.local_name().as_ref() == b"list" => {
                    let item_type = e
                        .try_get_attribute("itemType")?
                        .map(|a| QName(a.unescape_value().unwrap().into_owned()))
                        .unwrap_or_else(|| QName("xs:string".to_string()));
                    simple_type = Some(SimpleType::List { item_type });
                    self.skip_element()?;
                }
                Event::Start(e) if e.local_name().as_ref() == b"union" => {
                    let member_types_str = e
                        .try_get_attribute("memberTypes")?
                        .map(|a| a.unescape_value().unwrap().into_owned())
                        .unwrap_or_default();
                    let member_types = member_types_str
                        .split_whitespace()
                        .map(|s| QName(s.to_string()))
                        .collect();
                    simple_type = Some(SimpleType::Union { member_types });
                    self.skip_element()?;
                }
                Event::End(e) if e.local_name().as_ref() == b"simpleType" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        // Store the simple type in the model if it has a name
        if let (Some(n), Some(st)) = (name, simple_type) {
            self.model.simple_types.insert(n, st);
        }

        Ok(())
    }

    /// Parse a <restriction> element within a simpleType
    ///
    /// This extracts the base type and all restriction facets (enumerations, patterns, etc.)
    fn parse_restriction(&mut self, e: &BytesStart) -> Result<SimpleType, Box<dyn Error>> {
        // Extract base attribute
        let base = e
            .try_get_attribute("base")?
            .map(|a| QName(a.unescape_value().unwrap().into_owned()))
            .unwrap_or_else(|| QName("xs:string".to_string()));

        let mut restrictions = Vec::new();
        let mut buf = Vec::new();

        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Empty(e) => {
                    let name = e.local_name();
                    match name.as_ref() {
                        b"enumeration" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                restrictions
                                    .push(Restriction::Enumeration(value.unescape_value()?.into_owned()));
                            }
                        }
                        b"pattern" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                restrictions
                                    .push(Restriction::Pattern(value.unescape_value()?.into_owned()));
                            }
                        }
                        b"minLength" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                if let Ok(len) = value.unescape_value()?.parse::<u32>() {
                                    restrictions.push(Restriction::MinLength(len));
                                }
                            }
                        }
                        b"maxLength" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                if let Ok(len) = value.unescape_value()?.parse::<u32>() {
                                    restrictions.push(Restriction::MaxLength(len));
                                }
                            }
                        }
                        b"length" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                if let Ok(len) = value.unescape_value()?.parse::<u32>() {
                                    restrictions.push(Restriction::Length(len));
                                }
                            }
                        }
                        b"minInclusive" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                restrictions.push(Restriction::MinInclusive(
                                    value.unescape_value()?.into_owned(),
                                ));
                            }
                        }
                        b"maxInclusive" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                restrictions.push(Restriction::MaxInclusive(
                                    value.unescape_value()?.into_owned(),
                                ));
                            }
                        }
                        b"minExclusive" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                restrictions.push(Restriction::MinExclusive(
                                    value.unescape_value()?.into_owned(),
                                ));
                            }
                        }
                        b"maxExclusive" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                restrictions.push(Restriction::MaxExclusive(
                                    value.unescape_value()?.into_owned(),
                                ));
                            }
                        }
                        b"totalDigits" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                if let Ok(digits) = value.unescape_value()?.parse::<u32>() {
                                    restrictions.push(Restriction::TotalDigits(digits));
                                }
                            }
                        }
                        b"fractionDigits" => {
                            if let Some(value) = e.try_get_attribute("value")? {
                                if let Ok(digits) = value.unescape_value()?.parse::<u32>() {
                                    restrictions.push(Restriction::FractionDigits(digits));
                                }
                            }
                        }
                        _ => {} // Ignore unknown restriction facets
                    }
                }
                Event::End(e) if e.local_name().as_ref() == b"restriction" => break,
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        Ok(SimpleType::Restriction { base, restrictions })
    }

    /// Skip the current element and all its content
    ///
    /// This is used when we encounter elements we don't need to parse in detail,
    /// such as annotations, documentation, or unsupported simpleType restrictions.
    ///
    /// The function maintains a depth counter to correctly handle nested elements
    /// with the same name.
    pub(super) fn skip_element(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();
        let mut depth = 1;
        loop {
            match self.reader.read_event_into(&mut buf)? {
                Event::Start(_) => depth += 1,
                Event::End(_) => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(())
    }
}
