// api_key:  56f2b6fe4532800b2ca7a0d4:a474052c-9e14-495f-b9f9-b429e5f3563b
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[warn(non_snake_case)]
pub struct Link {
    id: String,
    title: String,
    description: Option<String>,
    sourceUrl: String,
    isPrivate: bool,
    thumbnailSource: Option<String>,
    thumbnailSourceUrl: Option<String>,
    thumbnailId: Option<String>,
    thumbnailUrl: Option<String>,
    url: String,
    createdAt: String,
    tags: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    id: String,
    name: String,
    url: String,
    pack: Option<Pack>,
}

#[derive(Deserialize, Debug)]
pub struct Pack {
    id: String,
    name: String,
    color: u16,
}

#[tokio::main]
pub async fn get_links() -> Result<Vec<Link>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://tagpacker.com/api/users/56f2b6fe4532800b2ca7a0d4/links")
        .await?
        .json::<Vec<Link>>()
        .await?;

    Ok(resp)
}
