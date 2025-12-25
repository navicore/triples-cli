//! Merge multiple Turtle files

use crate::error::Result;
use crate::graph::Graph;
use std::io::{self, Read, Write};

/// Merge Turtle input from stdin and write combined output to stdout
///
/// Reads all Turtle data from stdin (potentially multiple concatenated files),
/// combines into a single graph, and outputs as a single Turtle document.
///
/// # Errors
/// Returns error if parsing or serialization fails
pub fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let graph = Graph::new()?;
    graph.load_turtle(input.as_bytes())?;

    let stdout = io::stdout();
    let writer = stdout.lock();
    graph.write_turtle(writer)?;

    io::stdout().flush()?;

    Ok(())
}
