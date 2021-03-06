mod compress;
mod nonzero;
mod one_hot;
mod pad;
mod slice;

use tract_hir::internal::*;
use tract_hir::ops::array;

use crate::model::{OnnxOpRegister, ParsingContext};
use crate::pb::*;

pub fn register_all_ops(reg: &mut OnnxOpRegister) {
    reg.insert("Compress", compress::compress);
    reg.insert("Concat", concat);
    reg.insert("ConstantLike", constant_like);
    reg.insert("ConstantOfShape", constant_of_shape);
    reg.insert("Expand", |_, _| {
        Ok((expand(array::MultiBroadcastTo::default()), vec![]))
    });
    reg.insert("EyeLike", eye_like);
    reg.insert("Flatten", flatten);
    reg.insert("Gather", gather);
    reg.insert("NonZero", |_, _| Ok((Box::new(nonzero::NonZero), vec![])));
    reg.insert("OneHot", one_hot::one_hot);
    reg.insert("Pad", pad::pad);
    reg.insert("Reshape", |_, _| {
        Ok((expand(array::Reshape::default()), vec![]))
    });
    reg.insert("Shape", |_, _| {
        Ok((expand(array::Shape::new(DatumType::I64)), vec![]))
    });
    reg.insert("Size", |_, _| {
        Ok((expand(array::Size::new(DatumType::I64)), vec![]))
    });
    reg.insert("Transpose", transpose);
    reg.insert("Tile", |_, _| Ok((expand(array::Tile::default()), vec![])));
    reg.insert("Slice", slice::slice);
    reg.insert("Split", split);
    reg.insert("Squeeze", squeeze);
    reg.insert("Unsqueeze", unsqueeze);
}

pub fn concat(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let axis = node.get_attr("axis")?;
    Ok((expand(array::Concat::new(axis)), vec![]))
}

pub fn constant_like(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let value = node.get_attr_opt("value")?.unwrap_or(0.);
    if node.input.len() == 0 {
        let dt = node.get_attr_opt("dtype")?.unwrap_or(f32::datum_type());
        let shape: Vec<usize> = node.get_attr_vec("shape")?;
        let tensor = tensor0(value)
            .cast_to_dt(dt)?
            .broadcast_scalar_to_shape(&*shape)?
            .into_arc_tensor();
        Ok((Box::new(tract_hir::ops::konst::Const::new(tensor)), vec![]))
    } else {
        Ok((Box::new(array::ConstantLike::new(value)), vec![]))
    }
}

pub fn constant_of_shape(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let mut value = match node.get_attr_opt::<Tensor>("value")? {
        Some(val) => val.into_arc_tensor(),
        None => rctensor0(0.0),
    };
    if value.rank() > 0 {
        if value.len() != 1 {
            bail!("Expected scalar (or vector of length 1), got {:?}", value);
        }
        value = value.nth(0)?.into_arc_tensor();
    }
    Ok((expand(array::ConstantOfShape::new(value)), vec![]))
}

pub fn eye_like(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let dt = node.get_attr_opt("dtype")?;
    let k = node.get_attr_opt("k")?.unwrap_or(0);
    Ok((Box::new(array::EyeLike::new(dt, k)), vec![]))
}

pub fn flatten(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let axis = node.get_attr_opt("axis")?.unwrap_or(1);
    Ok((expand(array::Flatten::new(axis)), vec![]))
}

pub fn gather(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let axis = node.get_attr_opt("axis")?.unwrap_or(0);
    Ok((expand(array::Gather::new(axis)), vec![]))
}

pub fn split(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let axis = node.get_attr_opt("axis")?.unwrap_or(0);
    let split = node.get_attr_opt_vec("split")?;
    Ok((
        expand(array::Split::new(axis, node.output.len(), split)),
        vec![],
    ))
}

pub fn squeeze(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let axes = node.get_attr_opt_vec("axes")?;
    Ok((expand(array::Squeeze::new(axes)), vec![]))
}

pub fn transpose(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let perm = node.get_attr_opt_vec("perm")?;
    Ok((
        expand(array::PermuteAxes::new(perm.map(|t| t.into()))),
        vec![],
    ))
}

pub fn unsqueeze(
    _ctx: &ParsingContext,
    node: &NodeProto,
) -> TractResult<(Box<dyn InferenceOp>, Vec<String>)> {
    let axes = node
        .get_attr_vec::<i64>("axes")?
        .into_iter()
        .map(|x| x as isize)
        .collect();
    Ok((expand(array::AddDims::new(axes)), vec![]))
}
