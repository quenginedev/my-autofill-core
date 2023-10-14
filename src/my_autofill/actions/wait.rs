use async_trait::async_trait;
use playwright::{api::Page, Error};

use super::Action;

pub struct Wait<'a> {
  pub page: &'a Page,
  pub seconds: &'a usize
}

#[async_trait]
impl<'a> Action for Wait<'a> {
  async fn execute(&self) -> Result<(), Error> {
    let milliseconds = (self.seconds * 1000) as f64;
    self.page.wait_for_timeout(milliseconds).await;
    Ok(())
  }
}

