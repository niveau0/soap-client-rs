//! Observability Example for SOAP Client
//!
//! This example demonstrates:
//! - Tracing with OpenTelemetry/Jaeger
//! - Metrics with Prometheus
//! - SOAP client calls with full observability
//! - Axum web server with /metrics endpoint
//!
//! Run with:
//! ```bash
//! docker-compose up -d  # Start Jaeger + Prometheus
//! cargo run             # Run this example
//! ```
//!
//! Then visit:
//! - Jaeger UI: http://localhost:16686
//! - Prometheus: http://localhost:9090
//! - Metrics endpoint: http://localhost:3000/metrics
//! - Service endpoint: http://localhost:3000/calculate

// Include the generated SOAP client code
include!(concat!(env!("OUT_DIR"), "/soap_client.rs"));

use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize OpenTelemetry with Jaeger exporter
fn init_tracer() -> opentelemetry_sdk::trace::Tracer {
    opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("soap-client-observability-example")
        .with_auto_split_batch(true)
        .with_max_packet_size(9_216)
        .with_endpoint("localhost:6831")
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("Failed to install Jaeger tracer")
}

/// Initialize Prometheus metrics exporter
fn init_metrics() -> PrometheusHandle {
    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("soap_request_duration_seconds".to_string()),
            &[
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ],
        )
        .unwrap()
        .set_buckets_for_metric(
            Matcher::Full("soap_response_size_bytes".to_string()),
            &[100.0, 500.0, 1000.0, 5000.0, 10000.0, 50000.0, 100000.0],
        )
        .unwrap()
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
}

/// Initialize tracing subscriber with OpenTelemetry layer
fn init_tracing(tracer: opentelemetry_sdk::trace::Tracer) {
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    // Use JSON format if RUST_LOG_JSON=1, otherwise use default format
    if std::env::var("RUST_LOG_JSON").is_ok() {
        tracing_subscriber::registry()
            .with(telemetry)
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(telemetry)
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

/// Query parameters for calculate endpoint
#[derive(Debug, serde::Deserialize)]
struct CalculateParams {
    a: i32,
    b: i32,
    #[serde(default = "default_operation")]
    op: String,
}

fn default_operation() -> String {
    "add".to_string()
}

/// Handler for /metrics endpoint
async fn metrics_handler(State(prometheus): State<PrometheusHandle>) -> Response {
    prometheus.render().into_response()
}

/// Handler for /calculate endpoint
#[tracing::instrument(name = "calculate_endpoint", skip(calculator))]
async fn calculate_handler(
    State(calculator): State<Calculator>,
    Query(params): Query<CalculateParams>,
) -> Result<String, String> {
    tracing::info!(
        operation = %params.op,
        a = params.a,
        b = params.b,
        "Received calculation request"
    );

    let result = match params.op.as_str() {
        "add" => {
            let request = Add {
                int_a: params.a,
                int_b: params.b,
            };
            calculator
                .add(request)
                .await
                .map(|r| r.add_result)
                .map_err(|e| format!("Add failed: {}", e))?
        }
        "subtract" => {
            let request = Subtract {
                int_a: params.a,
                int_b: params.b,
            };
            calculator
                .subtract(request)
                .await
                .map(|r| r.subtract_result)
                .map_err(|e| format!("Subtract failed: {}", e))?
        }
        "multiply" => {
            let request = Multiply {
                int_a: params.a,
                int_b: params.b,
            };
            calculator
                .multiply(request)
                .await
                .map(|r| r.multiply_result)
                .map_err(|e| format!("Multiply failed: {}", e))?
        }
        "divide" => {
            let request = Divide {
                int_a: params.a,
                int_b: params.b,
            };
            calculator
                .divide(request)
                .await
                .map(|r| r.divide_result)
                .map_err(|e| format!("Divide failed: {}", e))?
        }
        _ => return Err(format!("Unknown operation: {}", params.op)),
    };

    tracing::info!(result = result, "Calculation completed");
    Ok(format!(
        "{} {} {} = {}\n",
        params.a, params.op, params.b, result
    ))
}

/// Handler for root endpoint (info page)
async fn root_handler() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>SOAP Client Observability Example</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 50px auto; padding: 20px; }
        h1 { color: #333; }
        .endpoint { background: #f4f4f4; padding: 10px; margin: 10px 0; border-radius: 5px; }
        code { background: #e0e0e0; padding: 2px 5px; border-radius: 3px; }
        a { color: #0066cc; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <h1>🔍 SOAP Client Observability Example</h1>
    <p>This example demonstrates distributed tracing and metrics for a SOAP client.</p>
    
    <h2>Endpoints</h2>
    <div class="endpoint">
        <strong>GET /calculate?a={number}&b={number}&op={operation}</strong><br>
        Call SOAP calculator service with tracing and metrics<br>
        Operations: add, subtract, multiply, divide<br>
        <a href="/calculate?a=10&b=5&op=add">Try: 10 + 5</a> |
        <a href="/calculate?a=20&b=4&op=divide">Try: 20 / 4</a>
    </div>
    
    <div class="endpoint">
        <strong>GET /metrics</strong><br>
        Prometheus metrics endpoint<br>
        <a href="/metrics">View metrics</a>
    </div>
    
    <h2>Observability Stack</h2>
    <ul>
        <li><strong>Jaeger UI:</strong> <a href="http://localhost:16686" target="_blank">http://localhost:16686</a></li>
        <li><strong>Prometheus:</strong> <a href="http://localhost:9090" target="_blank">http://localhost:9090</a></li>
        <li><strong>Metrics Endpoint:</strong> <a href="http://localhost:3000/metrics" target="_blank">http://localhost:3000/metrics</a></li>
    </ul>
    
    <h2>Available Metrics</h2>
    <ul>
        <li><code>soap_requests_total</code> - Total SOAP requests (counter)</li>
        <li><code>soap_request_duration_seconds</code> - Request duration (histogram)</li>
        <li><code>soap_response_size_bytes</code> - Response size (histogram)</li>
        <li><code>soap_errors_total</code> - Total errors (counter)</li>
    </ul>
    
    <h2>Quick Test</h2>
    <p>Run this in your terminal to generate some traffic:</p>
    <pre>for i in {1..10}; do curl "http://localhost:3000/calculate?a=$i&b=2&op=multiply"; done</pre>
    
    <p><em>Then check Jaeger for traces and Prometheus for metrics!</em></p>
</body>
</html>
"#,
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔍 SOAP Client Observability Example\n");

    // Initialize Prometheus metrics
    println!("📊 Initializing Prometheus metrics exporter...");
    let prometheus_handle = init_metrics();

    // Initialize OpenTelemetry/Jaeger tracer
    println!("🔭 Initializing Jaeger tracer...");
    let tracer = init_tracer();

    // Initialize tracing subscriber
    println!("📝 Initializing tracing subscriber...");
    init_tracing(tracer);

    tracing::info!("Observability stack initialized");

    // Create SOAP client
    let endpoint = "http://www.dneonline.com/calculator.asmx";
    let calculator = Calculator::new(endpoint);
    tracing::info!(endpoint = %endpoint, "SOAP client created");

    // Test SOAP client once at startup
    println!("\n🧮 Testing SOAP client...");
    tracing::info!("Performing test calculation: 5 + 3");
    match calculator.add(Add { int_a: 5, int_b: 3 }).await {
        Ok(response) => {
            println!("✅ SOAP client working! 5 + 3 = {}", response.add_result);
            tracing::info!(result = response.add_result, "Test calculation successful");
        }
        Err(e) => {
            eprintln!("❌ SOAP client test failed: {}", e);
            tracing::error!(error = %e, "Test calculation failed");
            return Err(e.into());
        }
    }

    // Build Axum router with shared state
    #[derive(Clone)]
    struct AppState {
        calculator: Calculator,
        prometheus: PrometheusHandle,
    }

    let state = AppState {
        calculator: calculator.clone(),
        prometheus: prometheus_handle,
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .route(
            "/calculate",
            get(
                |State(state): State<AppState>, query: Query<CalculateParams>| async move {
                    calculate_handler(State(state.calculator), query).await
                },
            ),
        )
        .route(
            "/metrics",
            get(|State(state): State<AppState>| async move {
                metrics_handler(State(state.prometheus)).await
            }),
        )
        .with_state(state);

    // Start web server
    let addr = "0.0.0.0:3000";
    println!("\n🚀 Starting web server on {}", addr);
    println!("📍 Endpoints:");
    println!("   - Service:     http://localhost:3000/");
    println!("   - Calculate:   http://localhost:3000/calculate?a=10&b=5&op=add");
    println!("   - Metrics:     http://localhost:3000/metrics");
    println!("\n🔍 Observability:");
    println!("   - Jaeger UI:   http://localhost:16686");
    println!("   - Prometheus:  http://localhost:9090");
    println!("\nPress Ctrl+C to stop\n");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(addr = %addr, "Web server started");

    axum::serve(listener, app).await?;

    // Shutdown tracing
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
