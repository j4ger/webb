const RETRIES: i32 = 5;

use crate::{Error, LogExpect};
use log::{trace, warn};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct FetchClient {
    throttler: Arc<Semaphore>,
}

impl FetchClient {
    pub fn new(throttle_count: usize) -> Self {
        FetchClient {
            throttler: Arc::new(Semaphore::new(throttle_count)),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<String, Error> {
        let _permit = Arc::clone(&self.throttler)
            .acquire_owned()
            .await
            .log_expect("Failed to aquire permit.");
        trace!("Fetching {}.", url);
        let mut retry = RETRIES;
        while retry > 0 {
            retry -= 1;
            match reqwest::get(url).await {
                Ok(response) => match response.text().await {
                    Ok(inner) => {
                        return Ok(inner);
                    }
                    Err(error) => {
                        warn!(
                            "Decoding response from {} failed: {}, retrying[{}].",
                            url, error, retry
                        );
                    }
                },
                Err(error) => {
                    warn!("Fetching {} failed: {}, retrying[{}].", url, error, retry);
                }
            }
        }
        return Err(Error::NetworkError);
    }
}
