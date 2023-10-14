
use async_trait::async_trait;
use playwright::{api::Page, Error};
use super::Action;

pub struct Click<'a> {
  pub page: &'a Page,
  pub on: &'a String
}

#[async_trait]
impl<'a> Action for Click<'a> {

    async fn execute(&self) -> Result<(), Error> {
        
        let page = self.page;
        let on = self.on;
        
        page.wait_for_selector_builder(on)
            .timeout(30000.0);

            
        page.click_builder(on)
            .click()
            .await?;

        Ok(())
    }
}