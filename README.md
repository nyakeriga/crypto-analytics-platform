# Crypto Analytics Platform

## Enterprise-Grade Real-Time Cryptocurrency Analytics and Signal Generation Platform

The Crypto Analytics Platform is a high-performance, scalable, and AI-powered system designed for comprehensive cryptocurrency market analysis, real-time signal generation, and advanced trading insights. Built with enterprise-grade architecture, it processes vast amounts of market data to deliver actionable trading signals through sophisticated technical analysis, machine learning models, and market structure analysis.

## Table of Contents

- [Project Description](#project-description)
- [System Functions](#system-functions)
- [Architecture Overview](#architecture-overview)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Environment Setup](#environment-setup)
- [Setup Instructions](#setup-instructions)
- [Running Instructions](#running-instructions)
- [Service Startup Order](#service-startup-order)
- [API Documentation](#api-documentation)
- [Monitoring Setup](#monitoring-setup)
- [Troubleshooting](#troubleshooting)
- [Deployment](#deployment)
- [Usage](#usage)

## Project Description

The Crypto Analytics Platform provides institutional-grade cryptocurrency analytics with the following key capabilities:

- **Real-time Market Data Ingestion**: Aggregates live market data from multiple exchanges (Binance, Bybit, OKX) via WebSocket connections
- **Advanced Technical Analysis**: Calculates 20+ technical indicators including RSI, MACD, EMA, VWAP, Bollinger Bands, and custom indicators
- **Market Structure Analysis**: Identifies break of structure (BOS), trend analysis, and order block detection
- **AI-Powered Signal Generation**: Machine learning models for predictive analytics and signal enhancement
- **Backtesting Engine**: Historical performance simulation with comprehensive metrics
- **Risk Management**: Real-time risk assessment and position sizing calculations
- **Multi-tenant Architecture**: User management, subscriptions, and role-based access control
- **High-Performance Processing**: Rust-based computation layer for sub-millisecond latency
- **Enterprise Monitoring**: Comprehensive observability with Prometheus, Grafana, and ELK stack

## System Functions

### Core Analytics Functions
- **Market Data Processing**: Real-time OHLCV data aggregation and normalization
- **Indicator Calculation**: Parallel computation of technical indicators across multiple timeframes
- **Signal Generation**: Multi-factor signal scoring with confidence levels
- **Portfolio Optimization**: Risk-adjusted position sizing and diversification algorithms
- **Performance Analytics**: Real-time P&L tracking and risk metrics

### AI/ML Functions
- **Predictive Modeling**: Time series forecasting for price movements
- **Pattern Recognition**: Machine learning-based chart pattern detection
- **Sentiment Analysis**: Social media and news sentiment integration
- **Reinforcement Learning**: Adaptive trading strategy optimization

### Operational Functions
- **User Management**: Authentication, authorization, and subscription management
- **Data Persistence**: Multi-tier storage with PostgreSQL, TimescaleDB, and ClickHouse
- **Event Streaming**: Kafka-based event-driven architecture for real-time processing
- **Caching Layer**: Redis-based caching for low-latency data access

## Architecture Overview

The platform follows a layered microservices architecture designed for scalability and performance:

### Frontend Layer
- **Technology**: TypeScript with Next.js
- **Components**: Web dashboard and mobile application
- **Features**: Real-time data visualization, WebSocket connections, interactive charts

### API Gateway Layer
- **Technology**: C# with ASP.NET Core and YARP
- **Functions**: Request routing, JWT authentication, rate limiting, load balancing

### Backend Services Layer
- **Technology**: C# microservices with ASP.NET Core
- **Services**:
  - **User Service**: User profile and subscription management
  - **Signal Service**: Signal processing and delivery
  - **Auth Service**: Authentication and authorization
  - **Subscription Service**: Billing and plan management
  - **Admin Service**: Administrative operations

### Computation Layer
- **Technology**: Rust microservices
- **Services**:
  - **Market Ingestion**: Real-time data collection from exchanges
  - **Indicator Engine**: Technical indicator calculations
  - **Structure Engine**: Market structure analysis
  - **Signal Engine**: Signal generation and scoring
  - **Risk Engine**: Risk assessment and management
  - **Backtest Engine**: Historical performance simulation

### AI Engine Layer
- **Technology**: Python with FastAPI
- **Components**:
  - Feature engineering pipeline
  - Model training and evaluation
  - Model registry and versioning
  - Reinforcement learning agents

### Data Layer
- **PostgreSQL**: Relational data (users, signals, subscriptions)
- **TimescaleDB**: Time-series OHLCV data with hypertables
- **ClickHouse**: Analytical queries on aggregated data
- **Redis**: High-performance caching and session storage
- **Kafka**: Event streaming and message queuing

### Infrastructure Layer
- **Containerization**: Docker for service packaging
- **Orchestration**: Kubernetes for deployment and scaling
- **Infrastructure as Code**: Terraform for cloud provisioning
- **Monitoring**: Prometheus, Grafana, ELK stack
- **CI/CD**: GitHub Actions for automated deployment

## Tech Stack

### Backend & APIs
- **C# .NET 8**: ASP.NET Core Web APIs, Entity Framework Core
- **Rust 1.70+**: High-performance computation services
- **Python 3.11**: AI/ML services with FastAPI

### Data & Messaging
- **PostgreSQL 15**: Primary relational database
- **TimescaleDB**: Time-series data storage
- **ClickHouse**: Analytical database
- **Redis 7**: Caching and session storage
- **Apache Kafka 3.6**: Event streaming platform

### Infrastructure & DevOps
- **Docker**: Containerization
- **Kubernetes**: Container orchestration
- **Terraform**: Infrastructure provisioning
- **Prometheus**: Metrics collection
- **Grafana**: Visualization and dashboards
- **ELK Stack**: Logging and analysis

### Frontend
- **TypeScript**: Type-safe JavaScript
- **Next.js**: React framework for web applications
- **WebSocket**: Real-time data streaming

### Development Tools
- **Git**: Version control
- **GitHub Actions**: CI/CD pipelines
- **Pre-commit hooks**: Code quality enforcement
- **Docker Compose**: Local development environment

## Prerequisites

### System Requirements
- **Operating System**: Linux (Ubuntu 20.04+), macOS (12+), or Windows 10/11 with WSL2
- **CPU**: 4+ cores (8+ recommended for development)
- **RAM**: 16GB minimum (32GB recommended)
- **Storage**: 100GB SSD storage
- **Network**: Stable internet connection for market data feeds

### Software Dependencies
- **Docker**: 24.0+ with Docker Compose
- **Docker Compose**: v2.0+
- **Git**: 2.30+
- **.NET SDK**: 8.0+
- **Rust**: 1.70+
- **Python**: 3.11+
- **Node.js**: 18+ (for frontend development)
- **kubectl**: For Kubernetes deployments
- **Terraform**: 1.5+ (for infrastructure provisioning)

### External Services
- **Kafka Cluster**: For event streaming
- **PostgreSQL Database**: For relational data
- **TimescaleDB**: For time-series data
- **Redis Cluster**: For caching
- **Exchange API Keys**: Binance, Bybit, OKX for market data

## Environment Setup

### 1. Clone the Repository
```bash
git clone https://github.com/your-org/crypto-analytics-platform.git
cd crypto-analytics-platform
```

### 2. Environment Configuration
Create environment files for each service:

```bash
# Copy environment templates
cp backend/api-gateway/appsettings.json.example backend/api-gateway/appsettings.json
cp backend/signal-service/appsettings.json.example backend/signal-service/appsettings.json
cp core-engine/market-ingestion/.env.example core-engine/market-ingestion/.env
```

### 3. Configure API Keys
Update the following files with your API credentials:

- `backend/api-gateway/appsettings.json`: Database connections, JWT secrets
- `core-engine/market-ingestion/.env`: Exchange API keys and Kafka brokers
- `monitoring/prometheus/prometheus.yml`: Service endpoints

### 4. Database Setup
```bash
# Start databases
docker-compose -f docker-compose.infrastructure.yml up -d

# Run migrations
cd backend/signal-service
dotnet ef database update

cd ../user-service
dotnet ef database update
```

### 5. Kafka Setup
```bash
# Start Kafka cluster
docker-compose -f docker-compose.kafka.yml up -d

# Create topics
docker exec kafka kafka-topics --create --topic market-ohlcv --bootstrap-server localhost:9092
docker exec kafka kafka-topics --create --topic indicators --bootstrap-server localhost:9092
docker exec kafka kafka-topics --create --topic signals --bootstrap-server localhost:9092
```

## Setup Instructions

### Local Development Setup

1. **Install Dependencies**
```bash
# Install .NET dependencies
cd backend
dotnet restore

# Install Rust dependencies
cd ../core-engine
cargo build

# Install Python dependencies
cd ../mlops/model-serving
pip install -r requirements.txt
```

2. **Build Services**
```bash
# Build .NET services
cd backend
dotnet build

# Build Rust services
cd ../core-engine
cargo build --release

# Build Docker images
docker-compose build
```

3. **Database Initialization**
```bash
# Run database migrations
cd backend/signal-service
dotnet ef database update

# Seed initial data
dotnet run -- seed
```

### Production Setup

1. **Infrastructure Provisioning**
```bash
cd infrastructure/terraform
terraform init
terraform plan
terraform apply
```

2. **Kubernetes Deployment**
```bash
cd k8s
kubectl apply -f namespaces/
kubectl apply -f configmaps/
kubectl apply -f secrets/
kubectl apply -f deployments/
kubectl apply -f services/
```

3. **Monitoring Setup**
```bash
cd monitoring
kubectl apply -f prometheus/
kubectl apply -f grafana/
```

## Running Instructions

### Local Development

Start all services using Docker Compose:
```bash
docker-compose up -d
```

Individual service startup:
```bash
# Start databases and infrastructure
docker-compose -f docker-compose.infrastructure.yml up -d

# Start backend services
cd backend/api-gateway
dotnet run

cd ../signal-service
dotnet run

# Start computation services
cd ../../core-engine/market-ingestion
cargo run

cd ../indicator-engine
cargo run

# Start AI services
cd ../../mlops/model-serving
uvicorn main:app --reload
```

### Production Deployment

```bash
# Deploy to Kubernetes
kubectl apply -f k8s/

# Check deployment status
kubectl get pods -n crypto-analytics
kubectl get services -n crypto-analytics

# Monitor logs
kubectl logs -f deployment/api-gateway -n crypto-analytics
```

## Service Startup Order

For proper initialization, start services in the following order:

1. **Infrastructure Services**
   - PostgreSQL, TimescaleDB, ClickHouse
   - Redis cluster
   - Kafka cluster
   - Prometheus, Grafana

2. **Data Layer Services**
   - Database migrations
   - Cache warming
   - Initial data seeding

3. **Computation Layer**
   - Market Ingestion Engine
   - Indicator Engine
   - Structure Engine
   - Signal Engine

4. **Backend Services**
   - User Service
   - Signal Service
   - Auth Service
   - Subscription Service

5. **API Gateway**
   - Route configuration
   - Authentication setup

6. **AI Services**
   - Model Serving
   - Feature Engineering

7. **Frontend**
   - Web application
   - Mobile application

## API Documentation

### REST APIs

#### User Service API
- **Base URL**: `http://localhost:5001/api/users`
- **Authentication**: JWT Bearer token

**Endpoints**:
- `GET /api/users/{id}` - Get user profile
- `POST /api/users` - Create user
- `PUT /api/users/{id}` - Update user
- `DELETE /api/users/{id}` - Delete user

#### Signal Service API
- **Base URL**: `http://localhost:5002/api/signals`
- **Authentication**: JWT Bearer token

**Endpoints**:
- `GET /api/signals` - Get user signals
- `POST /api/signals` - Create signal
- `GET /api/signals/{id}` - Get signal details
- `DELETE /api/signals/{id}` - Delete signal

### WebSocket APIs

#### Real-time Data Stream
- **URL**: `ws://localhost:5000/ws/market-data`
- **Protocol**: JSON-RPC 2.0

**Subscriptions**:
- `ohlcv.{symbol}.{interval}` - OHLCV data
- `indicators.{symbol}` - Technical indicators
- `signals.{userId}` - User signals

### GraphQL API (Future)

The platform includes a GraphQL gateway for flexible data querying:
- **URL**: `http://localhost:5000/graphql`
- **Schema**: Auto-generated from service schemas

## Monitoring Setup

### Prometheus Configuration

The platform uses Prometheus for metrics collection with the following key metrics:

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'api-gateway'
    static_configs:
      - targets: ['api-gateway:80']
    metrics_path: '/metrics'

  - job_name: 'signal-service'
    static_configs:
      - targets: ['signal-service:80']
    metrics_path: '/metrics'

  - job_name: 'market-ingestion'
    static_configs:
      - targets: ['market-ingestion:8080']
    metrics_path: '/metrics'
```

### Grafana Dashboards

Pre-configured dashboards include:
- **System Health**: CPU, memory, disk usage
- **Application Metrics**: Request rates, error rates, latency
- **Business Metrics**: Signal accuracy, user engagement
- **Market Data**: Data ingestion rates, processing latency

### Alerting Rules

Critical alerts configured:
- Service downtime
- High error rates (>5%)
- Data ingestion delays
- Database connection issues
- Memory/CPU thresholds

## Troubleshooting

### Common Issues

#### Service Startup Failures

**Database Connection Issues**:
```bash
# Check database connectivity
docker exec -it postgres pg_isready -h localhost -p 5432

# Verify connection strings in appsettings.json
cat backend/signal-service/appsettings.json | grep ConnectionString
```

**Kafka Connection Issues**:
```bash
# Check Kafka cluster status
docker exec kafka kafka-cluster cluster-id --bootstrap-server localhost:9092

# Verify topic creation
docker exec kafka kafka-topics --list --bootstrap-server localhost:9092
```

#### Performance Issues

**High Memory Usage**:
- Monitor Rust services for memory leaks
- Adjust Docker memory limits
- Implement connection pooling

**Slow Queries**:
```sql
-- Enable query logging in PostgreSQL
ALTER DATABASE crypto_analytics SET log_statement = 'all';
ALTER DATABASE crypto_analytics SET log_duration = 'on';
```

#### Data Ingestion Issues

**Missing Market Data**:
- Verify exchange API keys
- Check WebSocket connections
- Monitor rate limits

**Data Quality Issues**:
- Validate OHLCV data integrity
- Check timestamp synchronization
- Monitor data gaps

### Logs and Debugging

**Centralized Logging**:
```bash
# View service logs
docker-compose logs -f api-gateway

# Kubernetes logs
kubectl logs -f deployment/signal-service -n crypto-analytics
```

**Debug Mode**:
```bash
# Enable debug logging
export ASPNETCORE_ENVIRONMENT=Development
export RUST_LOG=debug

# Run with debugging
dotnet run --debug
cargo run -- --debug
```

### Health Checks

**Service Health Endpoints**:
- API Gateway: `GET /health`
- Signal Service: `GET /api/health`
- Market Ingestion: `GET /health`

**Database Health**:
```bash
# PostgreSQL health check
psql -h localhost -U postgres -d crypto_analytics -c "SELECT 1"
```

## Deployment

### Docker Deployment

```bash
# Build and deploy
docker-compose -f docker-compose.prod.yml up -d

# Scale services
docker-compose up -d --scale market-ingestion=3
```

### Kubernetes Deployment

```yaml
# k8s/deployment.yml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api-gateway
  template:
    metadata:
      labels:
        app: api-gateway
    spec:
      containers:
      - name: api-gateway
        image: crypto-analytics/api-gateway:latest
        ports:
        - containerPort: 80
        env:
        - name: ASPNETCORE_ENVIRONMENT
          value: Production
```

### Cloud Deployment

**AWS Deployment**:
```bash
# Using Terraform
cd infrastructure/terraform/aws
terraform apply

# Deploy to EKS
kubectl apply -f k8s/
```

**Azure Deployment**:
```bash
# Using Terraform
cd infrastructure/terraform/azure
terraform apply

# Deploy to AKS
kubectl apply -f k8s/
```

## Usage

### Getting Started

1. **User Registration**:
```bash
curl -X POST http://localhost:5001/api/users \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"securepassword"}'
```

2. **Authentication**:
```bash
curl -X POST http://localhost:5001/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"securepassword"}'
```

3. **Subscribe to Signals**:
```bash
curl -X POST http://localhost:5002/api/signals/subscribe \
  -H "Authorization: Bearer {token}" \
  -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","type":"technical"}'
```

### WebSocket Usage

```javascript
// Connect to WebSocket
const ws = new WebSocket('ws://localhost:5000/ws/market-data');

// Subscribe to BTC/USDT data
ws.send(JSON.stringify({
  jsonrpc: '2.0',
  method: 'subscribe',
  params: ['ohlcv.BTCUSDT.1m'],
  id: 1
}));

// Handle incoming data
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Received:', data);
};
```

### API Usage Examples

**Fetch Technical Indicators**:
```python
import requests

headers = {'Authorization': 'Bearer {token}'}
response = requests.get('http://localhost:5002/api/indicators/BTCUSDT', headers=headers)
indicators = response.json()
```

**Backtest Strategy**:
```python
import requests

payload = {
  'strategy': 'rsi_divergence',
  'symbol': 'BTCUSDT',
  'start_date': '2023-01-01',
  'end_date': '2023-12-31',
  'initial_capital': 10000
}

response = requests.post('http://localhost:5003/api/backtest', json=payload, headers=headers)
results = response.json()
```

### Advanced Usage

**Custom Indicator Development**:
```rust
// core-engine/indicator-engine/src/custom_indicator.rs
pub struct CustomIndicator {
    period: usize,
    values: Vec<f64>,
}

impl CustomIndicator {
    pub fn new(period: usize) -> Self {
        CustomIndicator {
            period,
            values: Vec::new(),
        }
    }

    pub fn next(&mut self, value: f64) -> Option<f64> {
        self.values.push(value);
        if self.values.len() >= self.period {
            // Calculate custom indicator logic
            Some(self.calculate_indicator())
        } else {
            None
        }
    }

    fn calculate_indicator(&self) -> f64 {
        // Implementation here
        0.0
    }
}
```

**ML Model Integration**:
```python
# mlops/model-serving/app.py
from fastapi import FastAPI
from sklearn.ensemble import RandomForestClassifier
import joblib

app = FastAPI()
model = joblib.load('models/signal_predictor.pkl')

@app.post('/predict')
async def predict_signal(features: dict):
    prediction = model.predict_proba([list(features.values())])
    return {'signal': prediction[0][1] > 0.7}
```

This comprehensive README provides all necessary information for understanding, setting up, deploying, and using the Crypto Analytics Platform. For additional support, please refer to the documentation or contact the development team.