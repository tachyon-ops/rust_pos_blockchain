use std::fmt;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockData {
    foo: String,
}

impl fmt::Display for BlockData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockData: ()")
    }
}

impl BlockData {
    pub fn new() -> Self {
        Self {
            foo: String::from("foo"),
        }
    }
}
