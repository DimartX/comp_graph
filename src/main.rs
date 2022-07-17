mod computational_graph;
use computational_graph::{log, sin, ComputeNode, Input}; // imports Input, +, *, sin, log

// round to precision decimal digits
fn round(x: f32, precision: u32) -> f32 {
    let m = 10i32.pow(precision) as f32;
    (x * m).round() / m
}

fn main() {
    // let mut holder: Vec<RefCell<f32>> = Vec::new();
    // holder.push(RefCell::new(0f32));
    // holder.push(RefCell::new(0f32));
    // holder.push(RefCell::new(0f32));

    // x1, x2, x3 implement trait ComputeNode
    let x1: Input = Input::new();
    let x2 = Input::new();
    let x3 = Input::new();

    // sin and log are structs which implement the trait ComputeNode.
    // + and * are opps which take two ComputeNodes and produce a single ComputeNode.
    let output = x1.clone() + x2.clone() * sin::new(x2.clone() + log::new(x1.clone(), 2.0f32));

    x1.set(1f32);
    x2.set(2f32);
    x3.set(3f32);
    let mut result = output.compute();
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), 2.8186);

    x1.set(2f32);
    x2.set(3f32);
    x3.set(4f32);
    result = output.compute();
    println!("Graph output = {}", result);
    assert_eq!(round(result, 5), -0.27041);
}
