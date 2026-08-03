
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub domains: Vec<Domain>,
}

pub struct Node {
    pub edges_in: Vec<&Edge>,
    pub edges_out: Vec<&Edge>,
}

pub struct Edge {
    pub from: &Node,
    pub to: &Node,
}

pub struct Domain {
    pub edges: Vec<&Edge>,
}
