//! In-memory RDF graph wrapper around oxigraph

use crate::error::{Result, TriplesError};
use oxigraph::io::{RdfFormat, RdfSerializer};
use oxigraph::model::GraphNameRef;
use oxigraph::sparql::QueryResults;
use oxigraph::store::Store;
use std::io::{Read, Write};

/// An in-memory RDF graph
pub struct Graph {
    store: Store,
}

impl Graph {
    /// Create a new empty graph
    ///
    /// # Errors
    /// Returns error if store creation fails
    pub fn new() -> Result<Self> {
        let store = Store::new().map_err(|e| TriplesError::Parse(e.to_string()))?;
        Ok(Self { store })
    }

    /// Load Turtle data from a reader
    ///
    /// # Errors
    /// Returns error if parsing fails
    pub fn load_turtle<R: Read>(&self, reader: R) -> Result<()> {
        self.store
            .load_from_reader(RdfFormat::Turtle, reader)
            .map_err(|e| TriplesError::Parse(e.to_string()))?;
        Ok(())
    }

    /// Write the graph as Turtle to a writer
    ///
    /// # Errors
    /// Returns error if serialization fails
    pub fn write_turtle<W: Write>(&self, writer: W) -> Result<()> {
        let mut serializer = RdfSerializer::from_format(RdfFormat::Turtle).for_writer(writer);
        for quad in &self.store {
            let quad = quad.map_err(|e| TriplesError::Parse(e.to_string()))?;
            serializer
                .serialize_triple(quad.as_ref())
                .map_err(|e| TriplesError::Parse(e.to_string()))?;
        }
        serializer
            .finish()
            .map_err(|e| TriplesError::Parse(e.to_string()))?;
        Ok(())
    }

    /// Execute a SPARQL query and return results
    ///
    /// # Errors
    /// Returns error if query parsing or execution fails
    pub fn query(&self, sparql: &str) -> Result<QueryResults> {
        self.store
            .query(sparql)
            .map_err(|e| TriplesError::Sparql(e.to_string()))
    }

    /// Add a triple to the graph
    ///
    /// # Errors
    /// Returns error if insertion fails
    pub fn insert_triple(
        &self,
        subject: &str,
        predicate: &str,
        object: &str,
        object_is_literal: bool,
    ) -> Result<()> {
        use oxigraph::model::{Literal, NamedNode, Triple};

        let subj = NamedNode::new(subject).map_err(|e| TriplesError::InvalidIri(e.to_string()))?;
        let pred =
            NamedNode::new(predicate).map_err(|e| TriplesError::InvalidIri(e.to_string()))?;

        let triple = if object_is_literal {
            Triple::new(subj, pred, Literal::new_simple_literal(object))
        } else {
            let obj =
                NamedNode::new(object).map_err(|e| TriplesError::InvalidIri(e.to_string()))?;
            Triple::new(subj, pred, obj)
        };

        self.store
            .insert(triple.as_ref().in_graph(GraphNameRef::DefaultGraph))
            .map_err(|e| TriplesError::Parse(e.to_string()))?;

        Ok(())
    }

    /// Get the number of triples in the graph
    #[must_use]
    pub fn len(&self) -> usize {
        self.store.len().unwrap_or(0)
    }

    /// Check if the graph is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
