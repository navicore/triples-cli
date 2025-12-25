//! Error types for triples-cli

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriplesError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("RDF parse error: {0}")]
    Parse(String),

    #[error("SPARQL error: {0}")]
    Sparql(String),

    #[error("Invalid IRI: {0}")]
    InvalidIri(String),
}

pub type Result<T> = std::result::Result<T, TriplesError>;
