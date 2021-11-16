use super::{vertex::DomainVertex, Link};
use dashmap::DashMap;
use std::sync::Arc;

pub struct HotAWG {
    pub edges: Arc<DashMap<DomainVertex, Vec<Link>>>,
    pub weights: DashMap<i32, f32>,
}
