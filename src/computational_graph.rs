use std::cell::RefCell;
use std::ops::{Add, Mul};
use std::rc::Rc;

pub trait ComputeNode {
    fn compute(&self) -> f32;
}

#[derive(Clone)]
pub struct Input {
    value: Rc<RefCell<f32>>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            value: Rc::new(RefCell::new(0f32)),
        }
    }

    pub fn set(&self, value: f32) {
        let mut mut_val = self.value.borrow_mut();
        *mut_val = value;
    }
}

impl<'a> ComputeNode for Input {
    fn compute(&self) -> f32 {
        let val = self.value.borrow();
        *val
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct sin<T: ComputeNode> {
    node: Box<T>,
}

impl<T: ComputeNode> sin<T> {
    pub fn new(node: T) -> Self {
        Self {
            node: Box::new(node),
        }
    }
}

impl<T: ComputeNode> ComputeNode for sin<T> {
    fn compute(&self) -> f32 {
        f32::sin(self.node.compute())
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct log<T: ComputeNode> {
    node: Box<T>,
    degree: f32,
}

impl<T: ComputeNode> log<T> {
    pub fn new(node: T, degree: f32) -> Self {
        Self {
            node: Box::new(node),
            degree: degree,
        }
    }
}

impl<T: ComputeNode> ComputeNode for log<T> {
    fn compute(&self) -> f32 {
        f32::log(self.node.compute(), self.degree)
    }
}

#[derive(Clone)]
pub struct ANode<T, U>
where
    T: ComputeNode,
    U: ComputeNode,
{
    lhs: Box<T>,
    rhs: Box<U>,
    op: fn(f32, f32) -> f32,
}

impl<T, U> ComputeNode for ANode<T, U>
where
    T: ComputeNode,
    U: ComputeNode,
{
    fn compute(&self) -> f32 {
        (self.op)(self.lhs.compute(), self.rhs.compute())
    }
}

impl<R> Add<R> for Input
where
    R: ComputeNode + Clone,
{
    type Output = ANode<Input, R>;

    fn add(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other),
            op: move |x: f32, y: f32| -> f32 { x + y },
        }
    }
}

impl<R, T, U> Add<R> for ANode<T, U>
where
    R: ComputeNode + Clone,
    T: ComputeNode,
    U: ComputeNode,
{
    type Output = ANode<ANode<T, U>, R>;

    fn add(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x + y },
        }
    }
}

impl<R, T> Add<R> for sin<T>
where
    R: ComputeNode + Clone,
    T: ComputeNode,
{
    type Output = ANode<sin<T>, R>;

    fn add(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x + y },
        }
    }
}

impl<R, T> Add<R> for log<T>
where
    R: ComputeNode + Clone,
    T: ComputeNode,
{
    type Output = ANode<log<T>, R>;

    fn add(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x + y },
        }
    }
}

impl<R> Mul<R> for Input
where
    R: ComputeNode + Clone,
{
    type Output = ANode<Input, R>;

    fn mul(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self.clone()),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x * y },
        }
    }
}
impl<R, T, U> Mul<R> for ANode<T, U>
where
    R: ComputeNode + Clone,
    T: ComputeNode,
    U: ComputeNode,
{
    type Output = ANode<ANode<T, U>, R>;

    fn mul(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x * y },
        }
    }
}

impl<R, T> Mul<R> for sin<T>
where
    R: ComputeNode + Clone,
    T: ComputeNode,
{
    type Output = ANode<sin<T>, R>;

    fn mul(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x * y },
        }
    }
}

impl<R, T> Mul<R> for log<T>
where
    R: ComputeNode + Clone,
    T: ComputeNode,
{
    type Output = ANode<log<T>, R>;

    fn mul(self, other: R) -> Self::Output {
        ANode {
            lhs: Box::new(self),
            rhs: Box::new(other.clone()),
            op: move |x: f32, y: f32| -> f32 { x * y },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum() {
        let x1: Input = Input::new();
        let x2 = Input::new();
        // let mut x3 = Input::new();
        x1.set(-1f32);
        x2.set(2f32);
        let output =
            x1.clone() + x2.clone() + x1.clone() * (x1.clone() + x2.clone()) + sin::new(x1.clone());
        x1.set(2f32);
        x2.set(2f32);
        // x3.set(3f32);
        let result = output.compute();
        println!("Prev operation res = {}", result);
        x1.set(100f32);

        let res2 = (x1.clone() * x1.clone()).compute();
        println!("kek = {}", res2);
        let result = output.compute();
        println!("This operation res = {}", result);
        // assert_eq!(result, 3.0);
    }
}
