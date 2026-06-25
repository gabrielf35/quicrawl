use scraper::{Html, Selector};

pub async fn get(url: &str) -> std::result::Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("quicrawl (https://github.com/indium114/quicrawl)")
        .build()?;

    let response = client.get(url).send().await?;

    response.text().await
}

pub fn parse_links(html: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(link.to_string());
        }
    }

    links
}
