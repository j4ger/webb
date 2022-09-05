use crate::{Error, ROOT};
use scraper::{Html, Selector};
// parse article urls
pub fn parse_day_page(content: String) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();

    let structure = Html::parse_document(&content);

    let li_selector = Selector::parse(".card-text")
        .map_err(|_| Error::ParsingError("li selector".to_string()))?;

    for each in structure.select(&li_selector) {
        let a_selector =
            Selector::parse("a").map_err(|_| Error::ParsingError("link selector".to_string()))?;
        let link = each
            .select(&a_selector)
            .next()
            .ok_or(Error::ParsingError("link".to_string()))?;
        results.push(format!(
            "{}{}",
            ROOT,
            link.value()
                .attr("href")
                .ok_or(Error::ParsingError("link href".to_string()))?
                .to_string(),
        ));
    }

    Ok(results)
}
