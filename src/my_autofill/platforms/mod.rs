mod anime_owl;

use serde::{Serialize, Deserialize};

use super::Instruction;

#[derive(Serialize, Deserialize, Debug)]
pub enum Platform {
  AnimeOwl
}

pub fn get_platform_instructions(platform: Platform) -> Vec<Instruction> {
  match platform {
      Platform::AnimeOwl => anime_owl::instructions()
  }
}