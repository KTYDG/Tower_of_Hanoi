pub struct List {
    pub graph: Vec<Vec<Vec<u8>>>,
}

impl List {
    pub fn push_front(&mut self, element: Vec<Vec<u8>>) {
        self.graph.insert(0, element);
    }
    pub fn push_back(&mut self, element: Vec<Vec<u8>>) {
        self.graph.push(element);
    }

    pub fn pop_front(&mut self) -> Option<Vec<Vec<u8>>> {
        let popped = Some(self.graph.get(0).unwrap().clone());
        self.graph.remove(0);
        popped
    }
    pub fn pop_back(&mut self) -> Option<Vec<Vec<u8>>> {
        self.graph.pop()
    }

    pub fn get(&self) -> &Vec<Vec<Vec<u8>>> {
        &self.graph
    }

    pub fn new_graph(graph: Vec<Vec<Vec<u8>>>) -> List {
        List { graph }
    }
    pub fn new_one(element: Vec<Vec<u8>>) -> List {
        List {
            graph: vec![element],
        }
    }
    pub fn new() -> List {
        List { graph: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn exist(&self, element: &Vec<Vec<u8>>) -> bool {
        self.graph.contains(&element)
    }
}
