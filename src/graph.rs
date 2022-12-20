use hashbrown::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<VId, V = (), E = ()> {
    vertices: HashMap<VId, V>,
    adjacent: HashMap<VId, Vec<(VId, E)>>,
}

impl<VId, V, E> Graph<VId, V, E>
where
    VId: Eq + Hash,
    V: Hash,
{
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            adjacent: HashMap::new(),
        }
    }

    pub fn push_vertex(&mut self, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }

    pub fn push_edge(&mut self, from: VId, to: VId, edge: E) {
        let edge_entry = self.adjacent.entry(from).or_default();
        edge_entry.push((to, edge));
    }

    pub fn push_edge_undirected(&mut self, a: VId, b: VId, edge: E)
    where
        VId: Clone,
        E: Clone,
    {
        self.push_edge(a.clone(), b.clone(), edge.clone());
        self.push_edge(b, a, edge);
    }
}
