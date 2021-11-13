use serde::Deserialize;

#[derive(Deserialize)]
pub struct FetchRequest {
    pub fqdn: String,
    pub page: u32,
    pub per_page: u32,
}
