# Try it out!

## Prerequisites
- Running UNIX os.
- Must have rust, docker, and docker-compose installed.

## Step-by-step
- [ ] Clone this repo, and cd into the directory.
- [ ] Create a `otel-collector-config.yaml` file and paste [this](#otel-collector-config).
- [ ] Run `docker compose up -d` to start the collector and the jaeger containers.
  - Check that the containers are running with `docker ps --all`.
  - Check logs with `docker logs <container-id>` if a container fails to start.
- [ ] Run `cargo run` to generate a trace.
  - Open jaeger in your browser with `localhost:16686` and select your service.
  - Check logs of collector to see the logger version of the trace.

## Otel Collector Config

```yml
receivers:
  otlp:
    protocols:
      http:
      grpc:

exporters:
  logging:
    loglevel: debug
    
  jaeger:
    endpoint: jaeger-all-in-one:14250
    tls:
      insecure: true
      
# If you have a honeycomb api key you can use this exporter. (add `otelp` to the exporters array below)
#  otlp:
#    endpoint: "api.honeycomb.io:443"
#    headers:
#      "x-honeycomb-team": "<your api key here>"

processors:
  batch:

extensions:
  health_check:
  pprof:
    endpoint: :1888
  zpages:
    endpoint: :55679

service:
  extensions: [pprof, zpages, health_check]
  pipelines:
    traces:
      receivers: [otlp]
      processors: [batch]
      exporters: [logging, jaeger] # add exporters to this array to enable them
    metrics:
      receivers: [otlp]
      processors: [batch]
      exporters: [logging]
```
