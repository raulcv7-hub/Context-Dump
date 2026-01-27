use serde::{Deserialize, Serialize};

/// Estructura de transferencia de datos para la respuesta del motor Docling.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DoclingResult {
    pub content: String,
    pub token_count: usize,
}
