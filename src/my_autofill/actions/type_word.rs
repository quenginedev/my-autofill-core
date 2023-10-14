use async_trait::async_trait;
use playwright::{api::Page, Error};

use super::{Action, Click};

pub struct Type<'a> {
    pub page: &'a Page,
    pub value: &'a String,
    pub on: &'a Option<String>,
}

#[async_trait]
impl<'a> Action for Type<'a> {
    async fn execute(&self) -> Result<(), Error> {
        if let Some(on) = self.on {
            Click { on, page: self.page }.execute().await?
        }
        self.page.keyboard.r#type(&self.value, Some(300.00)).await?;
        Ok(())
    }
}

