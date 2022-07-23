use crate::node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ComputeGraph {
    fn compute(&self) -> f32;
}

trait InvalidateGraph {
    fn invalidate(&self);
}

pub struct NodeImpl {
    name: &'static str,
    cache: Rc<RefCell<Option<f32>>>,
    nodes: Vec<Rc<RefCell<NodeImpl>>>,
    pub parents: Vec<Rc<RefCell<NodeImpl>>>,
    additional: Rc<RefCell<Vec<f32>>>,
    op: fn(&[Rc<RefCell<NodeImpl>>], &[f32]) -> f32,
}

impl NodeImpl {
    pub fn new(
        name: &'static str,
        nodes: Vec<Rc<RefCell<NodeImpl>>>,
        additional: Vec<f32>,
        op: fn(&[Rc<RefCell<NodeImpl>>], &[f32]) -> f32,
    ) -> NodeImpl {
        NodeImpl {
            name: name,
            cache: Rc::new(RefCell::new(None)),
            nodes: nodes,
            parents: Vec::new(),
            additional: Rc::new(RefCell::new(additional)),
            op: op,
        }
    }

    pub fn set(&self, value: f32) {
        let mut additional = self.additional.borrow_mut();
        additional[0] = value;

        self.invalidate();
    }

    pub fn get_childs(&self) -> &Vec<Rc<RefCell<NodeImpl>>> {
        &self.nodes
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_cache(&self) -> Rc<RefCell<Option<f32>>> {
        self.cache.clone()
    }
}

impl InvalidateGraph for NodeImpl {
    fn invalidate(&self) {
        let mut cache = self.cache.borrow_mut();
        match *cache {
            Some(_) => {
                *cache = None;
                for parent in &self.parents {
                    parent.borrow().invalidate();
                }
            }
            None => (),
        }
    }
}

impl ComputeGraph for NodeImpl {
    fn compute(&self) -> f32 {
        if (*self.cache.borrow()) == None {
            (*self.cache.borrow_mut()) = Some((self.op)(&self.nodes, &self.additional.borrow()));
        }
        match *self.cache.borrow() {
            Some(val) => val,
            None => 0f32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::{is_valid, round};

    #[test]
    fn invalidate_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let x3 = create_input("x3");
        // graph variable is the output node of the graph:
        let graph = add(
            x1.clone(),
            mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
        );
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), false);
        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), false);
        graph.compute();
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), true);
        x1.set(3f32);
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), false);
    }

    #[test]
    fn compute_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let x3 = create_input("x3");
        // graph variable is the output node of the graph:
        let graph = add(
            x1.clone(),
            mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
        );
        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);
        let mut result = graph.compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.32727);
        x1.set(2f32);
        x2.set(3f32);
        x3.set(4f32);
        result = graph.compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.56656);
    }
}
