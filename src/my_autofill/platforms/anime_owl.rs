use crate::my_autofill::instruction::Instruction;

pub fn instructions() -> Vec<Instruction> {
  vec![
    Instruction::Visit {url: "https://anime-owl.net/".to_string()},
    Instruction::Type {
      value: "attack on titan".to_string(),
      on: Some("#search-form-1".to_string())
    },
    Instruction::Wait  {seconds: 5},
  ]
}