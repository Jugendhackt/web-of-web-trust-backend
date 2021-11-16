use super::{vertex::DomainVertex, Link};
use crate::domains::graph::GraphError;
use dashmap::DashMap;
use std::sync::Arc;

// Acyclic counter graph
pub struct ACG {
    pub edges: Arc<DashMap<DomainVertex, Vec<Link>>>,
}

impl ACG {
    pub fn new() -> Self {
        Self {
            edges: Arc::new(DashMap::new()),
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

    pub fn _decycle(
        domain: DomainVertex,
        path: Vec<DomainVertex>,
        edges: Arc<DashMap<DomainVertex, Vec<Link>>>,
        marked: Arc<DashMap<DomainVertex, bool>>,
    ) {
        let links = edges.get_mut(&domain).unwrap().value_mut();

        for (index, link) in links.into_iter().enumerate() {
            match path.binary_search(&link.target) {
                Ok(_) => {
                    links.remove(index);
                }
                Err(_) => match marked.get(&link.target) {
                    Some(node_marked) => {
                        if !node_marked.value() {
                            Self::_decycle(
                                link.target,
                                [path, vec![domain]].concat(),
                                edges,
                                marked.clone(),
                            )
                        }
                    }
                    None => (),
                },
            };
        }
    }

    pub fn decycle(&mut self, root: Vec<DomainVertex>) {
        let marked_map = Arc::new(DashMap::with_capacity(self.edges.capacity() - 1));

        for domain in root {
            Self::_decycle(domain, vec![], self.edges, marked_map.clone());
        }
    }
}
