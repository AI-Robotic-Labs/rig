//! The module defines the [Embeddable] trait, which must be implemented for types that can be embedded.
//! //! # Example
//! ```rust
//! use std::env;
//!
//! use serde::{Deserialize, Serialize};
//!
//! struct FakeDefinition {
//!     id: String,
//!     word: String,
//!     definitions: Vec<String>,
//! }
//!
//! let fake_definition = FakeDefinition {
//!     id: "doc1".to_string(),
//!     word: "hello".to_string(),
//!     definition: "used as a greeting or to begin a conversation".to_string()
//! };
//!
//! impl Embeddable for FakeDefinition {
//!     type Error = anyhow::Error;
//!
//!     fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
//!         // Embeddigns only need to be generated for `definition` field.
//!         // Select it from te struct and return it as a single item.
//!         Ok(OneOrMany::from(self.definition.clone()))
//!     }
//! }
//! ```

/// Error type used for when the `embeddable` method fails.
/// Used by default implementations of `Embeddable` for common types.
#[derive(Debug, thiserror::Error)]
pub enum EmbeddableError {
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Trait for types that can be embedded.
/// The `embeddable` method returns a OneOrMany<String> which contains strings for which embeddings will be generated by the embeddings builder.
/// If there is an error generating the list of strings, the method should return an error that implements `std::error::Error`.
pub trait Embeddable {
    type Error: std::error::Error;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error>;
}

/// Struct containing either a single item or a list of items of type T.
/// If a single item is present, `first` will contain it and `rest` will be empty.
/// If multiple items are present, `first` will contain the first item and `rest` will contain the rest.
/// IMPORTANT: this struct cannot be created with an empty vector.
/// OneOrMany objects can only be created using OneOrMany::from() or OneOrMany::try_from().
#[derive(PartialEq, Eq, Debug)]
pub struct OneOrMany<T> {
    /// First item in the list.
    first: T,
    /// Rest of the items in the list.
    rest: Vec<T>,
}

impl<T: Clone> OneOrMany<T> {
    /// Get the first item in the list.
    pub fn first(&self) -> T {
        self.first.clone()
    }

    /// Get the rest of the items in the list (excluding the first one).
    pub fn rest(&self) -> Vec<T> {
        self.rest.clone()
    }

    /// Get all items in the list (joins the first with the rest).
    pub fn all(&self) -> Vec<T> {
        let mut all = vec![self.first.clone()];
        all.extend(self.rest.clone());
        all
    }
}

/// Create a OneOrMany object with a single item.
impl<T> From<T> for OneOrMany<T> {
    fn from(item: T) -> Self {
        OneOrMany {
            first: item,
            rest: vec![],
        }
    }
}

/// Create a OneOrMany object with a list of items.
impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(items: Vec<T>) -> Self {
        let mut iter = items.into_iter();
        OneOrMany {
            first: match iter.next() {
                Some(item) => item,
                None => panic!("Cannot create OneOrMany with an empty vector."),
            },
            rest: iter.collect(),
        }
    }
}

/// Merge a list of OneOrMany items into a single OneOrMany item.
impl<T: Clone> From<Vec<OneOrMany<T>>> for OneOrMany<T> {
    fn from(value: Vec<OneOrMany<T>>) -> Self {
        let items = value
            .into_iter()
            .flat_map(|one_or_many| one_or_many.all())
            .collect::<Vec<_>>();

        OneOrMany::from(items)
    }
}

//////////////////////////////////////////////////////
/// Implementations of Embeddable for common types ///
//////////////////////////////////////////////////////
impl Embeddable for String {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.clone()))
    }
}

impl Embeddable for i8 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for i16 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for i32 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for i64 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for i128 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for f32 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for f64 {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for bool {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for char {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(self.to_string()))
    }
}

impl Embeddable for serde_json::Value {
    type Error = EmbeddableError;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        Ok(OneOrMany::from(
            serde_json::to_string(self).map_err(EmbeddableError::SerdeError)?,
        ))
    }
}

impl<T: Embeddable> Embeddable for Vec<T> {
    type Error = T::Error;

    fn embeddable(&self) -> Result<OneOrMany<String>, Self::Error> {
        let items = self
            .iter()
            .map(|item| item.embeddable())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(OneOrMany::from(items))
    }
}
