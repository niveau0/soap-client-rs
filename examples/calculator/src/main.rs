//! Calculator SOAP Client Example
//!
//! This example demonstrates using the generated SOAP client to call
//! a real public SOAP web service (Calculator Service).
//!
//! Service: http://www.dneonline.com/calculator.asmx
//! Operations: Add, Subtract, Multiply, Divide

// Include the generated SOAP client code
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));

#[tokio::main]
async fn main() -> SoapResult<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("🧮 Calculator SOAP Client Example\n");

    // Create Calculator client (uses the generated code)
    let calculator = Calculator::new("http://www.dneonline.com/calculator.asmx");

    println!("📡 Connecting to: http://www.dneonline.com/calculator.asmx\n");

    // Test Addition: 5 + 3
    println!("➕ Testing Add operation: 5 + 3");
    let add_request = Add { int_a: 5, int_b: 3 };

    match calculator.add(add_request).await {
        Ok(response) => {
            println!("   Result: {}", response.add_result);
            assert_eq!(response.add_result, 8, "Addition failed: expected 8");
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
            return Err(e);
        }
    }

    // Test Subtraction: 10 - 4
    println!("\n➖ Testing Subtract operation: 10 - 4");
    let subtract_request = Subtract {
        int_a: 10,
        int_b: 4,
    };

    match calculator.subtract(subtract_request).await {
        Ok(response) => {
            println!("   Result: {}", response.subtract_result);
            assert_eq!(
                response.subtract_result, 6,
                "Subtraction failed: expected 6"
            );
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
            return Err(e);
        }
    }

    // Test Multiplication: 7 * 6
    println!("\n✖️  Testing Multiply operation: 7 * 6");
    let multiply_request = Multiply { int_a: 7, int_b: 6 };

    match calculator.multiply(multiply_request).await {
        Ok(response) => {
            println!("   Result: {}", response.multiply_result);
            assert_eq!(
                response.multiply_result, 42,
                "Multiplication failed: expected 42"
            );
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
            return Err(e);
        }
    }

    // Test Division: 20 / 4
    println!("\n➗ Testing Divide operation: 20 / 4");
    let divide_request = Divide {
        int_a: 20,
        int_b: 4,
    };

    match calculator.divide(divide_request).await {
        Ok(response) => {
            println!("   Result: {}", response.divide_result);
            assert_eq!(response.divide_result, 5, "Division failed: expected 5");
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
            return Err(e);
        }
    }

    println!("\n✅ All operations completed successfully!");
    println!("🎉 SOAP client is working perfectly!\n");

    Ok(())
}
