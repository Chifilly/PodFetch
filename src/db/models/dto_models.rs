use crate::db_object;
#[derive(Deserialize, Serialize, Debug)]
pub struct PodcastFavorUpdateModel {
    pub id: i32,
    pub favored: bool,
}
