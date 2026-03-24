use std::env::args;

use trpl::Html;

async fn fetch_site_title(site_url: String) -> Option<String> {
    let response = trpl::get(&site_url).await;
    let text = response.text().await;

    return Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
}

fn main() {
    let args: Vec<String> = args().collect();

    trpl::block_on(async {
        let url = &args[3];

        match fetch_site_title(url.to_string()).await {
            Some(title) => println!("{}", title),
            None => println!("Err"),
        }
        println!("After await");
    })
}
