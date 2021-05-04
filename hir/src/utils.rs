use tract_core::ops::{array::{Gather, Pad}, cnn::{MaxPool, PoolSpec, SumPool}, dummy::Dummy, element_wise::ElementWiseOp, identity::Identity, konst::Const, logic::Iff, math::{Add, Max, Min}, unimpl::UnimplementedOp};
use serde::Deserialize;
use serde::Serialize;

use crate::ops::{array::{ConstantLike, EyeLike}, binary::Nary, expandable::Expansion};

pub trait MathGen {
    fn gen_forward(&self,idx: usize)->String{
        "".to_string()
    }
    fn gen_forward_value(&self, idx:usize , inputs:Vec<String>) ->String{
        "".to_string()
    }
    fn gen_backward(&self, idx: usize)->String{
        "".to_string()
    }
    fn gen_backward_value(&self, idx:usize , inputs:Vec<String>) ->String{
        "".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum FormulKind {
    Activation,
    Function,
    Base,
    Not,
}

pub fn gen_symbol(symbol: Option<String>,n_type: FormulKind,idx: usize) ->String{
    match n_type{
        FormulKind::Activation => {
            format!("h_{}", idx)
        }
        FormulKind::Function => {
            format!("f_{}", idx)
        }
        _=> {
            if let Some(s)= symbol{
                format!("{}_{}",s,idx)
            }else{
                "".to_string()
            }
        }
    }
}

macro_rules! empty_mathgen {
    ($struct:ident) => {
        impl crate::utils::MathGen for $struct {
            fn gen_forward(&self, idx: usize)->String{
                "".to_string()
            }
            fn gen_forward_value(&self, idx:usize , inputs:Vec<String>) ->String{
                "".to_string()
            }
            fn gen_backward(&self, idx: usize)->String{
                "".to_string()
            }
            fn gen_backward_value(&self, idx:usize , inputs:Vec<String>) ->String{
                "".to_string()
            }
        }
    };
}

empty_mathgen!(Dummy);
impl MathGen for ElementWiseOp{}
impl MathGen for Pad{}
impl MathGen for Const{}
impl MathGen for ConstantLike{}
impl MathGen for EyeLike{}
impl MathGen for Gather{}
impl MathGen for SumPool{}
impl MathGen for PoolSpec{}
impl MathGen for MaxPool{}
impl MathGen for UnimplementedOp{}
impl MathGen for Iff{}
impl MathGen for Nary{}
impl MathGen for Identity{}

