package com.crypto.analytics.streams;

import org.apache.kafka.common.serialization.Serdes;
import org.apache.kafka.streams.KafkaStreams;
import org.apache.kafka.streams.StreamsBuilder;
import org.apache.kafka.streams.StreamsConfig;
import org.apache.kafka.streams.kstream.KStream;
import org.apache.kafka.streams.kstream.KTable;
import org.apache.kafka.streams.kstream.Materialized;
import org.apache.kafka.streams.kstream.TimeWindows;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.time.Duration;
import java.util.Properties;
import java.util.logging.Logger;

public class MarketDataProcessor {
    private static final Logger logger = Logger.getLogger(MarketDataProcessor.class.getName());
    private static final ObjectMapper objectMapper = new ObjectMapper();

    public static void main(String[] args) {
        Properties props = new Properties();
        props.put(StreamsConfig.APPLICATION_ID_CONFIG, "crypto-market-data-processor");
        props.put(StreamsConfig.BOOTSTRAP_SERVERS_CONFIG, "localhost:9092");
        props.put(StreamsConfig.DEFAULT_KEY_SERDE_CLASS_CONFIG, Serdes.String().getClass());
        props.put(StreamsConfig.DEFAULT_VALUE_SERDE_CLASS_CONFIG, Serdes.String().getClass());

        StreamsBuilder builder = new StreamsBuilder();

        // Input stream from market data topic
        KStream<String, String> marketDataStream = builder.stream("market-data-raw");

        // Process and aggregate market data
        KStream<String, String> processedData = marketDataStream
            .mapValues(MarketDataProcessor::parseAndEnrichMarketData)
            .filter((key, value) -> value != null);

        // Create aggregated views
        KTable<String, String> priceAggregates = processedData
            .groupByKey()
            .windowedBy(TimeWindows.ofSizeWithNoGrace(Duration.ofMinutes(1)))
            .aggregate(
                () -> "{\"count\": 0, \"sum\": 0.0, \"avg\": 0.0}",
                (key, value, aggregate) -> aggregatePrice(key, value, aggregate),
                Materialized.with(Serdes.String(), Serdes.String())
            )
            .toStream()
            .map((windowedKey, value) -> new org.apache.kafka.streams.KeyValue<>(windowedKey.key(), value));

        // Output to processed topics
        processedData.to("market-data-processed");
        priceAggregates.to("market-data-aggregates");

        KafkaStreams streams = new KafkaStreams(builder.build(), props);

        // Add shutdown hook
        Runtime.getRuntime().addShutdownHook(new Thread(streams::close));

        try {
            streams.start();
            logger.info("Market Data Processor started");
        } catch (Exception e) {
            logger.severe("Error starting streams: " + e.getMessage());
            System.exit(1);
        }
    }

    private static String parseAndEnrichMarketData(String value) {
        try {
            JsonNode data = objectMapper.readTree(value);
            // Add processing logic here (e.g., calculate VWAP, volume analysis)
            data = enrichWithTechnicalIndicators(data);
            return objectMapper.writeValueAsString(data);
        } catch (Exception e) {
            logger.warning("Error parsing market data: " + e.getMessage());
            return null;
        }
    }

    private static JsonNode enrichWithTechnicalIndicators(JsonNode data) {
        // Simplified enrichment - in real implementation, calculate RSI, MACD, etc.
        // For demo, just add a timestamp if missing
        if (!data.has("processed_at")) {
            ((com.fasterxml.jackson.databind.node.ObjectNode) data).put("processed_at", System.currentTimeMillis());
        }
        return data;
    }

    private static String aggregatePrice(String key, String value, String aggregate) {
        try {
            JsonNode current = objectMapper.readTree(value);
            JsonNode agg = objectMapper.readTree(aggregate);

            double price = current.get("price").asDouble();
            int count = agg.get("count").asInt() + 1;
            double sum = agg.get("sum").asDouble() + price;
            double avg = sum / count;

            com.fasterxml.jackson.databind.node.ObjectNode newAgg = (com.fasterxml.jackson.databind.node.ObjectNode) agg;
            newAgg.put("count", count);
            newAgg.put("sum", sum);
            newAgg.put("avg", avg);

            return objectMapper.writeValueAsString(newAgg);
        } catch (Exception e) {
            logger.warning("Error aggregating price: " + e.getMessage());
            return aggregate;
        }
    }
}