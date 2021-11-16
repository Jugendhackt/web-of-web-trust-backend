use self::vertex::DomainVertex;

pub mod acg;
pub mod awg;
pub mod vertex;

#[derive(Debug)]
pub enum GraphError {
    DomainNotFound,
    TargetNotFound,
    LinkNotFound,
    DuplicateLink,
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
