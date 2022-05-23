use serde::Serialize;
use rss::Channel;
use worker::*;

#[derive(Serialize)]
struct Item {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    pub_date: Option<String>,
}

async fn load_channel(url: &str) -> Result<Channel> {
    let mut res = Fetch::Url(url.parse()?).send().await?;
    let bytes = res.bytes().await?;

    Channel::read_from(&bytes[..]).map_err( |e| worker::Error::RustError(e.to_string()))
}

pub async fn handler(_: Request, _: RouteContext<()>) -> Result<Response> {
   let channel: Channel = load_channel("https://blog.aotoki.me/index.xml").await?;
   let items = channel.items.into_iter().map(|item| Item {
     title: item.title,
     link: item.link,
     description: item.description,
     pub_date: item.pub_date
   }).collect::<Vec<_>>();

   Response::from_json(&items)
}
