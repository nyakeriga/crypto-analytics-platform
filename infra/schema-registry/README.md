# Schema Registry

Schema Registry manages Avro/Protobuf schemas for streaming data. Ensures data consistency and compatibility across services.

## Example Schema (Avro)
```json
{
	"type": "record",
	"name": "MarketData",
	"fields": [
		{"name": "symbol", "type": "string"},
		{"name": "price", "type": "double"},
		{"name": "timestamp", "type": "long"}
	]
}
```
# schema-registry
Schema registry for data serialization.
