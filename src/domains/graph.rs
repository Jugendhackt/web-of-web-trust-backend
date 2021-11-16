use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub type DomainId = i32;

#[derive(Debug)]
pub enum GraphError {
    DomainNotFound,
    TargetNotFound,
    LinkNotFound,
    DuplicateLink,
}

#[derive(PartialEq, Clone, PartialOrd, Ord, Eq, Hash)]
pub enum DomainVertex {
    Id(i32),
    Colorable(i32, bool),
}

#[derive(PartialEq, Clone, PartialOrd, Ord, Eq)]
pub struct Link {
    pub target: DomainVertex,
    pub count: u32,
}

#[derive(PartialEq, Clone, PartialOrd, Ord, Eq)]
pub struct ColoredLink {
    pub link: Link,
    pub color: bool,
}

impl Link {
    pub fn new(target: DomainVertex, count: u32) -> Self {
        Self { target, count }
    }
}

impl From<Link> for ColoredLink {
    fn from(link: Link) -> Self {
        Self { link, color: false }
    }
}

// Acyclic counter graph
pub struct ACG {
    pub edges: DashMap<DomainVertex, Vec<Link>>,
}

impl ACG {
    pub fn new() -> Self {
        Self {
            edges: DashMap::new(),
        }
    }

    pub fn add_domain(&mut self, domain: DomainVertex) -> Result<(), GraphError> {
        match self.edges.insert(domain.clone(), vec![]) {
            Some(links) => {
                self.edges.insert(domain, links);
                Err(GraphError::DuplicateLink)
            }
            None => Ok(()),
        }
    }

    pub fn get_domain(&self, domain: &DomainVertex) -> Result<Vec<Link>, GraphError> {
        match self.edges.get(domain) {
            Some(edges_ref) => Ok(edges_ref.value().clone()),
            None => Err(GraphError::DomainNotFound),
        }
    }

    pub fn remove_domain(&mut self, domain: &DomainVertex) -> Result<Vec<Link>, GraphError> {
        match self.edges.remove(domain) {
            Some((_, links)) => Ok(links),
            None => Err(GraphError::DomainNotFound),
        }
    }

    pub async fn decycle(
        &mut self,
        domain: &DomainVertex,
        path: Vec<DomainVertex>,
        semaphore: Arc<Semaphore>,
    ) -> Result<(), GraphError> {
        let mut links = self.edges.get_mut(&domain).unwrap().clone();
        let mut join_handles = Vec::new();

        for (index, link) in links.clone().iter().enumerate() {
            if path.contains(&link.target) {
                links.remove(index);
            } else {
                let permit = semaphore.clone().acquire_owned().await.unwrap();

                join_handles.push(tokio::spawn(async move {
                    self.decycle(
                        &link.target,
                        [vec![domain.clone()], path].concat(),
                        semaphore.clone(),
                    )
                    .await;
                    drop(permit);
                }));
            }
        }

        for handle in join_handles {
            handle.await.unwrap();
        }

        Ok(())
    }

    pub async fn remove_cycles(&mut self, root_set: &Vec<DomainVertex>) -> Result<(), GraphError> {
        let semaphore = Arc::new(Semaphore::new(3));

        for domain in root_set {
            self.decycle(domain, vec![], semaphore.clone()).await?;
        }

        Ok(())
    }

    pub fn unlink(
        &mut self,
        source: &DomainVertex,
        target: &DomainVertex,
    ) -> Result<Link, GraphError> {
        match self.edges.get_mut(source) {
            Some(mut edge_ref) => {
                let edges = edge_ref.value_mut();

                match edges.binary_search_by_key(target, |link| link.target.clone()) {
                    Ok(index) => Ok(edges.remove(index)),
                    Err(_) => Err(GraphError::TargetNotFound),
                }
            }
            None => Err(GraphError::DomainNotFound),
        }
    }

    pub fn link(
        &mut self,
        source: &DomainVertex,
        target: &DomainVertex,
        count: u32,
    ) -> Result<(), GraphError> {
        match self.edges.get_mut(source) {
            Some(mut edge_ref) => {
                let edges = edge_ref.value_mut();

                match edges.binary_search_by_key(target, |link| link.target.clone()) {
                    Ok(index) => Err(GraphError::DuplicateLink),
                    Err(index) => {
                        edges.insert(index, Link::new(target.clone(), count));
                        Ok(())
                    }
                }
            }
            None => Err(GraphError::DomainNotFound),
        }
    }
}
