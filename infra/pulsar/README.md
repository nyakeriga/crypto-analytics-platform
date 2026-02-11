# Apache Pulsar Infrastructure

Pulsar is used for scalable, multi-tenant messaging and streaming. Use for decoupled microservices and real-time event distribution.

## Example Producer (Python)
```python
import pulsar
client = pulsar.Client('pulsar://localhost:6650')
producer = client.create_producer('market-data')
producer.send(('BTCUSDT:price:68700').encode('utf-8'))
client.close()
```
# pulsar
Apache Pulsar messaging system configurations.
