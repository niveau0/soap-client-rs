//! SOAP envelope building
//!
//! This module handles the construction of SOAP envelopes for both SOAP 1.1 and 1.2.
//! It serializes request bodies to XML and wraps them in the appropriate SOAP envelope structure.

use crate::error::{SoapError, SoapResult};
use serde::Serialize;

#[cfg(feature = "tracing")]
use tracing::debug;

// Standard SOAP envelope namespace URIs as defined by W3C SOAP specification
const SOAP_11_ENVELOPE_NS: &str = "http://schemas.xmlsoap.org/soap/envelope/";
const SOAP_12_ENVELOPE_NS: &str = "http://www.w3.org/2003/05/soap-envelope";

/// SOAP protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SoapVersion {
    /// SOAP 1.1
    #[default]
    Soap11,
    /// SOAP 1.2
    Soap12,
}

/// SOAP envelope builder
pub struct SoapEnvelope;

impl SoapEnvelope {
    /// Build a SOAP envelope with the given body
    ///
    /// # Arguments
    ///
    /// * `body` - The request body to serialize
    /// * `version` - SOAP protocol version to use
    ///
    /// # Returns
    ///
    /// The complete SOAP envelope as an XML string
    pub fn build<T>(body: &T, version: SoapVersion) -> SoapResult<String>
    where
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        debug!(soap_version = ?version, "Building SOAP envelope");

        Self::build_with_namespace(body, version, None, true)
    }

    /// Build a SOAP envelope with optional namespace on the body element
    ///
    /// # Arguments
    ///
    /// * `element_form_qualified` - If false, namespace is only added to root element,
    ///   not inherited by children (for elementFormDefault="unqualified" in XSD)
    pub fn build_with_namespace<T>(
        body: &T,
        version: SoapVersion,
        namespace: Option<&str>,
        element_form_qualified: bool,
    ) -> SoapResult<String>
    where
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        debug!(soap_version = ?version, namespace = ?namespace, element_form_qualified = %element_form_qualified, "Building SOAP envelope with namespace");

        match version {
            SoapVersion::Soap11 => Self::build_soap11(body, namespace, element_form_qualified),
            SoapVersion::Soap12 => Self::build_soap12(body, namespace, element_form_qualified),
        }
    }

    /// Build a SOAP 1.1 envelope
    ///
    /// Format:
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
    ///   <soap:Body>
    ///     <!-- serialized body content -->
    ///   </soap:Body>
    /// </soap:Envelope>
    /// ```
    pub fn build_soap11<T>(
        body: &T,
        namespace: Option<&str>,
        element_form_qualified: bool,
    ) -> SoapResult<String>
    where
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        debug!("Serializing request body to XML");

        let body_xml = if let Some(ns) = namespace {
            if element_form_qualified {
                // Serialize with namespace - quick-xml will qualify all child elements
                Self::serialize_to_xml_with_namespace(body, ns)?
            } else {
                // Serialize without namespace, then add namespace PREFIX to root element
                // Using a prefix (ns:element) instead of default namespace (xmlns="...")
                // prevents child elements from inheriting the namespace
                let xml = Self::serialize_to_xml(body)?;
                Self::add_namespace_prefix_to_root(&xml, ns, "ns")
            }
        } else {
            Self::serialize_to_xml(body)?
        };

        #[cfg(feature = "tracing")]
        debug!(body_xml_size = body_xml.len(), "Building SOAP 1.1 envelope");

        // Build envelope manually to avoid escaping the body XML
        let envelope = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><soap:Envelope xmlns:soap="{}"><soap:Body>{}</soap:Body></soap:Envelope>"#,
            SOAP_11_ENVELOPE_NS, body_xml
        );

        Ok(envelope)
    }

    /// Build a SOAP 1.2 envelope
    ///
    /// Format:
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <env:Envelope xmlns:env="http://www.w3.org/2003/05/soap-envelope">
    ///   <env:Body>
    ///     <!-- serialized body content -->
    ///   </env:Body>
    /// </env:Envelope>
    /// ```
    pub fn build_soap12<T>(
        body: &T,
        namespace: Option<&str>,
        element_form_qualified: bool,
    ) -> SoapResult<String>
    where
        T: Serialize,
    {
        #[cfg(feature = "tracing")]
        debug!("Serializing request body to XML");

        let body_xml = if let Some(ns) = namespace {
            if element_form_qualified {
                // Serialize with namespace - quick-xml will qualify all child elements
                Self::serialize_to_xml_with_namespace(body, ns)?
            } else {
                // Serialize without namespace, then add namespace PREFIX to root element
                // Using a prefix (ns:element) instead of default namespace (xmlns="...")
                // prevents child elements from inheriting the namespace
                let xml = Self::serialize_to_xml(body)?;
                Self::add_namespace_prefix_to_root(&xml, ns, "ns")
            }
        } else {
            Self::serialize_to_xml(body)?
        };

        #[cfg(feature = "tracing")]
        debug!(body_xml_size = body_xml.len(), "Building SOAP 1.2 envelope");

        // Build envelope manually to avoid escaping the body XML
        let envelope = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><env:Envelope xmlns:env="{}"><env:Body>{}</env:Body></env:Envelope>"#,
            SOAP_12_ENVELOPE_NS, body_xml
        );

        Ok(envelope)
    }

    /// Serialize a value to XML string using quick-xml
    fn serialize_to_xml<T>(value: &T) -> SoapResult<String>
    where
        T: Serialize,
    {
        quick_xml::se::to_string(value).map_err(|e| SoapError::SerializationError(e.to_string()))
    }

    /// Serialize a value to XML with namespace on the root element
    ///
    /// This adds the xmlns attribute to the root element, which is required
    /// by many SOAP services (especially .NET-based ones).
    pub fn serialize_to_xml_with_namespace<T>(value: &T, namespace: &str) -> SoapResult<String>
    where
        T: Serialize,
    {
        // First serialize with quick-xml
        let xml = quick_xml::se::to_string(value)
            .map_err(|e| SoapError::SerializationError(e.to_string()))?;

        // Add namespace to root element
        let xml_with_ns = Self::add_namespace_to_root(&xml, namespace);

        Ok(xml_with_ns)
    }

    /// Add namespace declaration to the root element of an XML string
    ///
    /// Converts `<Add>...</Add>` to `<Add xmlns="...">...</Add>`
    fn add_namespace_to_root(xml: &str, namespace: &str) -> String {
        // Find the end of the first opening tag
        if let Some(pos) = xml.find('>') {
            // Check if it's a self-closing tag
            if pos > 0 && xml.as_bytes()[pos - 1] == b'/' {
                // Self-closing tag: <Tag /> -> <Tag xmlns="..." />
                let insert_pos = pos - 1;
                let mut result = String::with_capacity(xml.len() + namespace.len() + 10);
                result.push_str(&xml[..insert_pos]);
                result.push_str(&format!(" xmlns=\"{}\"", namespace));
                result.push_str(&xml[insert_pos..]);
                result
            } else {
                // Regular tag: <Tag> -> <Tag xmlns="...">
                let mut result = String::with_capacity(xml.len() + namespace.len() + 10);
                result.push_str(&xml[..pos]);
                result.push_str(&format!(" xmlns=\"{}\"", namespace));
                result.push_str(&xml[pos..]);
                result
            }
        } else {
            // Invalid XML or empty - return as-is
            xml.to_string()
        }
    }

    /// Add namespace prefix to the root element of an XML string
    ///
    /// Converts `<Add>...</Add>` to `<prefix:Add xmlns:prefix="...">...</prefix:Add>`
    /// This prevents child elements from inheriting the namespace (for elementFormDefault="unqualified")
    fn add_namespace_prefix_to_root(xml: &str, namespace: &str, prefix: &str) -> String {
        // Find the opening and closing tags
        if let Some(start_pos) = xml.find('<') {
            if let Some(end_pos) = xml.find('>') {
                // Extract tag name
                let tag_content = &xml[start_pos + 1..end_pos];
                let tag_name = tag_content.split_whitespace().next().unwrap_or(tag_content);

                // Check if self-closing
                let is_self_closing = xml.as_bytes()[end_pos - 1] == b'/';

                if is_self_closing {
                    // Self-closing: <Tag/> -> <prefix:Tag xmlns:prefix="..."/>
                    let insert_pos = end_pos - 1;
                    format!(
                        "<{}:{} xmlns:{}=\"{}\"{}",
                        prefix,
                        tag_name,
                        prefix,
                        namespace,
                        &xml[insert_pos..]
                    )
                } else {
                    // Find closing tag
                    let closing_tag = format!("</{}>", tag_name);
                    if let Some(close_pos) = xml.rfind(&closing_tag) {
                        // <Tag>content</Tag> -> <prefix:Tag xmlns:prefix="...">content</prefix:Tag>
                        let content = &xml[end_pos + 1..close_pos];
                        format!(
                            "<{}:{} xmlns:{}=\"{}\">{}</{}:{}>",
                            prefix, tag_name, prefix, namespace, content, prefix, tag_name
                        )
                    } else {
                        // Fallback to default namespace
                        Self::add_namespace_to_root(xml, namespace)
                    }
                }
            } else {
                xml.to_string()
            }
        } else {
            xml.to_string()
        }
    }

    /// Fix unescaped ampersands in XML
    ///
    /// Replaces `&` with `&amp;` unless it's part of a valid entity reference.
    /// This handles server bugs where text contains unescaped ampersands like "1&4".
    fn fix_unescaped_ampersands(xml: &str) -> String {
        let mut result = String::with_capacity(xml.len());
        let mut chars = xml.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '&' {
                // Check if this is a valid entity reference
                let lookahead: String = chars.clone().take(10).collect();
                if lookahead.starts_with("amp;")
                    || lookahead.starts_with("lt;")
                    || lookahead.starts_with("gt;")
                    || lookahead.starts_with("quot;")
                    || lookahead.starts_with("apos;")
                    || lookahead.starts_with("#")
                {
                    // Valid entity, keep as-is
                    result.push(ch);
                } else {
                    // Invalid, escape it
                    result.push_str("&amp;");
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Parse a SOAP response and extract the body content
    ///
    /// This function extracts the content between `<soap:Body>` or `<env:Body>` tags
    /// and deserializes it into the expected response type.
    pub fn parse_response<T>(xml: &str) -> SoapResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        #[cfg(feature = "tracing")]
        debug!(response_size = xml.len(), "Parsing SOAP response");

        // Fix invalid XML: replace unescaped & with &amp;
        // This handles server bugs where text contains unescaped ampersands
        let fixed_xml = Self::fix_unescaped_ampersands(xml);

        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(&fixed_xml);

        let mut buf = Vec::new();
        let mut in_body = false;
        let mut body_content = String::new();
        let mut depth = 0;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = e.name();
                    let local_name = name.as_ref();

                    // Check if this is a Body element (SOAP 1.1 or 1.2)
                    if local_name.ends_with(b"Body") {
                        in_body = true;
                        depth = 0;
                    } else if in_body {
                        depth += 1;
                        // Capture the start tag
                        let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                        body_content.push('<');
                        body_content.push_str(&tag);

                        // Add attributes
                        for attr in e.attributes().flatten() {
                            body_content.push(' ');
                            body_content.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                            body_content.push_str("=\"");
                            body_content.push_str(&String::from_utf8_lossy(&attr.value));
                            body_content.push('"');
                        }
                        body_content.push('>');
                    }
                }
                Ok(Event::End(e)) => {
                    let name = e.name();
                    let local_name = name.as_ref();

                    if local_name.ends_with(b"Body") && in_body && depth == 0 {
                        // End of Body - we're done
                        break;
                    } else if in_body {
                        depth -= 1;
                        let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                        body_content.push_str("</");
                        body_content.push_str(&tag);
                        body_content.push('>');
                    }
                }
                Ok(Event::Text(e)) if in_body => {
                    body_content.push_str(&e.unescape().unwrap_or_default());
                }
                Ok(Event::Empty(e)) if in_body => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    body_content.push('<');
                    body_content.push_str(&tag);

                    // Add attributes
                    for attr in e.attributes().flatten() {
                        body_content.push(' ');
                        body_content.push_str(&String::from_utf8_lossy(attr.key.as_ref()));
                        body_content.push_str("=\"");
                        body_content.push_str(&String::from_utf8_lossy(&attr.value));
                        body_content.push('"');
                    }
                    body_content.push_str("/>");
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(SoapError::XmlError(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        if body_content.is_empty() {
            return Err(SoapError::InvalidResponse(
                "No body content found in SOAP response".to_string(),
            ));
        }

        #[cfg(feature = "tracing")]
        debug!(
            body_content_size = body_content.len(),
            "Extracted body content from SOAP response"
        );

        // Fix unescaped ampersands in body content before deserialization
        let fixed_body_content = Self::fix_unescaped_ampersands(&body_content);

        // Deserialize the body content
        quick_xml::de::from_str(&fixed_body_content)
            .map_err(|e| SoapError::DeserializationError(e.to_string()))
    }

    /// Check if a SOAP response contains a fault
    pub fn check_for_fault(xml: &str) -> SoapResult<()> {
        #[cfg(feature = "tracing")]
        debug!("Checking SOAP response for faults");

        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(xml);

        let mut buf = Vec::new();
        let mut in_fault = false;
        let mut fault_code = String::new();
        let mut fault_string = String::new();
        let mut in_faultcode = false;
        let mut in_faultstring = false;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = e.name();
                    let local_name = name.as_ref();

                    if local_name.ends_with(b"Fault") {
                        in_fault = true;
                    } else if in_fault {
                        if local_name.ends_with(b"faultcode") || local_name.ends_with(b"Code") {
                            in_faultcode = true;
                        } else if local_name.ends_with(b"faultstring")
                            || local_name.ends_with(b"Reason")
                        {
                            in_faultstring = true;
                        }
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_faultcode {
                        fault_code = e.unescape().unwrap_or_default().to_string();
                        in_faultcode = false;
                    } else if in_faultstring {
                        fault_string = e.unescape().unwrap_or_default().to_string();
                        in_faultstring = false;
                    }
                }
                Ok(Event::End(e)) => {
                    let name = e.name();
                    let local_name = name.as_ref();

                    if local_name.ends_with(b"Fault") {
                        // We found a fault - return error
                        return Err(SoapError::SoapFault {
                            code: fault_code,
                            message: fault_string,
                            detail: None,
                        });
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(SoapError::XmlError(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestRequest {
        name: String,
        value: i32,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        result: String,
    }

    #[test]
    fn test_build_soap11_envelope() {
        let request = TestRequest {
            name: "test".to_string(),
            value: 42,
        };

        let envelope = SoapEnvelope::build_soap11(&request, None, true).unwrap();
        println!("SOAP 1.1 Envelope:\n{}", envelope);

        assert!(envelope.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(envelope.contains("<soap:Envelope"));
        assert!(envelope.contains(&format!("xmlns:soap=\"{}\"", SOAP_11_ENVELOPE_NS)));
        assert!(envelope.contains("<soap:Body>"));
        assert!(envelope.contains("</soap:Body>"));
        assert!(envelope.contains("</soap:Envelope>"));
        assert!(envelope.contains("<name>test</name>"));
        assert!(envelope.contains("<value>42</value>"));
    }

    #[test]
    fn test_build_soap12_envelope() {
        let request = TestRequest {
            name: "test".to_string(),
            value: 42,
        };

        let envelope = SoapEnvelope::build_soap12(&request, None, true).unwrap();
        println!("SOAP 1.2 Envelope:\n{}", envelope);

        assert!(envelope.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(envelope.contains("<env:Envelope"));
        assert!(envelope.contains(&format!("xmlns:env=\"{}\"", SOAP_12_ENVELOPE_NS)));
        assert!(envelope.contains("<env:Body>"));
        assert!(envelope.contains("</env:Body>"));
        assert!(envelope.contains("</env:Envelope>"));
        assert!(envelope.contains("<name>test</name>"));
        assert!(envelope.contains("<value>42</value>"));
    }

    #[test]
    fn test_build_with_version() {
        let request = TestRequest {
            name: "test".to_string(),
            value: 42,
        };

        let envelope11 = SoapEnvelope::build(&request, SoapVersion::Soap11).unwrap();
        assert!(envelope11.contains("soap:Envelope"));

        let envelope12 = SoapEnvelope::build(&request, SoapVersion::Soap12).unwrap();
        assert!(envelope12.contains("env:Envelope"));
    }

    #[test]
    fn test_parse_soap11_response() {
        let response_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <TestResponse>
      <result>success</result>
    </TestResponse>
  </soap:Body>
</soap:Envelope>"#;

        let response: TestResponse = SoapEnvelope::parse_response(response_xml).unwrap();
        assert_eq!(response.result, "success");
    }

    #[test]
    fn test_parse_soap12_response() {
        let response_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<env:Envelope xmlns:env="http://www.w3.org/2003/05/soap-envelope">
  <env:Body>
    <TestResponse>
      <result>success</result>
    </TestResponse>
  </env:Body>
</env:Envelope>"#;

        let response: TestResponse = SoapEnvelope::parse_response(response_xml).unwrap();
        assert_eq!(response.result, "success");
    }

    #[test]
    fn test_check_for_fault_no_fault() {
        let response_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <TestResponse>
      <result>success</result>
    </TestResponse>
  </soap:Body>
</soap:Envelope>"#;

        let result = SoapEnvelope::check_for_fault(response_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_for_fault_with_fault() {
        let fault_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <soap:Fault>
      <faultcode>soap:Server</faultcode>
      <faultstring>Internal Server Error</faultstring>
    </soap:Fault>
  </soap:Body>
</soap:Envelope>"#;

        let result = SoapEnvelope::check_for_fault(fault_xml);
        assert!(result.is_err());

        if let Err(SoapError::SoapFault { code, message, .. }) = result {
            assert_eq!(code, "soap:Server");
            assert_eq!(message, "Internal Server Error");
        } else {
            panic!("Expected SoapFault error");
        }
    }

    #[test]
    fn test_default_soap_version() {
        assert_eq!(SoapVersion::default(), SoapVersion::Soap11);
    }

    #[test]
    fn test_element_form_qualified() {
        #[derive(Serialize)]
        struct TestRequest {
            user_name: String,
            password: String,
        }

        let request = TestRequest {
            user_name: "admin".to_string(),
            password: "secret".to_string(),
        };

        // Test with element_form_qualified = true (default)
        let envelope_qualified =
            SoapEnvelope::build_soap11(&request, Some("urn:test"), true).unwrap();

        // Should have namespace on root and children inherit it
        assert!(envelope_qualified.contains("<TestRequest xmlns=\"urn:test\">"));

        // Test with element_form_qualified = false (unqualified children)
        let envelope_unqualified =
            SoapEnvelope::build_soap11(&request, Some("urn:test"), false).unwrap();

        // Should have namespace prefix on root element
        assert!(envelope_unqualified.contains("<ns:TestRequest xmlns:ns=\"urn:test\">"));
        assert!(envelope_unqualified.contains("</ns:TestRequest>"));
        // Child elements should have NO prefix (no namespace)
        assert!(envelope_unqualified.contains("<user_name>admin</user_name>"));
        assert!(envelope_unqualified.contains("<password>secret</password>"));
    }
}

#[test]
fn test_fix_unescaped_ampersands() {
    // Test valid entities - should remain unchanged
    assert_eq!(
        SoapEnvelope::fix_unescaped_ampersands("&amp; &lt; &gt; &quot; &apos;"),
        "&amp; &lt; &gt; &quot; &apos;"
    );

    // Test numeric entities - should remain unchanged
    assert_eq!(
        SoapEnvelope::fix_unescaped_ampersands("&#123; &#xAB;"),
        "&#123; &#xAB;"
    );

    // Test unescaped ampersand - should be escaped
    assert_eq!(SoapEnvelope::fix_unescaped_ampersands("1&4"), "1&amp;4");

    // Test real-world example from Rotorsoft
    assert_eq!(
        SoapEnvelope::fix_unescaped_ampersands("<name>Ville 1&4 (E)</name>"),
        "<name>Ville 1&amp;4 (E)</name>"
    );

    // Test mixed content
    assert_eq!(
        SoapEnvelope::fix_unescaped_ampersands("A&B &amp; C&D"),
        "A&amp;B &amp; C&amp;D"
    );
}

#[test]
fn test_parse_response_with_unescaped_ampersand() {
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct TestResponse {
        name: String,
    }

    // Simulate a SOAP response with unescaped ampersand (like from Rotorsoft)
    let soap_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <TestResponse>
      <name>Ville 1&4 (E)</name>
    </TestResponse>
  </soap:Body>
</soap:Envelope>"#;

    let result: Result<TestResponse, _> = SoapEnvelope::parse_response(soap_xml);

    match &result {
        Ok(_) => {}
        Err(e) => eprintln!("Parse error: {}", e),
    }

    assert!(
        result.is_ok(),
        "Should successfully parse despite unescaped ampersand: {:?}",
        result.err()
    );
    let response = result.unwrap();
    assert_eq!(response.name, "Ville 1&4 (E)");
}
