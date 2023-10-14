use async_trait::async_trait;
use playwright::{api::Page, Error};

use super::Action;

pub struct Visit<'a> {
  pub page: &'a Page,
  pub url: &'a String
}

#[async_trait]
impl<'a> Action for Visit<'a> {
  async fn execute(&self) -> Result<(), Error> {
    let builder = self.page.goto_builder(self.url);
    builder.goto().await?;
    Ok(())
  }
}