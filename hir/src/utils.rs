use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use tract_core::{
    dyn_clone,
    internal::ElementWiseMiniOp,
    model::Op,
    ops::{
        array::{Gather, Pad},
        cnn::{MaxPool, PoolSpec, SumPool},
        dummy::Dummy,
        element_wise::ElementWiseOp,
        identity::Identity,
        konst::Const,
        logic::Iff,
        math::{Add, Max, Min},
        nn::Sigmoid,
        unimpl::UnimplementedOp,
    },
};

use crate::ops::{
    array::{ConstantLike, EyeLike},
    binary::Nary,
    expandable::Expansion,
};

pub trait MathGen {
    fn get_original_type(&self) -> FormulKind {
        let result = FormulKind::Undefined;
        // println!("default called: {:?}",result);
        result
    }
    fn get_symbol_type(&self, extra_symbol: Option<String>) -> FormulKind {
        match extra_symbol {
            Some(ref s) if get_extra_symbol(s.clone()) != FormulKind::Undefined => {
                get_extra_symbol(s.clone())
            }
            _ => self.get_original_type(),
        }
    }
    fn gen_forward(&self, extra_symbol: Option<String>, idx: usize) -> String {
        let kind = self.get_symbol_type(extra_symbol.clone());
        gen_symbol(extra_symbol, kind, idx)
    }
    fn gen_forward_value(&self, inputs: Vec<String>,input_shape: Option<Vec<usize>>,output_shape: Option<Vec<usize>>) -> String {
        "".to_string()
    }
    fn gen_backward(&self, upper: String, under: String) -> String {
        format!(r#"\frac{{\partial {}}}{{\partial {}}}"#, upper, under)
    }
    fn gen_backward_value(&self, inputs: Vec<String>) -> Option<String> {
        None
    }
    fn attributes(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub enum FormulKind {
    Activation,
    Function,
    Base,
    Not,
    Undefined,
    Bias,
    Weight,
    Input,
    Cnn,
    Const,
}
fn get_extra_symbol(original: String) -> FormulKind {
    match original.as_str() {
        "weight" => FormulKind::Weight,
        "bias" => FormulKind::Bias,
        "Source" => FormulKind::Input,
        _ => FormulKind::Undefined,
    }
}
pub fn is_weightable(form: FormulKind)-> Option<FormulKind>{
    match form{
        FormulKind::Function | FormulKind::Cnn =>{
            Some(form)
        }
        _ => {
            None
        }
    }
}

pub fn gen_symbol(symbol: Option<String>, n_type: FormulKind, idx: usize) -> String {
    match n_type {
        FormulKind::Activation => {
            format!("h_{}", idx)
        }
        FormulKind::Function => {
            format!("f_{}", idx)
        }
        FormulKind::Weight => {
            format!(r#"\overline{{W_{}}}"#, idx)
        }
        FormulKind::Bias => {
            format!(r#"\overline{{B_{}}}"#, idx)
        }
        FormulKind::Input => {
            format!(r#"\overline{{Input}}"#)
        }
        FormulKind::Cnn => {
            format!(r#"Cnn_{}"#,idx)
        }
        _ => {
            if let Some(s) = symbol {
                format!("{}_{}", s, idx)
            } else {
                "Undefined".to_string()
            }
        }
    }
}
pub fn mathgen_op<T: Op + MathGen + Clone>(op: &dyn Op) -> Option<Box<dyn MathGen>> {
    op.downcast_ref::<T>()
        .map(|s| Box::new(s.clone()) as Box<dyn MathGen>)
}
pub fn mathgen_ele_op<T: ElementWiseMiniOp + MathGen + Clone>(
    op: &dyn ElementWiseMiniOp,
) -> Option<Box<dyn MathGen>> {
    op.downcast_ref::<T>()
        .map(|s| Box::new(s.clone()) as Box<dyn MathGen>)
}

impl MathGen for Dummy {}
impl MathGen for ElementWiseOp {}
impl MathGen for Pad {}
impl MathGen for Const {
    fn get_original_type(&self) -> FormulKind {
        FormulKind::Const
    }
}

impl MathGen for ConstantLike {}
impl MathGen for EyeLike {}
impl MathGen for Gather {}
impl MathGen for SumPool {}
impl MathGen for PoolSpec {}
impl MathGen for MaxPool {}
impl MathGen for UnimplementedOp {}
impl MathGen for Iff {}
impl MathGen for Nary {}
impl MathGen for Identity {}

// elemini section
impl MathGen for Sigmoid {
    fn get_original_type(&self) -> FormulKind {
        FormulKind::Activation
    }
    fn gen_forward_value(&self, inputs: Vec<String>,input_shape: Option<Vec<usize>>,output_shape: Option<Vec<usize>>) -> String {
        format!(r#"\frac{{1}}{{1+e^{{-({})}}}}"#, inputs[0])
    }
}
