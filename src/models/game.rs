#[derive(Derive, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub day: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct CreateGameSchema {
    pub name: String,
    pub address: String,
    pub day: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub name: Option<String>,
    pub address: Option<String>,
    pub day: Option<chrono::DateTime<chrono::Utc>>,
}
