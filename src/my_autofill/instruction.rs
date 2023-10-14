use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Instruction {
  Visit { url: String },
  Click { on: String },
  Press { on: String },
  Type { value: String, on: Option<String> },
  Wait { seconds: usize }
}
