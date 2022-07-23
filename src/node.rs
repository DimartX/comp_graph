use crate::computational_graph::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node {
    pub node: Rc<RefCell<NodeImpl>>,
}

impl Node {
    pub fn new(node: Rc<RefCell<NodeImpl>>) -> Node {
        Node { node: node }
    }

    pub fn set(&self, value: f32) {
        self.node.borrow().set(value);
    }

    pub fn get_node(&self) -> Rc<RefCell<NodeImpl>> {
        self.node.clone()
    }
}

impl ComputeGraph for Node {
    fn compute(&self) -> f32 {
        self.node.borrow().compute()
    }
}

pub fn create_input(name: &'static str) -> Node {
    Node::new(Rc::new(RefCell::new(NodeImpl::new(
        name,
        Vec::new(),
        vec![0f32],
        move |_nodes: &[Rc<RefCell<NodeImpl>>], additional: &[f32]| -> f32 { additional[0] },
    ))))
}

pub fn add(lhs: Node, rhs: Node) -> Node {
    let new_node = Rc::new(RefCell::new(NodeImpl::new(
        "add",
        vec![lhs.node.clone(), rhs.node.clone()],
        Vec::new(),
        move |nodes: &[Rc<RefCell<NodeImpl>>], _additional: &[f32]| -> f32 {
            nodes[0].borrow().compute() + nodes[1].borrow().compute()
        },
    )));
    lhs.node.borrow_mut().parents.push(new_node.clone());
    rhs.node.borrow_mut().parents.push(new_node.clone());
    Node::new(new_node)
}

pub fn mul(lhs: Node, rhs: Node) -> Node {
    let new_node = Rc::new(RefCell::new(NodeImpl::new(
        "mul",
        vec![lhs.node.clone(), rhs.node.clone()],
        Vec::new(),
        move |nodes: &[Rc<RefCell<NodeImpl>>], _additional: &[f32]| -> f32 {
            nodes[0].borrow().compute() * nodes[1].borrow().compute()
        },
    )));
    lhs.node.borrow_mut().parents.push(new_node.clone());
    rhs.node.borrow_mut().parents.push(new_node.clone());
    Node::new(new_node)
}

pub fn sin(node: Node) -> Node {
    let new_node = Rc::new(RefCell::new(NodeImpl::new(
        "sin",
        vec![node.node.clone()],
        Vec::new(),
        move |nodes: &[Rc<RefCell<NodeImpl>>], _additional: &[f32]| -> f32 {
            nodes[0].borrow().compute().sin()
        },
    )));
    node.node.borrow_mut().parents.push(new_node.clone());
    Node::new(new_node)
}

pub fn pow_f32(node: Node, degree: f32) -> Node {
    let new_node = Rc::new(RefCell::new(NodeImpl::new(
        "pow_f32",
        vec![node.node.clone()],
        vec![degree],
        move |nodes: &[Rc<RefCell<NodeImpl>>], additional: &[f32]| -> f32 {
            nodes[0].borrow().compute().powf(additional[0])
        },
    )));
    node.node.borrow_mut().parents.push(new_node.clone());
    Node::new(new_node)
}

#[cfg(test)]
mod test {
    use super::*;
    fn round(x: f32, precision: u32) -> f32 {
        let m = 10i32.pow(precision) as f32;
        (x * m).round() / m
    }
    #[test]
    fn input_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        x1.set(1f32);
        x2.set(2f32);
        assert_eq!(1f32, x1.compute());
        assert_eq!(2f32, x2.compute());
    }
    #[test]
    fn simple_add_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let out = add(x1.clone(), x2.clone());
        x1.set(1f32);
        x2.set(2f32);
        let res = out.compute();
        println!("{}", res);
        assert_eq!(3f32, res);
    }
    #[test]
    fn add_in_add_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let out = add(x1.clone(), add(x1.clone(), x2.clone()));
        x1.set(1f32);
        x2.set(2f32);
        let res = out.compute();
        println!("{}", res);
        assert_eq!(4f32, res);
    }
    #[test]
    fn simple_mul_test() {
        let x1 = create_input("x1");
        let x2 = create_input("x2");
        let out = mul(x1.clone(), x2.clone());
        x1.set(4f32);
        x2.set(2f32);
        let res = out.compute();
        println!("{}", res);
        assert_eq!(8f32, res);
    }
    #[test]
    fn simple_sin_test() {
        let x1 = create_input("x1");
        let out = sin(x1.clone());
        x1.set(4f32);
        let res = out.compute();
        println!("{}", res);
        assert_eq!(round(res, 5), -0.7568f32);
    }
    #[test]
    fn simple_pow_test() {
        let x1 = create_input("x1");
        let degree = 2.5f32;
        let out = pow_f32(x1.clone(), degree);
        x1.set(4f32);
        let res = out.compute();
        println!("{}", res);
        assert_eq!(round(res, 5), 32f32);
    }
}
