//! triples-cli: A file-oriented CLI for RDF triples
//!
//! This library provides functionality for:
//! - Converting CSV to Turtle format
//! - Merging multiple Turtle files
//! - Running SPARQL queries against TTL input
//! - Interactive SPARQL shell (REPL)

pub mod csv2ttl;
pub mod error;
pub mod graph;
pub mod merge;
pub mod query;
pub mod repl;
