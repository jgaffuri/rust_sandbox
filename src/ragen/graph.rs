
pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    pub domains: Vec<Domain<'a>>,
}

pub struct Node<'a> {
    pub edges_in: Vec<&'a Edge<'a>>,
    pub edges_out: Vec<&'a Edge<'a>>,
}

pub struct Edge<'a> {
    pub from: &'a Node<'a>,
    pub to: &'a Node<'a>,
}

pub struct Domain<'a> {
    pub edges: Vec<&'a Edge<'a>>,
}
