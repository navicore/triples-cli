//! CSV to Turtle conversion

use crate::error::Result;
use crate::graph::Graph;
use std::io::{self, Write};
use uuid::Uuid;

/// Convert CSV from stdin to Turtle on stdout
///
/// # Errors
/// Returns error if CSV parsing or Turtle generation fails
#[allow(clippy::significant_drop_tightening)]
pub fn run(base: &str, subject_column: Option<usize>) -> Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut csv_reader = csv::Reader::from_reader(reader);

    let graph = Graph::new()?;

    // Get headers as predicates
    let headers: Vec<String> = csv_reader
        .headers()?
        .iter()
        .map(sanitize_predicate)
        .collect();

    // Process each row
    for result in csv_reader.records() {
        let record = result?;

        // Determine subject IRI
        let subject = subject_column.map_or_else(
            || format!("{base}{}", Uuid::new_v4()),
            |col| {
                let value = record.get(col).unwrap_or_default();
                format!("{base}{}", sanitize_iri_part(value))
            },
        );

        // Add triples for each column
        for (i, value) in record.iter().enumerate() {
            // Skip empty values
            if value.is_empty() {
                continue;
            }

            // Skip subject column if used
            if subject_column == Some(i) {
                continue;
            }

            let predicate = format!("{base}{}", &headers[i]);
            graph.insert_triple(&subject, &predicate, value, true)?;
        }
    }

    // Write Turtle to stdout
    let stdout = io::stdout();
    let writer = stdout.lock();
    graph.write_turtle(writer)?;

    // Flush to ensure all output is written
    io::stdout().flush()?;

    Ok(())
}

/// Sanitize a string for use as a predicate name
fn sanitize_predicate(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Sanitize a string for use in an IRI
fn sanitize_iri_part(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
