
use async_trait::async_trait;
use playwright::{api::Page, Error};

use super::Action;

pub struct Press<'a> {
  pub page: &'a Page,
  pub on: &'a String
}

#[async_trait]
impl<'a> Action for Press<'a> {
  async fn execute(&self) -> Result<(), Error> {
    self.page.keyboard.press(self.on, None).await?;
    Ok(())
  }
}