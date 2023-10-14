mod click;
mod press;
mod type_word;
mod visit;
mod wait;

use click::Click;
use press::Press;
use type_word::Type;
use visit::Visit;
use wait::Wait;

use super::instruction::Instruction;
use async_trait::async_trait;
use playwright::{api::Page, Error};

pub struct Config<'a> {
    pub instructions: &'a Vec<Instruction>,
    pub page: &'a Page,
}

#[async_trait]
pub trait Action {
    async fn execute(&self) -> Result<(), Error>;
}

pub async fn autofill(Config { instructions, page }: Config<'_>) -> Result<(), Error> {
    for instruction in instructions.iter() {
        match instruction {
            Instruction::Visit { url } => Visit{ page, url }.execute().await?,
            Instruction::Click { on } => Click { page, on }.execute().await?,
            Instruction::Press { on } => Press { page, on }.execute().await?,
            Instruction::Type { on, value } => Type { on, value, page }.execute().await?,
            Instruction::Wait { seconds } => Wait{page, seconds}.execute().await?,
        };
    }
    Ok(())
}
