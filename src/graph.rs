#[derive(Debug, Default)]
pub struct Graph {
    adj: Vec<Vec<u32>>,
    m: u32,
}

impl Graph {
    pub fn new(num_vertices: u32) -> Self {
        Self {
            adj: vec![vec![]; num_vertices as usize],
            m: 0,
        }
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.m += 1;
        self.add_arc(u, v);
        self.add_arc(v, u);
    }

    fn add_arc(&mut self, u: u32, v: u32) {
        self.adj[u as usize].push(v);
    }

    pub fn neighborhood(&self, u: u32) -> &[u32] {
        &self.adj[u as usize]
    }

    pub fn order(&self) -> usize {
        self.adj.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;

    #[test]
    fn new_graph() {
        let graph = Graph::new(5);
        assert_eq!(graph.order(), 5);

        let graph = Graph::new(0);
        assert_eq!(graph.order(), 0);
    }

    #[test]
    fn neighborhood() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);

        let nb0: Vec<_> = graph.neighborhood(0).iter().copied().collect();
        let nb1: Vec<_> = graph.neighborhood(1).iter().copied().collect();
        let nb2: Vec<_> = graph.neighborhood(2).iter().copied().collect();
        let nb3: Vec<_> = graph.neighborhood(3).iter().copied().collect();
        let nb4: Vec<_> = graph.neighborhood(4).iter().copied().collect();

        assert_eq!(nb0.len(), 2);
        assert_eq!(nb1.len(), 2);
        assert_eq!(nb2.len(), 2);
        assert_eq!(nb3.len(), 0);
        assert_eq!(nb4.len(), 0);

        assert!(nb0.iter().any(|i| *i==1));
        assert!(nb0.iter().any(|i| *i==2));
        assert!(nb1.iter().any(|i| *i==0));
        assert!(nb1.iter().any(|i| *i==2));
        assert!(nb2.iter().any(|i| *i==0));
        assert!(nb2.iter().any(|i| *i==1));
    }
}
