use serde::Deserialize;


#[derive(Deserialize, Debug)]
/// # FetchRequest
/// Request sent by clients for fetching information about domains by a fqdn hash prefix.
pub struct FetchRequest {
    pub fqdn_hash: String,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Deserialize, Debug)]
/// # UpdateRequest
/// UpdateRequest sent by scrapers to the server with information about a domain and it's linked domains.
/// The link array may only have a max of 100 elements. Please batch large domains to decrease load on the server. Otherwise your request may be rejected by the server for size reasons
pub struct UpdateRequest {
    pub fqdn: String,
    pub network: bool,
    pub links: Vec<String>,
    pub last_updated: i64,
}
