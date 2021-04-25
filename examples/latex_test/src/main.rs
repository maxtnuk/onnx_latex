use std::path::Path;

use tract_ndarray::Array;
use tract_onnx::prelude::*;
use tract_onnx::latex_tool::*;
use rand::prelude::*;

fn main() -> TractResult<()> {
    Ok(())
}
fn test_part<F: AsRef<Path>>(path: F)->TractResult<()>{
    let model = tract_onnx::onnx()
    // load the model
    .model_for_path(path)?
    // specify input type and shap
    // optimize the model
    // make the model runnable and fix its inputs and outputs
    .into_runnable()?;
    let mut rng = thread_rng();
    let vals: Vec<_> = (0..64000).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape((64, 1000)).unwrap();
    let mut engine=LatexEngine::new();

    let result=engine.parse_plan(&model, tvec![input.into()],ParseMode::Full);

    for i in 0..model.model().nodes.len(){
        println!("backward: {}",result.get_node_backward(i));
    }
     // println!("{}",result.gen_json());
    Ok(())
}
fn test_info<F: AsRef<Path>>(path: F)->TractResult<()>{
    let model = tract_onnx::onnx()
        // load the model
        .model_for_path(path)?
        // specify input type and shape
        // optimize the model
        // make the model runnable and fix its inputs and outputs
        .into_runnable()?;

    for n in model.model().nodes(){
        let op_name=n.op().name();
        let node_name= n.name.clone();
        println!("id: {}",n.id);
        println!("op options {:?}",n.op());
        println!("inputs: ");
        for i in n.inputs.iter(){
            print!(" {:?}",i);
            let fact=model.model().outlet_fact(*i).unwrap();
            println!("shape: {:?}",fact.shape.clone());
            println!("value: {:?}",fact.value.clone());
            
        }
        for i in n.outputs.iter(){
            for j in i.successors.iter(){
                println!("output: {:?}",j); 
            }
              
        }
        println!("node name: {}", node_name);
        println!("op name: {}", op_name);
        println!();
    }
    Ok(())
}


#[test]
fn test_two_info()->TractResult<()>{
    test_info("test_models/l2.onnx")
}

#[test]
fn test_two_layer()-> TractResult<()>{
    test_part("test_models/l2s.onnx")
}

#[test]
fn test_three_layer()-> TractResult<()>{
    test_part("test_models/l3s.onnx")
}

