use async_trait::async_trait;

use crate::error::XKCDError;

#[async_trait]
pub trait XKCD<T> where 
    T: Sized,
{
    const BASE_URL: &'static str;
    const SUFFIX: &'static str;
    async fn get<S: ToString + Send>(url: S) -> Result<T, XKCDError>;
    async fn get_home() -> Result<T, XKCDError>;
    async fn get_page(page_id: u32) -> Result<T, XKCDError>;
}