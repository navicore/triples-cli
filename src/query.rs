//! SPARQL query execution

use crate::error::{Result, TriplesError};
use crate::graph::Graph;
use oxigraph::sparql::QueryResults;
use std::io::{self, Read, Write};

/// Run a SPARQL query against Turtle input from stdin
///
/// # Errors
/// Returns error if parsing, query execution, or output fails
pub fn run(sparql: &str, format: &str) -> Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input)?;

    let graph = Graph::new()?;
    graph.load_turtle(input.as_bytes())?;

    let results = graph.query(sparql)?;
    format_results(results, format)?;

    Ok(())
}

/// Format and print query results
///
/// # Errors
/// Returns error if writing to stdout fails
pub fn format_results(results: QueryResults, format: &str) -> Result<()> {
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    match results {
        QueryResults::Solutions(solutions) => {
            let variables: Vec<_> = solutions.variables().to_vec();

            match format {
                "csv" => {
                    // CSV header
                    let header: Vec<_> = variables
                        .iter()
                        .map(oxigraph::sparql::Variable::as_str)
                        .collect();
                    writeln!(writer, "{}", header.join(","))?;

                    // CSV rows
                    for solution in solutions {
                        let solution = solution.map_err(|e| TriplesError::Sparql(e.to_string()))?;
                        let row: Vec<_> = variables
                            .iter()
                            .map(|v| solution.get(v).map_or(String::new(), format_term))
                            .collect();
                        writeln!(writer, "{}", row.join(","))?;
                    }
                }
                "json" => {
                    writeln!(writer, "[")?;
                    let mut first = true;
                    for solution in solutions {
                        let solution = solution.map_err(|e| TriplesError::Sparql(e.to_string()))?;
                        if !first {
                            writeln!(writer, ",")?;
                        }
                        first = false;
                        write!(writer, "  {{")?;
                        let pairs: Vec<_> = variables
                            .iter()
                            .filter_map(|v| {
                                solution.get(v).map(|term| {
                                    format!("\"{}\": \"{}\"", v.as_str(), format_term(term))
                                })
                            })
                            .collect();
                        write!(writer, "{}", pairs.join(", "))?;
                        write!(writer, "}}")?;
                    }
                    writeln!(writer)?;
                    writeln!(writer, "]")?;
                }
                _ => {
                    // Table format (default)
                    print_table(&variables, solutions, &mut writer)?;
                }
            }
        }
        QueryResults::Boolean(value) => {
            writeln!(writer, "{value}")?;
        }
        QueryResults::Graph(_) => {
            writeln!(
                writer,
                "(CONSTRUCT queries not yet supported in this format)"
            )?;
        }
    }

    writer.flush()?;
    Ok(())
}

/// Print results as a formatted table
fn print_table<W: Write>(
    variables: &[oxigraph::sparql::Variable],
    solutions: oxigraph::sparql::QuerySolutionIter,
    writer: &mut W,
) -> Result<()> {
    // Collect all rows first to calculate column widths
    let mut rows: Vec<Vec<String>> = Vec::new();

    for solution in solutions {
        let solution = solution.map_err(|e| TriplesError::Sparql(e.to_string()))?;
        let row: Vec<String> = variables
            .iter()
            .map(|v| solution.get(v).map_or(String::new(), format_term))
            .collect();
        rows.push(row);
    }

    if rows.is_empty() {
        writeln!(writer, "(no results)")?;
        return Ok(());
    }

    // Calculate column widths
    let headers: Vec<_> = variables.iter().map(|v| v.as_str().to_string()).collect();
    let mut widths: Vec<usize> = headers.iter().map(String::len).collect();

    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }

    // Print header
    print_row(&headers, &widths, writer)?;
    print_separator(&widths, writer)?;

    // Print rows
    for row in &rows {
        print_row(row, &widths, writer)?;
    }

    Ok(())
}

fn print_row<W: Write>(row: &[String], widths: &[usize], writer: &mut W) -> Result<()> {
    for (i, cell) in row.iter().enumerate() {
        if i > 0 {
            write!(writer, " | ")?;
        }
        write!(writer, "{:width$}", cell, width = widths[i])?;
    }
    writeln!(writer)?;
    Ok(())
}

fn print_separator<W: Write>(widths: &[usize], writer: &mut W) -> Result<()> {
    for (i, width) in widths.iter().enumerate() {
        if i > 0 {
            write!(writer, "-+-")?;
        }
        write!(writer, "{}", "-".repeat(*width))?;
    }
    writeln!(writer)?;
    Ok(())
}

/// Format an RDF term for display
fn format_term(term: &oxigraph::model::Term) -> String {
    match term {
        oxigraph::model::Term::NamedNode(n) => n.as_str().to_string(),
        oxigraph::model::Term::BlankNode(b) => format!("_:{}", b.as_str()),
        oxigraph::model::Term::Literal(l) => l.value().to_string(),
        oxigraph::model::Term::Triple(_) => "(triple)".to_string(),
    }
}
