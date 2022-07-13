# SOAP Client Observability Example

This example demonstrates full observability stack integration for the SOAP client:

- **Distributed Tracing** with OpenTelemetry/Jaeger
- **Metrics Collection** with Prometheus
- **Visualization** with Grafana (optional)
- **JSON Logging** for log aggregation
- **Web Service** with `/metrics` endpoint

---

## 🎯 Features

### Distributed Tracing
- OpenTelemetry integration with Jaeger exporter
- Hierarchical spans for SOAP operations
- Request flow visualization
- Performance bottleneck identification

### Metrics
- **Request Duration**: Histogram of SOAP call latency
- **Request Count**: Total requests by operation and status
- **Error Count**: Failed requests by operation
- **Response Size**: Distribution of response sizes

### Web Service
- **`/`** - Info page with links and documentation
- **`/calculate`** - Calculator endpoint that calls SOAP service
- **`/metrics`** - Prometheus metrics endpoint

---

## 🚀 Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (latest stable)

### Step 1: Start Observability Stack

```bash
cd examples/observability
docker-compose up -d
```

This starts:
- **Jaeger** on http://localhost:16686
- **Prometheus** on http://localhost:9090
- **Grafana** on http://localhost:3001 (admin/admin)

### Step 2: Run the Example

```bash
cargo run
```

The service starts on http://localhost:3000

### Step 3: Generate Traffic

```bash
# Single calculation
curl "http://localhost:3000/calculate?a=10&b=5&op=add"

# Generate multiple requests
for i in {1..20}; do
  curl "http://localhost:3000/calculate?a=$i&b=2&op=multiply"
  sleep 0.5
done
```

### Step 4: View Results

1. **Jaeger UI**: http://localhost:16686
   - Select service: `soap-client-observability-example`
   - View traces and spans
   - Analyze request flow and duration

2. **Prometheus**: http://localhost:9090
   - Query: `soap_requests_total`
   - Query: `rate(soap_request_duration_seconds_sum[5m])`
   - View metrics and graphs

3. **Metrics Endpoint**: http://localhost:3000/metrics
   - View raw Prometheus metrics
   - See all SOAP client metrics

4. **Grafana** (optional): http://localhost:3001
   - Login: admin/admin
   - Create dashboards for SOAP metrics
   - Visualize request rates, latencies, errors

---

## 📊 Available Metrics

### Counters

- **`soap_requests_total{operation, endpoint, status}`**
  - Total SOAP requests
  - Labels: operation name, endpoint URL, success/error status

- **`soap_errors_total{operation, endpoint}`**
  - Total failed SOAP requests
  - Labels: operation name, endpoint URL

### Histograms

- **`soap_request_duration_seconds{operation, endpoint}`**
  - Request duration in seconds
  - Buckets: 0.005s to 10s
  - Labels: operation name, endpoint URL

- **`soap_response_size_bytes{operation, endpoint}`**
  - Response body size in bytes
  - Buckets: 100 to 100,000 bytes
  - Labels: operation name, endpoint URL

---

## 🔍 Tracing Spans

The example creates hierarchical spans:

```
calculate_endpoint (HTTP handler)
  └─ soap_operation (operation="Add", service="Calculator")
      └─ call_with_soap_action (endpoint="...", soap_version=Soap11)
          ├─ Building SOAP envelope
          ├─ HTTP POST request
          └─ Parsing response
```

Each span includes:
- **Duration**: How long each step took
- **Attributes**: Operation name, endpoint, SOAP version
- **Events**: Key milestones (envelope built, request sent, etc.)

---

## 🧪 Example Queries

### Prometheus Queries

```promql
# Total requests per second
rate(soap_requests_total[1m])

# Average request duration
rate(soap_request_duration_seconds_sum[5m]) / rate(soap_request_duration_seconds_count[5m])

# 95th percentile latency
histogram_quantile(0.95, rate(soap_request_duration_seconds_bucket[5m]))

# Error rate
rate(soap_errors_total[5m]) / rate(soap_requests_total[5m])

# Requests by operation
sum by (operation) (rate(soap_requests_total[5m]))

# Response size average
rate(soap_response_size_bytes_sum[5m]) / rate(soap_response_size_bytes_count[5m])
```

### Jaeger Queries

- **Service**: `soap-client-observability-example`
- **Operation**: `calculate_endpoint` or `soap_operation`
- **Tags**: `operation=Add`, `endpoint=http://...`
- **Lookback**: 1h, 6h, 24h

---

## 🎨 JSON Logging

Enable JSON logging for log aggregation systems:

```bash
RUST_LOG_JSON=1 cargo run
```

Output format:
```json
{
  "timestamp": "2024-12-17T10:30:45.123Z",
  "level": "INFO",
  "target": "soap_client_runtime::client",
  "span": {
    "operation": "Add",
    "service": "Calculator"
  },
  "fields": {
    "message": "Sending HTTP POST request",
    "endpoint": "http://www.dneonline.com/calculator.asmx"
  }
}
```

---

## 🔧 Configuration

### Environment Variables

- **`RUST_LOG`**: Log level (default: `info`)
  - `RUST_LOG=debug` - Detailed logs
  - `RUST_LOG=trace` - Very verbose
  - `RUST_LOG=soap_client_runtime=debug` - Filter by module

- **`RUST_LOG_JSON`**: Enable JSON logging
  - `RUST_LOG_JSON=1` - JSON format
  - Not set - Human-readable format

### Cargo Features

```toml
[features]
default = ["tracing", "metrics"]
tracing = ["soapus-runtime/tracing"]
metrics = ["soapus-runtime/metrics"]
```

Disable metrics:
```bash
cargo run --no-default-features --features tracing
```

---

## 🐳 Docker Compose Services

### Jaeger (All-in-One)

- **UI**: http://localhost:16686
- **Collector HTTP**: 14268
- **Collector gRPC**: 14250
- **Agent UDP**: 6831
- **Health**: http://localhost:14269

### Prometheus

- **UI**: http://localhost:9090
- **Config**: `prometheus.yml`
- **Scrape interval**: 5s
- **Targets**: Host application on port 3000

### Grafana (Optional)

- **UI**: http://localhost:3001
- **Login**: admin/admin
- **Datasources**: Prometheus + Jaeger (pre-configured)

---

## 📈 Performance

### Overhead

- **Tracing**: ~1-2% latency overhead
- **Metrics**: <1% overhead
- **Combined**: ~2-3% total overhead

### Optimization

- Adjust sampling rate in production:
  ```rust
  .with_sampler(Sampler::TraceIdRatioBased(0.1)) // 10% sampling
  ```

- Use asynchronous metric recording (already enabled)
- Buffer traces before export (already enabled)

---

## 🛠️ Troubleshooting

### Jaeger Not Receiving Traces

1. Check Jaeger is running:
   ```bash
   docker-compose ps jaeger
   ```

2. Verify agent port is open:
   ```bash
   nc -zv localhost 6831
   ```

3. Check application logs for export errors

### Prometheus Not Scraping

1. Check Prometheus targets:
   http://localhost:9090/targets

2. Ensure application is running on port 3000

3. Verify metrics endpoint:
   ```bash
   curl http://localhost:3000/metrics
   ```

### No Metrics Appearing

1. Generate some traffic:
   ```bash
   curl "http://localhost:3000/calculate?a=1&b=2&op=add"
   ```

2. Check `/metrics` endpoint shows `soap_*` metrics

3. Verify Prometheus scrape config includes host.docker.internal

### Grafana Can't Connect to Datasources

1. Check Grafana logs:
   ```bash
   docker-compose logs grafana
   ```

2. Verify datasources configuration:
   http://localhost:3001/datasources

3. Use container names (not localhost) in datasource URLs

---

## 🎓 Learning Resources

### OpenTelemetry
- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust)
- [OpenTelemetry Specification](https://opentelemetry.io/docs/)

### Jaeger
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [Jaeger UI Guide](https://www.jaegertracing.io/docs/1.21/frontend-ui/)

### Prometheus
- [Prometheus Querying](https://prometheus.io/docs/prometheus/latest/querying/basics/)
- [Histogram Metrics](https://prometheus.io/docs/practices/histograms/)

### Grafana
- [Grafana Dashboards](https://grafana.com/docs/grafana/latest/dashboards/)
- [Prometheus in Grafana](https://grafana.com/docs/grafana/latest/datasources/prometheus/)

---

## 📝 Example Scenarios

### Scenario 1: Find Slow Requests

1. Generate traffic:
   ```bash
   for i in {1..50}; do curl "http://localhost:3000/calculate?a=$i&b=2&op=divide"; done
   ```

2. Open Jaeger UI
3. Sort traces by duration
4. Click on slowest trace to see span breakdown

### Scenario 2: Monitor Error Rate

1. Open Prometheus UI
2. Query:
   ```promql
   rate(soap_errors_total[5m]) / rate(soap_requests_total[5m])
   ```
3. Set alert if error rate > 5%

### Scenario 3: Track Request Volume

1. Open Prometheus UI
2. Query:
   ```promql
   sum by (operation) (rate(soap_requests_total[1m]))
   ```
3. See requests/sec by operation type

### Scenario 4: Analyze Response Sizes

1. Open Prometheus UI
2. Query:
   ```promql
   histogram_quantile(0.99, rate(soap_response_size_bytes_bucket[5m]))
   ```
3. See 99th percentile response size

---

## 🚀 Next Steps

1. **Custom Dashboards**: Create Grafana dashboards for your metrics
2. **Alerting**: Set up Prometheus alerts for errors and latency
3. **Production**: Add authentication, TLS, rate limiting
4. **Sampling**: Implement trace sampling for high-volume services
5. **Log Aggregation**: Send JSON logs to ELK/Loki/Splunk

---

## 📦 Cleanup

Stop and remove containers:

```bash
docker-compose down
```

Remove volumes (deletes all data):

```bash
docker-compose down -v
```

---

## 🤝 Contributing

This example demonstrates best practices for SOAP client observability. Contributions welcome:

- Additional dashboards
- Alert rule examples
- Performance optimizations
- Documentation improvements

---

**Enjoy full observability for your SOAP clients! 🎉**