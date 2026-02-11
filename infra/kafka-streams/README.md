# Kafka Streams Infrastructure

This folder contains configuration and code for Kafka streaming pipelines. Use for real-time market data ingestion, processing, and analytics.

## Example Topics
- market-data
- signals
- execution-orders

## Example Code (Node.js)
```js
const { Kafka } = require('kafkajs');
const kafka = new Kafka({ clientId: 'crypto-platform', brokers: ['localhost:9092'] });
const producer = kafka.producer();
async function sendMarketData(data) {
	await producer.connect();
	await producer.send({ topic: 'market-data', messages: [{ value: JSON.stringify(data) }] });
	await producer.disconnect();
}
```
# kafka-streams
Kafka Streams processing applications.
