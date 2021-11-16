#[derive(PartialEq, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum DomainVertex {
    Simple(Vertex),
    Colorable(ColorableVertex),
}

impl DomainVertex {
    pub fn is_colored(&self) -> bool {
        match self {
            Self::Simple(_) => false,
            Self::Colorable(vertex) => vertex.color,
        }
    }

    pub fn mark(self) -> Self {
        match self {
            Self::Simple(vertex) => Self::Colorable(ColorableVertex::marked(vertex)),
            Self::Colorable(vertex) => Self::Colorable(vertex),
        }
    }

    pub fn ensure_colorable(self) -> Self {
        match self {
            Self::Simple(vertex) => Self::Colorable(ColorableVertex::unmarked(vertex)),
            Self::Colorable(vertex) => Self::Colorable(vertex),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
pub struct Vertex {
    pub id: i32,
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd, Hash)]
pub struct ColorableVertex {
    pub id: i32,
    pub color: bool,
}

impl From<Vertex> for ColorableVertex {
    fn from(vertex: Vertex) -> Self {
        Self {
            id: vertex.id,
            color: false,
        }
    }
}

impl ColorableVertex {
    pub fn unmarked(vertex: Vertex) -> Self {
        Self {
            id: vertex.id,
            color: false,
        }
    }

    pub fn marked(vertex: Vertex) -> Self {
        Self {
            id: vertex.id,
            color: true,
        }
    }
}
