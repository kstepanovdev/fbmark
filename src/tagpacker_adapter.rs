// api_key:  56f2b6fe4532800b2ca7a0d4:a474052c-9e14-495f-b9f9-b429e5f3563b
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[warn(non_snake_case)]
pub struct Link {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub sourceUrl: String,
    pub isPrivate: bool,
    pub thumbnailSource: Option<String>,
    pub thumbnailSourceUrl: Option<String>,
    pub thumbnailId: Option<String>,
    pub thumbnailUrl: Option<String>,
    pub url: String,
    pub createdAt: String,
    pub tags: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub url: String,
    pub pack: Option<Pack>,
}

#[derive(Deserialize, Debug)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub color: u16,
}

#[tokio::main]
pub async fn get_links() -> Result<Vec<Link>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://tagpacker.com/api/users/56f2b6fe4532800b2ca7a0d4/links")
        .await?
        .json::<Vec<Link>>()
        .await?;

    Ok(resp)
}
