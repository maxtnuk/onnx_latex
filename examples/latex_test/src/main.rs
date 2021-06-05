use std::path::Path;

use latex_gen::{Indexes, LatexEngine, TractResult};

fn main() -> TractResult<()> {
    // let result = returns_a_trait_object();
    // result.print_hello();
    Ok(())
}
fn test_part<F: AsRef<Path>>(path: F) -> TractResult<()> {
    let mut engine = LatexEngine::new();

    let _result = engine.parse_from_path(path.as_ref(), Some(4))?;
    let _proto = latex_gen::parse_proto(path)?;
    //  println!("{:?}", result.senario);
    let _indexes = Indexes::new(vec![10, 20, 30], vec![3, 4, 5]);
    // let parse_result = engine.gen_back_total(&mut result, &proto, indexes, Some(3));
    // if parse_result.is_ok() {
    //     for i in 0..result.symbol_map.len() {
    //         println!(
    //             "{}: {}",
    //             result.symbol_map[i].as_ref().unwrap().symbol.clone(),
    //             result.get_node_backward(i)
    //         );
    //     }
    // } else {
    //     println!("message: {:?}", parse_result.err());
    // }
    // for n in result.symbol_map.iter() {
    //     println!("forward: {}", n.clone().unwrap().forward_value);
    // }
    // let mut file = File::create("vgg.txt")?;
    // file.write_all(result.gen_map_json().as_bytes())?;
    // println!("{}", result.gen_json());

    // println!("{}",result.gen_json());
    Ok(())
}

fn test_info<F: AsRef<Path>>(_path: F) -> TractResult<()> {
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
    latex_gen::model_info("test_models/l2.onnx")
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
fn test_cnn() -> TractResult<()> {
    test_part("test_models/lcnn.onnx")
}

#[test]
fn test_cnn_info() -> TractResult<()> {
    latex_gen::model_info("test_models/lcnn.onnx")
}

#[test]
fn test_vgg_info() -> TractResult<()> {
    latex_gen::model_info("test_models/lvgg.onnx")
}
#[test]
fn test_vgg() -> TractResult<()> {
    test_part("test_models/lvgg.onnx")
}

#[test]
fn test_serde() -> TractResult<()> {
    Ok(())
    // latex_gen::into_proto(l)
}
#[test]
fn test_cnn_part() -> TractResult<()> {
    test_part("test_models/lcnn.onnx")
}
