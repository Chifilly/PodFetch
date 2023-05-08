use crate::schema::*;
use diesel::prelude::{AsChangeset, Identifiable, Insertable, Queryable};
use crate::service::environment_service::OidcConfig;
use utoipa::ToSchema;

#[derive(
    Serialize, Deserialize, Queryable, Insertable, Debug, Clone, Identifiable, AsChangeset,ToSchema
)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    pub id: i32,
    pub auto_download: bool,
    pub auto_update: bool,
    pub auto_cleanup: bool,
    pub auto_cleanup_days: i32,
    pub podcast_prefill: i32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigModel {
    pub podindex_configured: bool,
    pub rss_feed: String,
    pub server_url: String,
    pub basic_auth: bool,
    pub oidc_configured: bool,
    pub oidc_config: Option<OidcConfig>
}
