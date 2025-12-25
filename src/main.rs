//! triples-cli: A file-oriented CLI for RDF triples
//!
//! Commands:
//! - csv2ttl: Convert CSV to Turtle format
//! - merge: Combine multiple Turtle files
//! - query: Run SPARQL queries against TTL input
//! - repl: Interactive SPARQL shell

use clap::{Parser, Subcommand};
use triples_cli::error::Result;
use triples_cli::{csv2ttl, merge, query, repl};

#[derive(Parser)]
#[command(name = "triples-cli")]
#[command(about = "A file-oriented CLI for RDF triples", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert CSV to Turtle format
    ///
    /// Reads CSV from stdin, writes Turtle to stdout.
    /// First row is treated as headers (predicates).
    #[command(name = "csv2ttl")]
    Csv2Ttl {
        /// Base IRI for generated subjects
        #[arg(short, long, default_value = "http://example.org/")]
        base: String,

        /// Column to use as subject (0-indexed), or generate UUIDs if not specified
        #[arg(short, long)]
        subject_column: Option<usize>,
    },

    /// Merge multiple Turtle files into one
    ///
    /// Reads Turtle from stdin (concatenated), writes merged Turtle to stdout.
    /// Resolves prefix conflicts by renaming.
    Merge,

    /// Run a SPARQL query against Turtle input
    ///
    /// Reads Turtle from stdin, runs query, writes results to stdout.
    Query {
        /// SPARQL query string
        #[arg(short, long)]
        query: String,

        /// Output format: table, csv, json
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Interactive SPARQL shell
    ///
    /// Reads Turtle from stdin, then provides an interactive query prompt.
    Repl,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv2Ttl {
            base,
            subject_column,
        } => csv2ttl::run(&base, subject_column),
        Commands::Merge => merge::run(),
        Commands::Query { query, format } => query::run(&query, &format),
        Commands::Repl => repl::run(),
    }
}
