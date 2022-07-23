use crate::computational_graph::ComputeGraph;
use crate::node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}

pub fn is_valid(cache: Rc<RefCell<Option<f32>>>) -> bool {
    match *cache.borrow() {
        Some(_) => true,
        None => false,
    }
}

pub fn traverse(input_node: &Node) {
    let node = input_node.node.borrow();
    println!(
        "nodename = {}, is valid = {}",
        node.get_name(),
        is_valid(node.get_cache())
    );
    for child in (*node).get_childs() {
        traverse(&Node::new(child.clone()));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn is_valid(cache: Rc<RefCell<Option<f32>>>) -> bool {
        match *cache.borrow() {
            Some(_) => true,
            None => false,
        }
    }

    #[test]
    fn traverse_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let x3 = create_input("x3");
        // graph variable is the output node of the graph:
        let graph = add(
            x1.clone(),
            add(x2.clone(), add(x2.clone(), add(x2.clone(), x3.clone()))),
        );
        traverse(&graph);
        println!();
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), false);
        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), false);
        graph.compute();
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), true);
        traverse(&graph);
        println!();
        x1.set(3f32);
        traverse(&graph);
        println!();
        assert_eq!(is_valid((*graph.get_node().borrow()).get_cache()), false);
    }
}
