use tract_onnx::{
    pb::ModelProto,
    tract_core::ops::nn::Sigmoid,
    tract_hir::{
        infer::GenericFactoid,
        internal::{Expansion, OpState, SessionState},
        ops::{
            array::Pad,
            cnn::{MaxPool, SumPool},
            dummy::Dummy,
            element_wise::ElementWiseOp,
            konst::Const,
            source::Source,
        },
        utils::{is_weightable, mathgen_ele_op, FormulKind, MathGen},
    },
    Onnx,
};

use std::{fmt::Debug, io::ErrorKind};
use std::{fmt::Display, io::Read, path::Path};
use std::{hash::Hash, time::Instant};
use tract_onnx::tract_hir::utils::mathgen_op;
use tract_onnx::{prelude::*, tract_hir::infer::InferenceOp};

use crate::parse_struct::{except_self_symbol_parts, only_inputs_symbol_parts};
pub use tract_onnx::prelude::TractResult;

use self::{
    node_info::{Formul, FormulNode},
    parse_struct::{insert_symbol_parts, symbol_split, DebugValue},
};

use serde::{Deserialize, Serialize};

mod node_info;
mod parse_struct;

type InferenceNode = Node<InferenceFact, Box<dyn InferenceOp>>;

type InferencePlan =
    SimplePlan<InferenceFact, Box<dyn InferenceOp>, Graph<InferenceFact, Box<dyn InferenceOp>>>;

// get value from layers 
pub fn eval<F, O>(
    session_state: &mut SessionState,
    mut state: Option<&mut (dyn OpState + 'static)>,
    node: &Node<F, O>,
    input: TVec<Arc<Tensor>>,
) -> TractResult<TVec<Arc<Tensor>>>
where
    F: Fact + Hash + Clone + 'static,
    O: Debug + Display + AsRef<dyn Op> + AsMut<dyn Op> + Clone + 'static + Hash,
{
    let r = match state {
        Some(ref mut state) => state.eval(session_state, node.op(), input),
        None => node.op().eval(input),
    };
    r
}

// onnx parsing engine 
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LatexNode {
    pub index: usize,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub symbol: String,
    pub extra_symbol: Option<String>,
    pub forward_value: String,
    pub op_name: String,
    pub input_shape_ref: Option<Vec<usize>>,
    pub output_shape: Vec<usize>,
    pub backward_value: String,
    pub backward_symbol: String,
    pub description: String,
    pub op_attributes: DebugValue,
}
impl LatexNode {
    // erase prefix
    pub fn erase_slash(&mut self) {
        let r = |s: &String| -> String { s.replace(r#"\\"#, r#"\"#) };
        self.symbol = r(&self.symbol);
        self.forward_value = r(&self.forward_value);
        self.backward_symbol = r(&self.backward_symbol);
        self.backward_value = r(&self.backward_value);
    }
}

// predefined symbol library 
#[derive(Default, Clone)]
pub struct SymbolLibrary {
    pub func: Formul,
    pub etc: Formul,
    pub activation: Formul,
}

impl SymbolLibrary {
    // read from ron file 
    fn new() -> Self {
        let func_info = node_info::read_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/formuls/formul.ron"
        )))
        .expect("formul error");
        let etc_info = node_info::read_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/formuls/etc.ron"
        )))
        .expect("etc error");
        let activation_info = node_info::read_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/formuls/activation.ron"
        )))
        .expect("activation error");
        SymbolLibrary {
            func: func_info,
            etc: etc_info,
            activation: activation_info,
        }
    }
    // (symbol,form)
    pub fn get_symbol(&self, target: &str) -> Option<(String, FormulKind, FormulNode)> {
        let form = [&self.func, &self.etc, &self.activation];
        form.iter().filter_map(|x| x.gen_symbol(target).ok()).next()
    }
    // generate weight symbol 
    fn gen_w_symbol_inner(&self, target: String, indexes: &Indexes, deeper: bool) -> String {
        let (_, _, underform) = self.get_symbol("_Under").unwrap();
        let under_splits = symbol_split(underform.formul.as_str()).unwrap();
        let (_, _, weightform) = self.get_symbol("_Weight").unwrap();
        let weight_splits = symbol_split(weightform.formul.as_str()).unwrap();

        // func section
        let c = Self::continous_index(indexes.func_idx.iter().map(|x| x.to_string()).collect());
        let func_name = only_inputs_symbol_parts(under_splits.clone(), vec![target, c]);
        // weight_name
        let w_c = Self::continous_index(indexes.weight_idx.iter().map(|x| x.to_string()).collect());
        let between_symbol = only_inputs_symbol_parts(weight_splits.clone(), vec![func_name, w_c]);
        if deeper {
            only_inputs_symbol_parts(under_splits.clone(), vec!["w".to_string(), between_symbol])
        } else {
            between_symbol
        }
    }
    // generate error symbol 
    fn gen_error_symbol(&self, target: Vec<String>) -> String {
        let (e_symbol, _, _) = self.get_symbol("Error").unwrap();
        let splits = symbol_split(e_symbol.as_str()).unwrap();
        insert_symbol_parts(splits, target, Vec::new(), "".to_string())
    }
    // with index
    fn continous_index(proper_symbol: Vec<String>) -> String {
        let mut result = String::new();
        result += "(";
        for i in proper_symbol.iter() {
            result += &format!("{},", i);
        }
        result += ")";
        result
    }
    // gen proper symbol based on level
    fn get_p0p1(
        &self,
        level: usize,
        indexes: &Indexes,
        last_symbol: String,
        proper_symbol: &[String],
        many: usize,
    ) -> (String, String) {
        if level == 0 {
            // last symbol
            let p0_temp =
                Self::continous_index(indexes.func_idx.iter().map(|x| x.to_string()).collect());
            let p1_temp = self.gen_w_symbol_inner(last_symbol, indexes, false);
            (p0_temp, p1_temp)
        } else {
            let p0_num = level - 1;
            let p1_num = if level > 1 { level - 2 } else { 0 };

            let cf_func = |_x: &[String], n| {
                (0..many)
                    .map(|s| format!("{}n_{{{}}}", proper_symbol[s], n))
                    .collect()
            };
            (
                Self::continous_index(cf_func(proper_symbol, p0_num)),
                if level > 1 {
                    Self::continous_index(cf_func(proper_symbol, p1_num))
                } else {
                    Self::continous_index(indexes.func_idx.iter().map(|x| x.to_string()).collect())
                },
            )
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffChainNode {
    Weightable(usize, String),
    UnWeightable(usize, String),
    Sum(Box<DiffChainNode>, Vec<usize>),
    Chain(Vec<DiffChainNode>),
    Not,
}

#[derive(Default)]
pub struct LatexEngine {
    pub engine: Onnx,
    pub symbol_map: Vec<Option<LatexNode>>,
    pub weight_count: usize,
    pub bias_count: usize,
    pub const_count: usize,
    pub formul_count: usize,
    pub activation_count: usize,
    pub symbol_library: SymbolLibrary,
    pub math_op_vec: Vec<Option<Box<dyn MathGen>>>,
}
pub enum ErrorResultTo {
    Total,
    Innner(usize),
}
// search all op trait
macro_rules! each_op {
    ($op: ident,[$($x:ident),*]) => {
       {
        let mut inner:Vec<Option<Box<dyn MathGen>>>=Vec::new();
        $(
            let t = mathgen_op::<$x>($op);
            if let Some(e) = t{
                inner.push(Some(Box::new(e)));
            }else{
                inner.push(None);
            }
        )*
        inner
       }
    };
}
// search all element op 
macro_rules! each_ele_op {
    ($op: ident,[$($x:ident),*]) => {
       {
        let mut inner:Vec<Option<Box<dyn MathGen>>>=Vec::new();
        $(
            let t = mathgen_ele_op::<$x>($op);
            if let Some(e) = t{
                inner.push(Some(Box::new(e)));
            }else{
                inner.push(None);
            }
        )*
        inner
       }
    };
}

#[derive(Deserialize, Serialize)]
// model proto struct 
pub struct ParseModelResult {
    model: ModelProto,
    symbol: LatexResult,
}
impl ParseModelResult {
    pub fn new(model_proto: ModelProto, symbol: LatexResult) -> Self {
        ParseModelResult {
            model: model_proto,
            symbol: symbol,
        }
    }
    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
// from path
pub fn parse_proto<P: AsRef<Path>>(path: P) -> TractResult<ModelProto> {
    let proto_file = tract_onnx::onnx().proto_model_for_path(path)?;
    Ok(proto_file)
}
// from file 
pub fn parse_proto_from_file(reader: &mut dyn Read) -> TractResult<ModelProto> {
    let proto_file = tract_onnx::onnx().proto_model_for_read(reader)?;
    Ok(proto_file)
}
pub fn into_proto(input: String) -> TractResult<()> {
    let ss = serde_json::from_str::<ModelProto>(input.as_str()).unwrap();
    let l = tract_onnx::onnx().parse(&ss).unwrap();
    let _m = l.model.clone();
    // println!("{:?}",m);
    Ok(())
}
// just print model info 
pub fn model_info<P: AsRef<Path>>(path: P) -> TractResult<()> {
    let model = tract_onnx::onnx()
        // load the model
        .model_for_path(path)?
        .into_runnable()?;
    let inf_model = model.model.clone();

    for n in inf_model.nodes() {
        let op_name = n.op().name();
        let node_name = n.name.clone();
        println!("id: {}", n.id);
        println!("op options {:?}", n.op());
        println!("inputs: ");
        for i in n.inputs.iter() {
            print!(" {:?}", i);
            let fact = model.model().outlet_fact(*i).unwrap();
            println!("shape: {:?}", fact.shape.clone());
            // println!("value: {:?}", fact.value.clone());
        }
        for i in n.outputs.iter() {
            println!("out test:{:?}", i.fact.shape);
            for j in i.successors.iter() {
                println!("output: {:?}", j);
            }
        }
        println!("node name: {}", node_name);
        println!("op name: {}", op_name);
        println!();
    }
    Ok(())
}
// only for store func index and weight index
pub struct Indexes {
    weight_idx: Vec<usize>,
    func_idx: Vec<usize>,
}
impl Indexes {
    pub fn new(w: Vec<usize>, f: Vec<usize>) -> Self {
        Indexes {
            weight_idx: w,
            func_idx: f,
        }
    }
}

impl LatexEngine {
    pub fn new() -> Self {
        let symbol_lib = SymbolLibrary::new();

        LatexEngine {
            engine: tract_onnx::onnx(),
            symbol_map: Vec::new(),
            weight_count: 0,
            bias_count: 0,
            const_count: 0,
            formul_count: 0,
            activation_count: 0,
            symbol_library: symbol_lib,
            math_op_vec: Vec::new(),
        }
    }
    // read from file
    pub fn model_from_file(&self, reader: &mut dyn Read) -> TractResult<InferenceModel> {
        let s = self.engine.model_for_read(reader)?.into_runnable()?;
        Ok(s.model().clone())
    }
    fn flush(&mut self) {
        self.symbol_map = Vec::new();
        self.weight_count = 0;
        self.bias_count = 0;
        self.const_count = 0;
        self.activation_count = 0;
    }
    // read from path 
    pub fn parse_from_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        many: Option<usize>,
    ) -> TractResult<LatexResult> {
        let plan = self.engine.model_for_path(path)?.into_runnable()?;
        self.start_parse(&plan, many)
    }
    pub fn parse_from_file(
        &mut self,
        file: &mut dyn Read,
        many: Option<usize>,
    ) -> TractResult<LatexResult> {
        let plan = self.engine.model_for_read(file)?.into_runnable()?;
        self.start_parse(&plan, many)
    }

    // generate input and start parse 
    fn start_parse(
        &mut self,
        plan: &InferencePlan,
        many: Option<usize>,
    ) -> TractResult<LatexResult> {
        let mm = plan.model();
        let input_shape: Vec<usize> = mm.node(0).outputs[0]
            .fact
            .shape
            .dims()
            .map(|s| format!("{}", s).as_str().parse().unwrap())
            .collect();
        let _total_elements: usize = input_shape.iter().product();

        // let mut rng = thread_rng();
        // let vals: Vec<_> = (0..total_elements).map(|_| rng.gen::<f32>()).collect();
        // let input = tract_ndarray::arr1(&vals)
        //     .into_shape(input_shape.as_slice())
        //     .unwrap();

        let result = self.parse_plan(&plan, ParseMode::Full(many));

        Ok(result)
    }
    //  start parse
    pub fn parse_plan(&mut self, original: &InferencePlan, mode: ParseMode) -> LatexResult {
        let plan = original;
        let mut inf_model = plan.model().clone();
        // analyze input and fix order with obstinate
        inf_model.analyse(true).unwrap();

        let mut senario = Vec::new();
        self.symbol_map.resize(inf_model.nodes.len(), None);
        self.math_op_vec.resize(inf_model.nodes.len(), None);

        let start = Instant::now();
        let _print_time = |s: &Instant, _m: &str| {
            let _end = s.elapsed();
        };
        // iterate node by order 
        for (_step, n) in plan.order.iter().enumerate() {
            let node = inf_model.node(*n);
            println!("node {}", *n);
            // println!("node_kind {:?}",node_kind);
            let mt = self.math_op_vec.get_mut(*n).unwrap();
            // print_time(&start,"mathgen");
            let node_op = if let Some(a) = mt {
                a.clone()
            } else {
                let inner = Self::boxed_mathgen(node);
                // print_time(&start,"gen mathgen");
                *mt = Some(inner.clone());
                inner
            };
            let node_kind = self.configure_node(node, *n);
            // print_time(&start,"configure_node");
            let mut candidate: Option<usize> = None;
            // input part
            let input_ids: Vec<usize> = node.inputs.iter().map(|x| x.node).collect();
            if let Some(fk) = node_kind {
                match fk {
                    FormulKind::Activation | FormulKind::Function | FormulKind::Cnn => {
                        match fk {
                            FormulKind::Function => {}
                            FormulKind::Cnn => {
                                candidate = Some(input_ids[0]);
                            }
                            _ => {}
                        }
                        senario.push(*n);
                    }
                    _ => {}
                }
            }

            // configure input nodes
            for i in input_ids.iter() {
                let undefined_node = inf_model.node(*i);
                let _inner = self.configure_node(undefined_node, *i);
                // print_time(&start,"input configure");
            }
            let mut input_shape_option: Option<Vec<usize>> = None;
            // define input shape and output shape 
            for i in &node.inputs {
                let fact = inf_model.outlet_fact(*i).unwrap();

                let input_shape: Vec<usize> = fact
                    .shape
                    .dims()
                    .map(|s| match s {
                        GenericFactoid::Only(x) => x.to_i64().unwrap() as usize,
                        GenericFactoid::Any => {
                            unreachable!()
                        }
                    })
                    .collect();

                if let Some(l) = self.symbol_map[i.node].as_mut() {
                    if let Some(x) = candidate {
                        if x == i.node {
                            if l.output_shape.len() > 0 {
                                input_shape_option = Some(l.output_shape.clone());
                            } else {
                                input_shape_option = Some(input_shape.clone());
                            }
                        }
                    }
                    if l.output_shape.len() == 0 {
                        l.output_shape = input_shape;
                    }
                }
            }
            let output_shape: Vec<usize> = node
                .outputs
                .iter()
                .flat_map(|x| x.fact.shape.dims())
                .map(|x| match x {
                    GenericFactoid::Only(x) => x.to_i64().unwrap() as usize,
                    GenericFactoid::Any => {
                        unreachable!()
                    }
                })
                .collect();
            if let Some(form) = self.symbol_map[*n].as_mut() {
                form.output_shape = output_shape.clone();
                form.input_shape_ref = input_shape_option.clone();
            }

            // println!("opname {}",op_name);
            let forward_string = match mode {
                ParseMode::Brief => {
                    let input_symbols = input_ids
                        .iter()
                        .map(|s| self.symbol_map[*s].as_ref().unwrap().symbol.clone())
                        .collect();
                    node_op.gen_forward_value(
                        input_symbols,
                        input_shape_option,
                        Some(output_shape.clone()),
                    )
                }
                ParseMode::Full(many) => self.rec_node(node, &inf_model, many, &start),
            };
            // print_time(&start,"forward parsing")

            if let Some(form) = self.symbol_map[*n].as_mut() {
                form.inputs = input_ids.clone();
                form.forward_value = forward_string;
            }
        }

        // backward
        let mut latex_result = LatexResult::new(inf_model.nodes.len());
        latex_result.symbol_map = self.symbol_map.clone();

        latex_result.senario = senario;
        // print_time(&start,"end");
        self.flush();
        latex_result
    }
    // generate boxed mathgen 
    fn boxed_mathgen(node: &InferenceNode) -> Box<dyn MathGen> {
        if let Some(e) = node.op_as::<Box<dyn Expansion>>().cloned() {
            Box::new(e)
        } else {
            let op = node.op();
            // println!("op detail {:?}",op);
            let mut result = each_op!(op, [Const, Pad, Dummy, Source, MaxPool, SumPool]);
            let t = result.iter_mut().find_map(|s| std::mem::take(s));
            if let Some(x) = t {
                x
            } else {
                // elementwise
                let ele = node.op_as::<ElementWiseOp>();
                let inner_ref = ele.unwrap().0.as_ref();
                let mut ele_map = each_ele_op!(inner_ref, [Sigmoid]);
                ele_map.iter_mut().find_map(|s| std::mem::take(s)).unwrap()
            }
        }
    }
    // iterate all total back propagation 
    pub fn gen_back_total(
        &self,
        symbol_result: &mut LatexResult,
        model_proto: &ModelProto,
        input_indexs: Indexes,
        depth: Option<usize>,
    ) -> Result<(), std::io::Error> {
        let senario = symbol_result.senario.clone();
        let last_point = senario.last().unwrap();
        let model = tract_onnx::onnx()
            .model_for_proto_model(&model_proto)
            .unwrap();

        let math_ops = Self::math_op_vecs(&model);
        // iterate senario 
        for i in senario.iter() {
            let _node = model.node(*i);
            let math_op = math_ops[*i].as_ref();
            let sym_node = symbol_result.symbol_map[*i].as_ref().unwrap();

            let kind = math_op.get_symbol_type(sym_node.extra_symbol.clone());
            // if senario 
            if is_weightable(kind).is_none() {
                println!("skip {}", *i);
                continue;
            }
            // get each backpropagation 
            let (s, v) = self.gen_each_back(
                &math_ops,
                &model,
                symbol_result,
                (*i, *last_point),
                &input_indexs,
                depth,
            )?;
            if let Some(f) = symbol_result.symbol_map[*i].as_mut() {
                println!("called {},{}", v, s);
                f.backward_value = v;
                f.backward_symbol = s;
            }
        }
        Ok(())
    }
    // generate boxed mathgen in model node 
    pub fn math_op_vecs(model: &InferenceModel) -> Vec<Box<dyn MathGen>> {
        model
            .nodes()
            .iter()
            .map(|x| Self::boxed_mathgen(x))
            .collect()
    }
    //  return(symbol,value)
    pub fn gen_each_back(
        &self,
        math_opvec: &Vec<Box<dyn MathGen>>,
        model: &InferenceModel,
        symbol_result: &LatexResult,
        n_indxs: (usize, usize),
        input_indexs: &Indexes,
        depth: Option<usize>,
    ) -> Result<(String, String), std::io::Error> {
        let (index, last_point) = n_indxs;
        let (_, _, form) = self.symbol_library.get_symbol("_Diff").unwrap();
        let _d_splits = symbol_split(form.formul.as_str()).unwrap();

        let (symbol, shape) = symbol_result.symbol_map[index]
            .as_ref()
            .map(|s| (s.symbol.clone(), s.output_shape.clone()))
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found index",
            ))?;
        let _node = model.node(index);

        let math_op = math_opvec[index].as_ref();

        let _n_shape = shape
            .get(1)
            .unwrap_or(shape.get(0).ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found shape 0",
            ))?);
        // if *n_shape <= which.0 {
        //     return Err(std::io::Error::new(
        //         std::io::ErrorKind::InvalidInput,
        //         "which's size exceed shape range",
        //     ));
        // }
        let sym_node = symbol_result.symbol_map[index].as_ref().unwrap();
        let kind = math_op.get_symbol_type(sym_node.extra_symbol.clone());
        let start_node = if is_weightable(kind).is_some() {
            DiffChainNode::Weightable(index, symbol.clone())
        } else {
            // do not use this when start
            DiffChainNode::UnWeightable(index, symbol.clone())
        };

        let expand_value = self.expand_diff_symbol(
            symbol_result.symbol_map.as_ref(),
            model,
            start_node,
            last_point,
        );
        println!("expand {:?}", expand_value);
        let e_option = depth
            .map(|x| ErrorResultTo::Innner(x))
            .unwrap_or(ErrorResultTo::Total);
        let backward = self.gen_backward_value(&expand_value, &model, e_option, input_indexs);
        let e_symbol = self
            .symbol_library
            .gen_error_symbol(vec!["total".to_string(), "".to_string()]);

        let down_symbol = self
            .symbol_library
            .gen_w_symbol_inner(symbol, input_indexs, true);

        Ok((math_op.gen_backward(e_symbol, down_symbol), backward))
    }
    // recursive forward propgation 
    fn rec_node(
        &self,
        node: &InferenceNode,
        model: &InferenceModel,
        many: Option<usize>,
        timer: &Instant,
    ) -> String {
        let _print_time = |s: &Instant, m: &str| {
            let end = s.elapsed();
            println!("{}: {:?}", m, end);
        };
        let sym_node = self.symbol_map[node.id].as_ref().unwrap();
        if node.inputs.len() == 0 {
            return sym_node.symbol.clone();
        }
        match many {
            Some(x) if x == 0 => sym_node.symbol.clone(),
            _ => {
                let next_many = many.clone().map(|x| x - 1);
                let ins = node
                    .inputs
                    .iter()
                    .map(|out| {
                        let i_node = model.node(out.node);
                        self.rec_node(i_node, model, next_many, timer)
                    })
                    .collect();
                // print_time(&timer,"input");
                let n = node.id;
                let _n_name = node.op().name();

                // generate forward string with op
                let node_op = self.math_op_vec[n].as_ref().unwrap();
                let output_shape = sym_node.output_shape.clone();
                let input_shape = sym_node.input_shape_ref.clone();
                let result = node_op.gen_forward_value(ins, input_shape, Some(output_shape));
                // print_time(&timer,"forward");
                result
            }
        }
    }
    // count up symbol 
    fn countup(&mut self, kind: &FormulKind) -> Option<usize> {
        match kind {
            FormulKind::Activation => {
                self.activation_count += 1;
                Some(self.activation_count)
            }
            FormulKind::Function | FormulKind::Cnn => {
                self.formul_count += 1;
                Some(self.formul_count)
            }
            FormulKind::Bias => {
                self.bias_count += 1;
                Some(self.bias_count)
            }
            FormulKind::Weight => {
                self.weight_count += 1;
                Some(self.weight_count)
            }
            _ => None,
        }
    }
    // create symbol node 
    pub fn configure_node(&mut self, node: &InferenceNode, index: usize) -> Option<FormulKind> {
        if self.symbol_map[index].is_some() {
            return None;
        }
        self.symbol_map[index] = Some(LatexNode::default());
        // println!("node {}", index);
        let n_name = node.name.clone();

        let mt = self.math_op_vec.get_mut(index).unwrap();

        let node_op = if let Some(a) = mt {
            a.clone()
        } else {
            let inner = Self::boxed_mathgen(node);
            *mt = Some(inner.clone());
            inner
        };

        let op_name = node.op().name().to_string();
        // println!("op_name {}", op_name);
        let n_name_split: Vec<&str> = n_name.split(".").collect();
        let inner = n_name_split.last().map(|x| x.to_string());

        let extra_symbol = match node_op.get_symbol_type(inner.clone()) {
            FormulKind::Undefined => None,
            _ => inner.clone(),
        };
        // println!("option {:?}",extra_symbol);

        let kind = node_op.get_symbol_type(extra_symbol.clone());

        let i = self.countup(&kind).unwrap_or(0);
        let symbol = node_op.gen_forward(extra_symbol.clone(), i);
        if let Some(nn) = self.symbol_map[index].as_mut() {
            nn.op_name = op_name;
            nn.index = index;
            nn.symbol = symbol;
            nn.extra_symbol = extra_symbol.clone();
            if node.outputs.len() != 0 && node.outputs[0].successors.len() != 0 {
                for i in node.outputs[0].successors.iter() {
                    nn.outputs.push(i.node);
                }
            }
        }
        Some(kind)
    }
    // expand diff chain based on layer info 
    pub fn expand_diff_symbol(
        &self,
        symbol_map: &Vec<Option<LatexNode>>,
        model: &InferenceModel,
        target: DiffChainNode,
        error_node: usize,
    ) -> DiffChainNode {
        match target {
            // chain start
            DiffChainNode::Chain(v) => {
                // it means activation find it
                let mut sum_it = Vec::new();
                let mut v_clone = v.clone();

                match v[0].clone() {
                    DiffChainNode::Weightable(i, _s) => {
                        println!("weightable in chain: {}", i);
                        if i != error_node {
                            let node = symbol_map[i].as_ref().unwrap();
                            let into_node_id = node.outputs[0];

                            let in_node = symbol_map[into_node_id].as_ref().unwrap();
                            let sum = self.expand_diff_symbol(
                                symbol_map,
                                model,
                                self.diff_node(model, symbol_map, into_node_id),
                                error_node,
                            );
                            let size_check = if in_node.output_shape.len() > 1 {
                                in_node.output_shape.split_at(1).1.to_vec()
                            } else {
                                vec![in_node.output_shape[0]]
                            };
                            sum_it.push(DiffChainNode::Sum(Box::new(sum), size_check));
                            sum_it.append(&mut v_clone);
                            DiffChainNode::Chain(sum_it)
                        } else {
                            DiffChainNode::Chain(v.clone())
                        }
                    }
                    DiffChainNode::UnWeightable(i, _s) => {
                        println!("unwieghtable in chain: {}", i);
                        if i != error_node {
                            let node = symbol_map[i].as_ref().unwrap();
                            let into_node_id = node.outputs[0];

                            let in_node = symbol_map[into_node_id].as_ref().unwrap();
                            let sum = self.expand_diff_symbol(
                                symbol_map,
                                model,
                                self.diff_node(model, symbol_map, into_node_id),
                                error_node,
                            );
                            let size_check = if in_node.output_shape.len() > 1 {
                                in_node.output_shape.split_at(1).1.to_vec()
                            } else {
                                vec![in_node.output_shape[0]]
                            };
                            sum_it.push(DiffChainNode::Sum(Box::new(sum), size_check));
                            sum_it.append(&mut v_clone);
                            DiffChainNode::Chain(sum_it)
                        } else {
                            DiffChainNode::Chain(v.clone())
                        }
                    }
                    _ => DiffChainNode::Chain(v.clone()),
                }
            }
            // first
            DiffChainNode::Weightable(i, s) => {
                //
                let mut result = Vec::new();
                // println!("out length {}", node.outputs.len());
                let sym_node = symbol_map[i].as_ref().unwrap();
                let mut already_rec = false;
                if sym_node.outputs.len() != 0 {
                    let into_node_idx = sym_node.outputs[0];
                    let t_d = self.diff_node(model, symbol_map, into_node_idx);
                    match t_d.clone() {
                        x @ DiffChainNode::Weightable(_, _) => {
                            let in_node = symbol_map[into_node_idx].as_ref().unwrap();
                            let size_check = if in_node.output_shape.len() > 1 {
                                in_node.output_shape.split_at(1).1.to_vec()
                            } else {
                                vec![in_node.output_shape[0]]
                            };
                            let d = self.expand_diff_symbol(symbol_map, model, x, error_node);
                            already_rec = true;
                            result.push(DiffChainNode::Sum(Box::new(d), size_check));
                        }
                        _ => {
                            result.push(t_d.clone());
                        }
                    }
                }
                result.push(DiffChainNode::Weightable(i, s));
                if already_rec {
                    DiffChainNode::Chain(result)
                } else {
                    self.expand_diff_symbol(
                        symbol_map,
                        model,
                        DiffChainNode::Chain(result),
                        error_node,
                    )
                }
            }
            DiffChainNode::UnWeightable(i, s) => {
                //
                let mut result = Vec::new();
                // println!("out length {}", node.outputs.len());
                let sym_node = symbol_map[i].as_ref().unwrap();

                if sym_node.outputs.len() != 0 {
                    let into_node_idx = sym_node.outputs[0];
                    result.push(self.diff_node(model, symbol_map, into_node_idx));
                }
                result.push(DiffChainNode::UnWeightable(i, s));

                self.expand_diff_symbol(symbol_map, model, DiffChainNode::Chain(result), error_node)
            }
            x @ _ => x,
        }
    }
    fn diff_node(
        &self,
        model: &InferenceModel,
        symbol_map: &Vec<Option<LatexNode>>,
        node_idx: usize,
    ) -> DiffChainNode {
        let node = model.node(node_idx);
        let sym_node = symbol_map[node_idx].as_ref().unwrap();
        let math_op = Self::boxed_mathgen(node);
        let kind = math_op.get_symbol_type(sym_node.extra_symbol.clone());
        if is_weightable(kind).is_some() {
            DiffChainNode::Weightable(node_idx, sym_node.symbol.clone())
        } else {
            DiffChainNode::UnWeightable(node_idx, sym_node.symbol.clone())
        }
    }
    // max three
    fn rec_backward(
        &self,
        target: &DiffChainNode,
        back_package: &Vec<(&str, Vec<(&str, &str)>)>,
        model: &InferenceModel,
        level: usize,
        final_model_end: ErrorResultTo,
        pre_chain: Option<String>,
        input_indexs: &Indexes,
        prev_proper_symbols: &Vec<String>,
        prev_size: usize,
    ) -> String {
        let result = match *target {
            DiffChainNode::Sum(ref d, ref many) => match final_model_end {
                ErrorResultTo::Innner(i) if i == level => {
                    let s = pre_chain.unwrap();
                    let (p0_str, _p1_str) = self.symbol_library.get_p0p1(
                        level,
                        input_indexs,
                        s.clone(),
                        prev_proper_symbols,
                        prev_size,
                    );
                    let last_node = only_inputs_symbol_parts(
                        back_package[4].clone(),
                        vec![s.clone(), p0_str.clone()],
                    );
                    let e_sym = self
                        .symbol_library
                        .gen_error_symbol(vec!["total".to_string(), last_node]);
                    let a_sym = only_inputs_symbol_parts(
                        back_package[4].clone(),
                        vec![s.clone(), p0_str.clone()],
                    );
                    only_inputs_symbol_parts(back_package[0].clone(), vec![e_sym, a_sym.clone()])
                }
                _ => {
                    let (edi_symbols, _) = prev_proper_symbols.split_at(many.len());
                    let inner = self.rec_backward(
                        &d,
                        back_package,
                        model,
                        level + 1,
                        final_model_end,
                        pre_chain,
                        input_indexs,
                        prev_proper_symbols,
                        many.len(),
                    );
                    let mut result = String::new();
                    // fit to shape
                    let start_symbols: Vec<String> = edi_symbols
                        .iter()
                        .map(|s| format!("{}n_{{{}}}", s, level))
                        .collect();
                    for (i, s) in many.iter().enumerate() {
                        let end_symbol = (*s - 1).to_string();
                        if i < (many.len() - 1) {
                            let outer_sigma = except_self_symbol_parts(
                                back_package[5].clone(),
                                vec![],
                                vec![start_symbols[i].clone(), end_symbol],
                            );
                            result += &outer_sigma;
                        } else {
                            // last
                            let outer_sigma = except_self_symbol_parts(
                                back_package[1].clone(),
                                vec![inner.clone()],
                                vec![start_symbols[i].clone(), end_symbol],
                            );
                            result += &outer_sigma;
                        }
                    }
                    result
                }
            },
            DiffChainNode::Chain(ref d) => {
                let d1_symbol = if let Some(x) = d.get(1) {
                    Self::get_symbol_if_func(x)
                } else {
                    None
                };
                let d2_symbol = if let Some(x) = d.get(2) {
                    Self::get_symbol_if_func(x)
                } else {
                    None
                };
                let to_insert = pre_chain.unwrap_or("w".to_string());

                match d[0].clone() {
                    DiffChainNode::Weightable(_i, ref s) => {
                        let (p0_str, p1_str) = self.symbol_library.get_p0p1(
                            level,
                            input_indexs,
                            s.clone(),
                            prev_proper_symbols,
                            prev_size,
                        );
                        let last_node = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![s.clone(), p0_str.clone()],
                        );
                        let e_symbol = self
                            .symbol_library
                            .gen_error_symbol(vec!["total".to_string(), last_node]);

                        let a_sym = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![s.clone(), p0_str.clone()],
                        );
                        let p_sym = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![to_insert, p1_str.clone()],
                        );

                        let e_a = only_inputs_symbol_parts(
                            back_package[0].clone(),
                            vec![e_symbol, a_sym.clone()],
                        );
                        let a_p = only_inputs_symbol_parts(
                            back_package[0].clone(),
                            vec![a_sym.clone(), p_sym],
                        );

                        only_inputs_symbol_parts(back_package[2].clone(), vec![e_a, a_p])
                    }
                    DiffChainNode::UnWeightable(_i, ref s) => {
                        let (p0_str, p1_str) = self.symbol_library.get_p0p1(
                            level,
                            input_indexs,
                            d1_symbol.clone().unwrap(),
                            prev_proper_symbols,
                            prev_size,
                        );
                        let last_node = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![s.clone(), p0_str.clone()],
                        );
                        let e_symbol = self
                            .symbol_library
                            .gen_error_symbol(vec!["total".to_string(), last_node]);

                        let a_sym = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![s.clone(), p0_str.clone()],
                        );
                        let b_sym = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![d1_symbol.clone().unwrap(), p0_str.clone()],
                        );
                        let p_sym = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![to_insert, p1_str.clone()],
                        );

                        let e_a = only_inputs_symbol_parts(
                            back_package[0].clone(),
                            vec![e_symbol, a_sym.clone()],
                        );
                        let a_b = only_inputs_symbol_parts(
                            back_package[0].clone(),
                            vec![a_sym.clone(), b_sym.clone()],
                        );
                        let b_p = only_inputs_symbol_parts(
                            back_package[0].clone(),
                            vec![b_sym.clone(), p_sym],
                        );

                        only_inputs_symbol_parts(back_package[3].clone(), vec![e_a, a_b, b_p])
                    }
                    // inner
                    x @ _ => {
                        let first = self.rec_backward(
                            &x,
                            back_package,
                            model,
                            level,
                            final_model_end,
                            d1_symbol.clone(),
                            input_indexs,
                            prev_proper_symbols,
                            prev_size,
                        );

                        if let Some(ref d2) = d2_symbol {
                            let (p0_str, p1_str) = self.symbol_library.get_p0p1(
                                level,
                                input_indexs,
                                d2.clone(),
                                prev_proper_symbols,
                                prev_size,
                            );
                            let p_sym = only_inputs_symbol_parts(
                                back_package[4].clone(),
                                vec![to_insert, p1_str.clone()],
                            );
                            let a_sym = only_inputs_symbol_parts(
                                back_package[4].clone(),
                                vec![d1_symbol.clone().unwrap(), p0_str.clone()],
                            );

                            let b_sym = only_inputs_symbol_parts(
                                back_package[4].clone(),
                                vec![d2.clone(), p0_str.clone()],
                            );

                            let a_b = only_inputs_symbol_parts(
                                back_package[0].clone(),
                                vec![a_sym.clone(), b_sym.clone()],
                            );
                            let b_f = only_inputs_symbol_parts(
                                back_package[0].clone(),
                                vec![b_sym, p_sym],
                            );
                            only_inputs_symbol_parts(back_package[3].clone(), vec![first, a_b, b_f])
                        } else {
                            let (p0_str, p1_str) = self.symbol_library.get_p0p1(
                                level,
                                input_indexs,
                                d1_symbol.clone().unwrap(),
                                prev_proper_symbols,
                                prev_size,
                            );
                            let p_sym = only_inputs_symbol_parts(
                                back_package[4].clone(),
                                vec![to_insert, p1_str.clone()],
                            );
                            let a_sym = only_inputs_symbol_parts(
                                back_package[4].clone(),
                                vec![d1_symbol.clone().unwrap(), p0_str.clone()],
                            );
                            let a_f = only_inputs_symbol_parts(
                                back_package[0].clone(),
                                vec![a_sym.clone(), p_sym],
                            );
                            only_inputs_symbol_parts(back_package[2].clone(), vec![first, a_f])
                        }
                    }
                }
            }
            _ => "".to_string(),
        };
        result
    }

    fn get_symbol_if_func(target: &DiffChainNode) -> Option<String> {
        match target {
            DiffChainNode::Weightable(_, s) => Some(s.clone()),
            DiffChainNode::UnWeightable(_, s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn gen_backward_value(
        &self,
        target: &DiffChainNode,
        model: &InferenceModel,
        final_model_end: ErrorResultTo,
        input_indexs: &Indexes,
    ) -> String {
        let (_, _, form) = self.symbol_library.get_symbol("_Diff").unwrap();
        let d_splits = symbol_split(form.formul.as_str()).unwrap();
        let (_, _, sform) = self.symbol_library.get_symbol("_Sum").unwrap();
        let s_splits = symbol_split(sform.formul.as_str()).unwrap();
        let (_, _, c2form) = self.symbol_library.get_symbol("_Chain2").unwrap();
        let c2_splits = symbol_split(c2form.formul.as_str()).unwrap();
        let (_, _, c3form) = self.symbol_library.get_symbol("_Chain3").unwrap();
        let c3_splits = symbol_split(c3form.formul.as_str()).unwrap();
        let (_, _, underform) = self.symbol_library.get_symbol("_Under").unwrap();
        let under_splits = symbol_split(underform.formul.as_str()).unwrap();
        let (_, _, sum_w) = self.symbol_library.get_symbol("_Sum_w").unwrap();
        let sum_w_splits = symbol_split(sum_w.formul.as_str()).unwrap();

        let vv = vec![
            d_splits,
            s_splits,
            c2_splits,
            c3_splits,
            under_splits,
            sum_w_splits,
        ];
        let propers: Vec<String> = ["c", "h", "w", "b"].iter().map(|s| s.to_string()).collect();
        self.rec_backward(
            target,
            &vv,
            model,
            0,
            final_model_end,
            None,
            input_indexs,
            &propers,
            0,
        )
    }
}
// parsing result struct 
#[derive(Default, Serialize, Deserialize)]
pub struct LatexResult {
    pub symbol_map: Vec<Option<LatexNode>>,
    pub senario: Vec<usize>,
}

impl LatexResult {
    pub fn new(graph_size: usize) -> Self {
        let input = vec![None; graph_size];
        LatexResult {
            symbol_map: input.clone(),
            senario: Vec::new(),
        }
    }
    pub fn get_node_formul(&self, i: usize) -> String {
        if let Some(ref x) = self.symbol_map[i] {
            x.symbol.clone() + "=" + x.forward_value.as_str()
        } else {
            "".to_owned()
        }
    }
    pub fn get_node_backward(&self, i: usize) -> String {
        if let Some(ref x) = self.symbol_map[i] {
            x.backward_symbol.clone() + "=" + x.backward_value.as_str()
        } else {
            "".to_owned()
        }
    }
    pub fn gen_json(&self) -> String {
        let j = serde_json::to_string_pretty(self).unwrap();
        j
    }
    pub fn gen_map_json(&self) -> String {
        serde_json::to_string_pretty(&self.symbol_map).unwrap()
    }
    pub fn erase_slash(&mut self) {
        for n in self.symbol_map.iter_mut() {
            if let Some(r) = n {
                r.erase_slash();
            }
        }
    }
    pub fn from_reader(reader: Vec<u8>) -> Result<Self, std::io::Error> {
        let input_str = std::str::from_utf8(reader.as_slice()).unwrap();
        serde_json::from_str(input_str)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidInput, format!("{:?}", e)))
    }
}
// enum of forward propagation formula dpeth
pub enum ParseMode {
    Brief,
    Full(Option<usize>),
}

#[test]
fn test_expand() {}
