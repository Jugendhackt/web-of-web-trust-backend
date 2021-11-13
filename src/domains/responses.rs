use serde::Serialize;

#[derive(Serialize)]
pub struct DomainResponse {
    pub fqdn: String,
    pub score: [f32; 2],
    pub last_updated: i64,
}

#[derive(Serialize)]
pub struct AggregatedDomainResponse {
    pub domains: Vec<DomainResponse>,
}
