#[derive(Derive, FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub day: chrono::DateTime<chrono::Utc>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct CreateGameSchema {
    pub name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>,
}
