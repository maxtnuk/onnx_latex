use tract_ndarray::Array;
use tract_onnx::prelude::*;
use tract_onnx::latex_tool::*;
use rand::prelude::*;

fn main() -> TractResult<()> {
    let model = tract_onnx::onnx()
        // load the model
        .model_for_path("l2s.onnx")?
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
        println!("form: {}",result.get_node_formul(i));
    }
    println!("{:?}",result.senario);

    // for n in model.model().nodes(){
    //     let op_name=n.op().name();
    //     let node_name= n.name.clone();
    //     println!("id: {}",n.id);
    //     println!("inputs: ");
    //     for i in n.inputs.iter(){
    //         print!(" {:?}",i);
    //         let fact=model.model().outlet_fact(*i).unwrap();
    //         println!("shape: {:?}",fact.shape.clone());
    //         println!("value: {:?}",fact.value.clone());
    //     }   
    //     println!();
    //     println!("node name: {}", node_name);
    //     println!("op name: {}", op_name);
    //     println!();
    // }
    Ok(())
}
