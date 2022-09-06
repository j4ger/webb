use crate::{Error, SuccessTarget};
use log::info;
use scraper::{Html, Selector};
// parse article, returns the actual article
pub fn parse_article(content: String, url: &str) -> Result<SuccessTarget, Error> {
    let structure = Html::parse_document(&content);
    info!("Parsing {}.", url);

    let article_id_split = url.split('#').collect::<Vec<&str>>();
    let article_id = article_id_split
        .get(1)
        .ok_or(Error::ParsingError("article id selector".to_string()))?;

    let card_selector =
        Selector::parse(".card").map_err(|_| Error::ParsingError("card selector".to_string()))?;
    let link_selector =
        Selector::parse("a").map_err(|_| Error::ParsingError("link selector".to_string()))?;
    let card = structure
        .select(&card_selector)
        .find(|each| {
            if let Some(inner) = each.select(&link_selector).next() {
                return inner.value().id() == Some(*article_id);
            }
            return false;
        })
        .ok_or(Error::ParsingError("title element".to_string()))?;
    let title = card
        .select(&link_selector)
        .next()
        .ok_or(Error::ParsingError("title id".to_string()))?
        .text()
        .collect::<String>();

    let article_selector = Selector::parse("p.card-text")
        .map_err(|_| Error::ParsingError("article selector".to_string()))?;
    let article = card
        .select(&article_selector)
        .next()
        .ok_or(Error::ParsingError("select article".to_string()))?
        .text()
        .collect::<String>();

    Ok(SuccessTarget {
        url: url.to_string(),
        title: title.to_string(),
    })
}
