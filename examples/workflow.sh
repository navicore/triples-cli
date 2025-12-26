#!/bin/bash
# Example Workflow: CSV to Knowledge Graph Query
# ===============================================
# This script demonstrates a complete data pipeline:
# 1. Convert CSV to TTL
# 2. Merge with existing ontology
# 3. Query the combined graph

set -e
cd "$(dirname "$0")"

CLI="../target/release/triples-cli"

echo "Workflow: Build and Query a Knowledge Graph"
echo "============================================"
echo ""

# Step 1: Convert CSV data to TTL and merge with org structure
echo "Step 1: Building knowledge graph from CSV + TTL files..."
{
    cat people.csv | $CLI csv2ttl --base "http://example.org/people#" -s 0
    cat org.ttl
    cat relationships.ttl
} | $CLI merge > /tmp/knowledge-graph.ttl

echo "Created knowledge graph with $(cat /tmp/knowledge-graph.ttl | $CLI query -q 'SELECT * WHERE { ?s ?p ?o }' -f csv | wc -l) triples"
echo ""

# Step 2: Run some analytical queries
echo "Step 2: Analyzing the graph..."
echo ""

echo "All Engineers:"
cat /tmp/knowledge-graph.ttl | $CLI query -q '
SELECT ?person ?name WHERE {
    ?person <http://example.org/people#role> "Engineer" .
    ?person <http://example.org/people#name> ?name
}'
echo ""

echo "Department budgets:"
cat /tmp/knowledge-graph.ttl | $CLI query -q '
SELECT ?name ?budget WHERE {
    ?dept <http://example.org/org#name> ?name .
    ?dept <http://example.org/org#budget> ?budget
}'
echo ""

echo "Reporting structure:"
cat /tmp/knowledge-graph.ttl | $CLI query -q '
SELECT ?employee ?manager WHERE {
    ?e <http://example.org/org#reportsTo> ?m .
    ?e <http://example.org/people#name> ?employee .
    ?m <http://example.org/people#name> ?manager
}'
echo ""

# Step 3: Export results
echo "Step 3: Exporting query results to CSV..."
cat /tmp/knowledge-graph.ttl | $CLI query -q '
SELECT ?name ?email ?dept WHERE {
    ?person <http://example.org/people#name> ?name .
    ?person <http://example.org/people#email> ?email .
    ?person <http://example.org/org#worksIn> ?d .
    ?d <http://example.org/org#name> ?dept
}' -f csv > /tmp/employee-report.csv

echo "Exported to /tmp/employee-report.csv:"
cat /tmp/employee-report.csv
echo ""

# Cleanup
rm -f /tmp/knowledge-graph.ttl /tmp/employee-report.csv

echo "Workflow complete!"
