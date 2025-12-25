//! Interactive SPARQL REPL

use crate::error::Result;
use crate::graph::Graph;
use crate::query::format_results;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{self, Read};

const HELP_TEXT: &str = r"
Commands:
  .help      Show this help
  .prefixes  Show defined prefixes
  .count     Show triple count
  .exit      Exit the REPL
  .quit      Exit the REPL

Any other input is treated as a SPARQL query.

Example queries:
  SELECT * WHERE { ?s ?p ?o } LIMIT 10
  SELECT ?s WHERE { ?s a <http://example.org/Person> }
";

/// Run the interactive REPL
///
/// # Errors
/// Returns error if reading input or initialization fails
pub fn run() -> Result<()> {
    // Read all TTL from stdin first
    let stdin = io::stdin();
    let mut input = String::new();

    // Check if stdin is a tty - if so, no data to load
    if atty::is(atty::Stream::Stdin) {
        eprintln!("No TTL input provided. Starting with empty graph.");
        eprintln!("Pipe in TTL data: cat data.ttl | triples-cli repl");
    } else {
        stdin.lock().read_to_string(&mut input)?;
    }

    let graph = Graph::new()?;

    if !input.is_empty() {
        graph.load_turtle(input.as_bytes())?;
        eprintln!("Loaded {} triples", graph.len());
    }

    eprintln!("Type .help for commands, .exit to quit\n");

    let mut rl = DefaultEditor::new()
        .map_err(|e| crate::error::TriplesError::Io(io::Error::other(e.to_string())))?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line);

                match line {
                    ".help" => {
                        println!("{HELP_TEXT}");
                    }
                    ".exit" | ".quit" => {
                        break;
                    }
                    ".count" => {
                        println!("{} triples", graph.len());
                    }
                    ".prefixes" => {
                        // Query for prefixes - this is a simplified version
                        println!("(prefix extraction not yet implemented)");
                    }
                    _ => {
                        // Treat as SPARQL query
                        match graph.query(line) {
                            Ok(results) => {
                                if let Err(e) = format_results(results, "table") {
                                    eprintln!("Error formatting results: {e}");
                                }
                            }
                            Err(e) => {
                                eprintln!("Query error: {e}");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }

    Ok(())
}
