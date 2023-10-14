mod actions;
mod instruction;
pub mod platforms;

use actions::{autofill, Config as AutofillConfig};
use playwright::{Error, Playwright, api::BrowserContext};
pub use instruction::Instruction;
use tokio::spawn;

pub struct Config {
    pub instructions: Vec<Vec<Instruction>>,
}

async fn handle_autofill(context: BrowserContext, instructions: Vec<Instruction>) -> Result<(), Error> {
  let page = &context.new_page().await?;
  let config = AutofillConfig { instructions: & instructions, page };
  autofill(config).await?;
  Ok(())
}

pub async fn runner(config: Config) -> Result<(), Error> {
    for instructions in config.instructions.iter() {
        let process = instructions.clone();
        
        spawn(async move {
            let playwright = Playwright::initialize().await.expect("Error initializing playwright");
            playwright.prepare().expect("Error preparing playwright");
            let browser_type = playwright.chromium();
            let browser = browser_type.launcher().headless(false).launch().await.expect("Error launching chrome");
            let context = browser.context_builder().build().await.expect("Error creating browser context");
            handle_autofill(context, process).await.expect("Thread error");
        });
    };

    Ok(())
}
