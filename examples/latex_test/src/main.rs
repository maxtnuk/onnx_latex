use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
};

use latex_gen::{LatexEngine, TractResult};

fn main() -> TractResult<()> {
    // let result = returns_a_trait_object();
    // result.print_hello();
    Ok(())
}
fn test_part<F: AsRef<Path>>(path: F) -> TractResult<()> {
    let mut engine = LatexEngine::new();

    let mut result = engine.parse_from_path(path)?;
    // let parse_result = engine.gen_back_total(&mut result, (9, 4), Some(1));
    // if parse_result.is_ok() {
    //     for i in 0..result.symbol_map.len() {
    //         println!("backward: {}", result.get_node_backward(i));
    //     }
    // } else {
    //     println!("message: {:?}", parse_result.err());
    // }
    println!("{}", result.gen_json());

    // println!("{}",result.gen_json());
    Ok(())
}

#[test]
fn test_two_info() -> TractResult<()> {
    latex_gen::model_info("test_models/l3s.onnx")
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
fn test_cnn_info() -> TractResult<()> {
    latex_gen::model_info("test_models/lcnn.onnx")
}
#[test]
fn test_cnn_part() -> TractResult<()> {
    test_part("test_models/lcnn.onnx")
}