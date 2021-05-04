use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

use latex_gen::{LatexEngine, TractResult};
// trait Trait {
//     fn print_hello(&self);
// }

// impl Trait for i32 {
//     fn print_hello(&self) {
//         println!("hello");
//     }
// }

// fn returns_a_trait_object() -> impl Trait {
//     5
// }

fn main() -> TractResult<()> {
    // let result = returns_a_trait_object();
    // result.print_hello();
    Ok(())
}
fn test_part<F: AsRef<Path>>(path: F) -> TractResult<()> {
    let mut engine = LatexEngine::new();

    let mut result = engine.parse_from_path(path)?;
    let parse_result = engine.gen_back_total(&mut result, (9, 4), Some(1));
    if parse_result.is_ok() {
        for i in 0..result.symbol_map.len() {
            println!("backward: {}", result.get_node_backward(i));
        }
    } else {
        println!("message: {:?}", parse_result.err());
    }
    // println!("{}", result.gen_json());

    // println!("{}",result.gen_json());
    Ok(())
}

fn test_info<F: AsRef<Path>>(path: F) -> TractResult<()> {
    // let model = tract_onnx::onnx()
    //     // load the model
    //     .model_for_path(path)?
    //     // specify input type and shape
    //     // optimize the model
    //     // make the model runnable and fix its inputs and outputs
    //     .into_runnable()?;
    // // let mm = model.model();
    // // println!("input shape{}",mm.node(0).)

    // for n in model.model().nodes(){
    //     let op_name=n.op().name();
    //     let node_name= n.name.clone();
    //     println!("id: {}",n.id);
    //     println!("op options {:?}",n.op());
    //     println!("inputs: ");
    //     for i in n.inputs.iter(){
    //         print!(" {:?}",i);
    //         let fact=model.model().outlet_fact(*i).unwrap();
    //         println!("shape: {:?}",fact.shape.clone());
    //         println!("value: {:?}",fact.value.clone());

    //     }
    //     for i in n.outputs.iter(){
    //         println!("out test:{:?}",i.fact.shape);
    //         for j in i.successors.iter(){
    //             println!("output: {:?}",j);
    //         }

    //     }
    //     println!("node name: {}", node_name);
    //     println!("op name: {}", op_name);
    //     println!();
    // }
    Ok(())
}

#[test]
fn test_two_info() -> TractResult<()> {
    test_info("test_models/l2.onnx")
}

#[test]
fn test_two_layer() -> TractResult<()> {
    test_part("test_models/l2.onnx")
}

#[test]
fn test_three_layer() -> TractResult<()> {
    test_part("test_models/l3s.onnx")
}

#[test]
fn test_three_layer_file() -> TractResult<()> {
    Ok(())
}

#[test]
fn test_serde() -> TractResult<()>{
    let l = latex_gen::parse_proto("test_models/l3s.onnx").unwrap();
    println!("{}",l);
    Ok(())
    // latex_gen::into_proto(l)
}
