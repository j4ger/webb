const RETRIES: i32 = 5;

use crate::Error;
use log::{trace, warn};

pub async fn fetch(url: &str) -> Result<String, Error> {
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
