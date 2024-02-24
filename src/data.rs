use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}
