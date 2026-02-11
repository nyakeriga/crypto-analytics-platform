# Monitoring

System monitoring with Prometheus, Grafana, Loki.

## Example Prometheus Config
```yaml
scrape_configs:
  - job_name: 'crypto-platform'
    static_configs:
      - targets: ['localhost:9100']
```
