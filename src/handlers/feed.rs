use serde::Serialize;
use rss::{Channel};
use worker::*;
use futures::future::join_all;

#[derive(Serialize)]
struct Item {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

async fn load_channel(url: &str) -> Result<Channel> {
    let request = Request::new_with_init(url, &RequestInit {
        cf: CfProperties {
            cache_everything: Some(true),
            cache_ttl: Some(3600),
            ..CfProperties::default()
        },
        ..Default::default()
    })?;
    let mut res = Fetch::Request(request).send().await?;
    let bytes = res.bytes().await?;

    Channel::read_from(&bytes[..]).map_err( |e| worker::Error::RustError(e.to_string()))
}

fn convert_items(items: Vec<rss::Item>) -> Vec<Item> {
    let mut convert_items = vec![];
    for item in items {
        convert_items.push(Item {
            title: item.title,
            link: item.link,
            description: item.description,
            pub_date: item.pub_date
        })
    }
    convert_items
}

async fn collect_items(urls: &[&str]) -> Result<Vec<Item>> {
    let channels = join_all(urls.iter().map(|url| load_channel(url))).await;
    let items = channels
        .into_iter()
        .filter_map(|channel| channel.ok())
        .flat_map(|channel| convert_items(channel.items))
        .collect::<Vec<_>>();
    Ok(items)
}

pub async fn handler(_: Request, _: RouteContext<()>) -> Result<Response> {
   let items = collect_items(&[
       "https://blog.aotoki.me/index.xml",
       "https://vocus.cc/rss/user/5f494cdafd89780001b0be5e/article.xml"
   ]).await?;

   Response::from_json(&items)
}
