use std::{ ops::RangeInclusive};

use rig::{
     OneOrMany,
    embeddings::{Embedding, EmbeddingModel},
    vector_store::{
        InsertDocuments, VectorStoreError, VectorStoreIndex,
        request::{SearchFilter, VectorSearchRequest},
    },
};
// use serde::{Deserialize, Serialize, de::DeserializeOwned};
// use serde_json::Value;
use sqlx::PgPool;
// use uuid::Uuid;

mod enums;
mod search_filter;

pub use enums::PgVectorDistanceFunction;
pub use search_filter::PgSearchFilter;

pub struct PostgresVectorStore<Model: EmbeddingModel> {
    model: Model,
    pg_pool: PgPool,
    documents_table: String,
    distance_function: PgVectorDistanceFunction,
}



