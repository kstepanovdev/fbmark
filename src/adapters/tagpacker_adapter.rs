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
pub async fn get_links(user_id: &str) -> Result<Vec<Link>, Box<dyn std::error::Error>> {
    let url = format!("https://tagpacker.com/api/users/{}/links", user_id);
    let response = reqwest::get(url).await;
    match response {
        Ok(result) => {
            return Ok(result.json::<Vec<Link>>().await.unwrap());
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
