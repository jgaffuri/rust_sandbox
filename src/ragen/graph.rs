//use geos::GeometryTypes::{Point, LineString, Polygon};
//use crate::ragen::base::Feature;

pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    pub domains: Vec<Domain<'a>>,
}

pub struct Node<'a> {
    pub geometry: geos::Geometry,
    pub edges_in: Vec<&'a Edge<'a>>,
    pub edges_out: Vec<&'a Edge<'a>>,
}

pub struct Edge<'a> {
    //pub geometry: geos::GeometryTypes::LineString,
    pub from: &'a Node<'a>,
    pub to: &'a Node<'a>,
}

pub struct Domain<'a> {
    //pub geometry: geos::GeometryTypes::Polygon,
    pub edges: Vec<&'a Edge<'a>>,
}

/*
trait GraphBuilder {
    fn build_graph(&self, features: Vec<Feature>) -> Graph<'_>;
}
*/