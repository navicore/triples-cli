#!/bin/bash
# triples-cli Tutorial
# ====================
# This script demonstrates the four main commands of triples-cli.
# Run from the examples directory, or adjust paths accordingly.

set -e
cd "$(dirname "$0")"

CLI="../target/release/triples-cli"

# Check if binary exists
if [ ! -f "$CLI" ]; then
    echo "Building triples-cli first..."
    (cd .. && cargo build --release)
fi

echo "============================================"
echo "triples-cli Tutorial"
echo "============================================"
echo ""

# -----------------------------------------------------------------------------
echo "1. CSV to Turtle (csv2ttl)"
echo "--------------------------------------------"
echo "Convert a CSV file to RDF Turtle format."
echo ""
echo "Input (people.csv):"
cat people.csv
echo ""
echo "Command: cat people.csv | triples-cli csv2ttl --base http://example.org/people# -s 0"
echo ""
echo "Output:"
cat people.csv | $CLI csv2ttl --base "http://example.org/people#" -s 0
echo ""

# -----------------------------------------------------------------------------
echo "2. Merge Turtle Files (merge)"
echo "--------------------------------------------"
echo "Combine multiple TTL files into one graph."
echo ""
echo "Command: cat org.ttl relationships.ttl | triples-cli merge"
echo ""
echo "Output:"
cat org.ttl relationships.ttl | $CLI merge
echo ""

# -----------------------------------------------------------------------------
echo "3. SPARQL Query (query)"
echo "--------------------------------------------"
echo "Run SPARQL queries against TTL data."
echo ""

# First, create a combined dataset
echo "Creating combined dataset..."
cat org.ttl relationships.ttl > /tmp/combined.ttl
cat people.csv | $CLI csv2ttl --base "http://example.org/people#" -s 0 >> /tmp/combined.ttl

echo ""
echo "Query 1: List all triples (LIMIT 10)"
echo "Command: cat /tmp/combined.ttl | triples-cli query -q 'SELECT * WHERE { ?s ?p ?o } LIMIT 10'"
echo ""
cat /tmp/combined.ttl | $CLI query -q "SELECT * WHERE { ?s ?p ?o } LIMIT 10"
echo ""

echo "Query 2: Find all departments"
echo "Command: cat org.ttl | triples-cli query -q 'SELECT ?dept ?name WHERE { ?dept <http://example.org/org#name> ?name }'"
echo ""
cat org.ttl | $CLI query -q "SELECT ?dept ?name WHERE { ?dept <http://example.org/org#name> ?name }"
echo ""

echo "Query 3: Who reports to whom?"
echo "Command: cat relationships.ttl | triples-cli query -q 'SELECT ?person ?manager WHERE { ?person <http://example.org/org#reportsTo> ?manager }'"
echo ""
cat relationships.ttl | $CLI query -q "SELECT ?person ?manager WHERE { ?person <http://example.org/org#reportsTo> ?manager }"
echo ""

echo "Query 4: Output as CSV"
echo "Command: ... | triples-cli query -q '...' -f csv"
echo ""
cat relationships.ttl | $CLI query -q "SELECT ?person ?manager WHERE { ?person <http://example.org/org#reportsTo> ?manager }" -f csv
echo ""

echo "Query 5: Output as JSON"
echo "Command: ... | triples-cli query -q '...' -f json"
echo ""
cat relationships.ttl | $CLI query -q "SELECT ?person ?manager WHERE { ?person <http://example.org/org#reportsTo> ?manager }" -f json
echo ""

# -----------------------------------------------------------------------------
echo "4. Interactive REPL (repl)"
echo "--------------------------------------------"
echo "For interactive exploration, use the REPL:"
echo ""
echo "  cat org.ttl relationships.ttl | triples-cli repl"
echo ""
echo "REPL commands:"
echo "  .help      - Show help"
echo "  .count     - Show triple count"
echo "  .exit      - Exit"
echo "  <query>    - Run any SPARQL query"
echo ""
echo "(REPL is interactive, so not demonstrated in this script)"
echo ""

# -----------------------------------------------------------------------------
echo "============================================"
echo "Tutorial complete!"
echo "============================================"
echo ""
echo "Key concepts:"
echo "  - All input comes from stdin"
echo "  - All output goes to stdout"
echo "  - Pipe commands together for complex workflows"
echo "  - Use standard Unix tools (cat, tee, etc.) to compose"
echo ""

# Cleanup
rm -f /tmp/combined.ttl
