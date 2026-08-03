use geos::{Geom, Geometry, GeometryTypes};

pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    pub domains: Vec<Domain<'a>>,
}

pub struct Node<'a> {
    pub geometry: Point,
    pub edges_in: Vec<&'a Edge<'a>>,
    pub edges_out: Vec<&'a Edge<'a>>,
}

pub struct Edge<'a> {
    pub geometry: LineString,
    pub from: &'a Node<'a>,
    pub to: &'a Node<'a>,
}

pub struct Domain<'a> {
    pub geometry: Polygon,
    pub edges: Vec<&'a Edge<'a>>,
}
