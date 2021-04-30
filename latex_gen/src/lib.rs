use tract_onnx::tract_hir::internal::{OpState, SessionState};

use nom::error::ErrorKind;
use rand::prelude::*;
use std::hash::Hash;
use std::{borrow::Borrow, fmt::Debug};
use std::{fmt::Display, io::Read, path::Path};
use tract_onnx::{prelude::*, tract_hir::infer::InferenceOp};

use crate::parse_struct::{except_self_symbol_parts, only_inputs_symbol_parts};
pub use tract_onnx::prelude::TractResult;

use self::{
    node_info::{Formul, FormulKind, FormulNode},
    parse_struct::{insert_symbol_parts, op_parse, symbol_split, DebugValue},
};

use serde::{Deserialize, Serialize};

mod node_info;
mod parse_struct;

type InferenceNode = Node<InferenceFact, Box<dyn InferenceOp>>;

type InferencePlan =
    SimplePlan<InferenceFact, Box<dyn InferenceOp>, Graph<InferenceFact, Box<dyn InferenceOp>>>;

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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LatexNode {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub symbol: String,
    pub value: String,
    pub op_name: String,
    pub shape: Vec<usize>,
    pub forward_prefix: String,
    pub backward_prefix: String,
    pub backward_value: String,
    pub backward_symbol: String,
    pub op_attributes: DebugValue,
}
impl LatexNode {
    // except prefix
    pub fn erase_slash(&mut self) {
        let r = |s: &String| -> String { s.replace(r#"\\"#, r#"\"#) };
        self.symbol = r(&self.symbol);
        self.value = r(&self.value);
        self.backward_symbol = r(&self.backward_symbol);
        self.backward_value = r(&self.backward_value);
    }
}

#[derive(Default, Clone)]
pub struct SymbolLibrary {
    pub func: Formul,
    pub etc: Formul,
    pub activation: Formul,
}

impl SymbolLibrary {
    fn new() -> Self {
        // println!("{}",concat!(env!("OUT_DIR"), "/formuls/formul.ron"));
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffChainNode {
    Weightable(usize, String),
    UnWeightable(usize, String),
    Sum(Box<DiffChainNode>, usize),
    Chain(Vec<DiffChainNode>),
    Not,
}

#[derive(Default, Clone)]
pub struct LatexEngine {
    pub symbol_map: Vec<Option<LatexNode>>,
    pub weight_count: usize,
    pub bias_count: usize,
    pub const_count: usize,
    pub formul_count: usize,
    pub activation_count: usize,
    pub symbol_library: SymbolLibrary,
}
pub enum ErrorResultTo {
    Total,
    Innner(usize),
}

impl LatexEngine {
    pub fn new() -> Self {
        let symbol_lib = SymbolLibrary::new();

        LatexEngine {
            symbol_map: Vec::new(),
            weight_count: 0,
            bias_count: 0,
            const_count: 0,
            formul_count: 0,
            activation_count: 0,
            symbol_library: symbol_lib,
        }
    }
    fn flush(&mut self) {
        self.symbol_map = Vec::new();
        self.weight_count = 0;
        self.bias_count = 0;
        self.const_count = 0;
        self.activation_count = 0;
    }
    pub fn parse_from_path<P: AsRef<Path>>(&mut self, path: P) -> TractResult<LatexResult> {
        let plan = tract_onnx::onnx().model_for_path(path)?.into_runnable()?;
        self.start_parse(&plan)
    }
    pub fn parse_from_file(&mut self, file: &mut dyn Read) -> TractResult<LatexResult> {
        let plan = tract_onnx::onnx().model_for_read(file)?.into_runnable()?;
        self.start_parse(&plan)
    }
    fn start_parse(&mut self, plan: &InferencePlan) -> TractResult<LatexResult> {
        let mm = plan.model();
        let input_shape: Vec<usize> = mm.node(0).outputs[0]
            .fact
            .shape
            .dims()
            .map(|s| format!("{}", s).as_str().parse().unwrap())
            .collect();
        let total_elements: usize = input_shape.iter().product();

        let mut rng = thread_rng();
        let vals: Vec<_> = (0..total_elements).map(|_| rng.gen::<f32>()).collect();
        let input = tract_ndarray::arr1(&vals)
            .into_shape(input_shape.as_slice())
            .unwrap();

        let result = self.parse_plan(&plan, tvec![input.into()], ParseMode::Full);
        Ok(result)
    }

    pub fn parse_plan(
        &mut self,
        original: &InferencePlan,
        inputs: TVec<Tensor>,
        mode: ParseMode,
    ) -> LatexResult {
        let mut state = SimpleState::new(original).unwrap();
        state.set_inputs(inputs).expect("input fail");

        let &mut SimpleState {
            ref mut session_state,
            ref mut states,
            ref mut values,
            ..
        } = &mut state;
        let plan = original;
        let model = original.model();

        let mut senario = Vec::new();
        self.symbol_map.resize(model.nodes.len(), None);

        for (step, n) in plan.order.iter().enumerate() {
            let node = model.node(*n);
            // println!("node {}", *n);
            let node_kind = self.configure_node(node, *n);

            if let Some(fk) = node_kind {
                match fk {
                    FormulKind::Not | FormulKind::Base => {}
                    _ => {
                        senario.push(*n);
                    }
                }
            }

            // let op_name = node.op().name();

            // input part
            let mut inputs: TVec<Arc<Tensor>> = tvec![];
            let input_ids: Vec<usize> = node.inputs.iter().map(|x| x.node).collect();
            for i in input_ids.iter() {
                let undefined_node = model.node(*i);
                self.configure_node(undefined_node, *i);
            }

            for i in &node.inputs {
                let prec_node = model.node(i.node);
                let prec = values[i.node].as_ref().ok_or_else(|| "error").unwrap();
                let fact = model.outlet_fact(*i).unwrap();

                let input_shape: Vec<usize> = fact
                    .shape
                    .dims()
                    .map(|s| format!("{}", s).as_str().parse().unwrap())
                    .collect();
                if let Some(l) = self.symbol_map[i.node].as_mut() {
                    if l.shape.len() == 0 {
                        l.shape = input_shape;
                    }
                }
                inputs.push(prec[i.slot].clone().into())
            }

            // println!("opname {}",op_name);
            let forward_string = match mode {
                ParseMode::Brief => self.parse_symbol(
                    self.symbol_map[*n].as_ref().unwrap().forward_prefix.clone(),
                    *n,
                    input_ids.clone(),
                ),
                ParseMode::Full => self.rec_node(node, model),
            };
            // node formul

            let vs = eval(
                session_state,
                states[node.id].as_mut().map(|s| &mut **s),
                node,
                inputs,
            )
            .unwrap();
            values[node.id] = Some(vs.clone());

            if let Some(form) = self.symbol_map[*n].as_mut() {
                form.inputs = input_ids.clone();
                form.value = forward_string;
                form.shape = vs.iter().flat_map(|x| x.shape().iter()).cloned().collect();
            }
        }

        // backward
        // self.gen_back_total(model, latex_result.senario.clone());
        let mut latex_result = LatexResult::new(model.nodes.len());
        latex_result.symbol_map = self.symbol_map.clone();
        latex_result.senario = senario;
        self.flush();
        latex_result
    }
    pub fn gen_back_total(
        &self,
        symbol_result: &mut LatexResult,
        which: (usize, usize),
        depth: Option<usize>,
    ) -> Result<(), std::io::Error> {
        let senario = symbol_result.senario.clone();
        let last_point = senario.last().unwrap();

        for i in senario.iter() {
            let op_name = symbol_result.symbol_map[*i]
                .as_ref()
                .unwrap()
                .op_name
                .clone();
            if op_name.to_string() != "Gemm" {
                continue;
            }
            let (s, v) = self.gen_each_back(symbol_result, (*i, *last_point), which, depth)?;
            if let Some(f) = symbol_result.symbol_map[*i].as_mut() {
                f.backward_value = v;
                f.backward_symbol = s;
            }
        }
        Ok(())
    }
    //  return(symbol,value)
    pub fn gen_each_back(
        &self,
        symbol_result: &LatexResult,
        n_indxs: (usize, usize),
        which: (usize, usize),
        depth: Option<usize>,
    ) -> Result<(String, String), std::io::Error> {
        let (index, last_point) = n_indxs;
        let (_, _, form) = self.symbol_library.get_symbol("_Diff").unwrap();
        let d_splits = symbol_split(form.formul.as_str()).unwrap();

        let (symbol, shape) = symbol_result.symbol_map[index]
            .as_ref()
            .map(|s| (s.symbol.clone(), s.shape.clone()))
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found index",
            ))?;

        let n_shape = shape
            .get(1)
            .unwrap_or(shape.get(0).ok_or(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found shape 0",
            ))?);
        if *n_shape <= which.0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "which's size exceed shape range",
            ));
        }

        let start_node = DiffChainNode::Weightable(index, symbol.clone());
        // suppose total
        let expand_value =
            self.expand_diff_symbol(symbol_result.symbol_map.as_ref(), start_node, last_point);

        let e_option = depth
            .map(|x| ErrorResultTo::Innner(x))
            .unwrap_or(ErrorResultTo::Total);

        let backward = self.gen_backward_value(&expand_value, e_option, which);
        let e_symbol = self.gen_error_symbol(vec!["total".to_string(), "".to_string()]);

        let down_symbol = self.gen_w_symbol_inner(symbol, which, true);

        Ok((
            backward,
            only_inputs_symbol_parts(d_splits.clone(), vec![e_symbol, down_symbol]),
        ))
    }

    fn gen_w_symbol_inner(&self, target: String, index: (usize, usize), deeper: bool) -> String {
        let (_, _, underform) = self.symbol_library.get_symbol("_Under").unwrap();
        let under_splits = symbol_split(underform.formul.as_str()).unwrap();
        let (_, _, weightform) = self.symbol_library.get_symbol("_Weight").unwrap();
        let weight_splits = symbol_split(weightform.formul.as_str()).unwrap();

        let func_name = only_inputs_symbol_parts(
            under_splits.clone(),
            vec![target, format!("({})", index.0.to_string())],
        );
        let between_symbol =
            only_inputs_symbol_parts(weight_splits.clone(), vec![func_name, index.1.to_string()]);
        if deeper {
            only_inputs_symbol_parts(under_splits.clone(), vec!["w".to_string(), between_symbol])
        } else {
            between_symbol
        }
    }

    fn rec_node(&self, node: &InferenceNode, model: &InferenceModel) -> String {
        let input_ids: Vec<usize> = node.inputs.iter().map(|x| x.node).collect();

        if input_ids.len() == 0 {
            return self.symbol_map[node.id].as_ref().unwrap().symbol.clone();
        }

        let ins = input_ids.iter().fold(Vec::new(), |mut acc, x| {
            let i_node = model.node(*x);
            acc.push(self.rec_node(i_node, model));
            acc
        });
        let n = node.id;
        let n_name = node.op().name();

        if let Some((_, _, f)) = self.symbol_library.get_symbol(n_name.borrow()) {
            self.raw_parse_symbol(f.formul, n, ins)
        } else {
            "".to_string()
        }
    }
    fn create_new_symbol(
        &mut self,
        symbol: String,
        which: FormulKind,
        extra_type: Option<String>,
    ) -> String {
        match which {
            FormulKind::Activation => {
                self.activation_count += 1;
                format!("{}_{}", symbol, self.activation_count - 1)
            }
            FormulKind::Function => {
                self.formul_count += 1;
                format!("{}_{}", symbol, self.formul_count - 1)
            }
            FormulKind::Base => {
                let sharp_split: Vec<&str> = symbol.split("#").collect();
                if let Some(x) = extra_type {
                    let tt = match x.as_ref() {
                        "bias" => {
                            self.bias_count += 1;
                            self.bias_count
                        }
                        "weight" => {
                            self.weight_count += 1;
                            self.weight_count
                        }
                        _ => 1,
                    };
                    let splits = symbol_split(symbol.as_str()).unwrap();
                    insert_symbol_parts(splits, vec![tt.to_string()], Vec::new(), "".to_owned())
                } else {
                    format!("{}", sharp_split[0])
                }
            }
            _ => "".to_owned(),
        }
    }
    pub fn configure_node(&mut self, node: &InferenceNode, index: usize) -> Option<FormulKind> {
        if self.symbol_map[index].is_some() {
            return None;
        }
        let n_name = node.name.clone();
        let n_name_split: Vec<&str> = n_name.split(".").collect();
        let op_name = node.op().name();
        // println!("node id: {}",node.id);
        let mut fkind = FormulKind::Not;
        let debug_op = format!("{:?}", node.op());
        self.symbol_map[index] = Some(LatexNode::default());
        let (symbol, forward_prefix, backward_prefix) = if let Some((symbol, n_type, form)) =
            self.symbol_library.get_symbol(op_name.borrow())
        {
            fkind = n_type.clone();
            (
                self.create_new_symbol(symbol.clone(), n_type, None),
                form.formul,
                form.diff.unwrap_or("".to_string()),
            )
        } else {
            let temp = op_name.to_owned() + "." + n_name_split[1];
            if let Some((symbol, n_type, form)) = self.symbol_library.get_symbol(temp.borrow()) {
                fkind = n_type.clone();
                (
                    self.create_new_symbol(
                        symbol.clone(),
                        n_type,
                        Some(n_name_split[1].to_string()),
                    ),
                    form.formul,
                    form.diff.unwrap_or("".to_string()),
                )
            } else {
                ("".to_owned(), "".to_owned(), "".to_owned())
            }
        };
        if let Some(nn) = self.symbol_map[index].as_mut() {
            nn.op_attributes = op_parse::<(&str, ErrorKind)>(debug_op.as_str())
                .unwrap_or(("", DebugValue::Undefined("".to_owned())))
                .1;
            nn.op_name = op_name.to_string();
            nn.symbol = symbol;
            nn.forward_prefix = forward_prefix;
            nn.backward_prefix = backward_prefix;
            if node.outputs.len() != 0 && node.outputs[0].successors.len() != 0 {
                for i in node.outputs[0].successors.iter() {
                    nn.outputs.push(i.node);
                }
            }
        }
        Some(fkind)
    }

    //  # input,$ self, @: attribute
    fn raw_parse_symbol(&self, original: String, origin: usize, inputs: Vec<String>) -> String {
        let sym_node = self.symbol_map[origin].as_ref().unwrap();

        let splits = symbol_split(original.as_str()).unwrap();

        let attributes: Vec<String> = match sym_node.op_attributes {
            DebugValue::Tuple(ref v) => v.iter().map(|s| s.shallow_to_string()).collect(),
            DebugValue::Object(ref v) => v.iter().map(|(_, a)| a.shallow_to_string()).collect(),
            _ => Vec::new(),
        };
        let s_name = self.symbol_map[origin].as_ref().unwrap().symbol.clone();

        let parsing_result = insert_symbol_parts(splits, inputs, attributes, s_name);

        parsing_result
    }

    fn parse_symbol(&self, original: String, origin: usize, inputs: Vec<usize>) -> String {
        let to_insert: Vec<String> = inputs
            .iter()
            .map(|x| {
                if let Some(ref i) = self.symbol_map[*x] {
                    i.symbol.as_str()
                } else {
                    ""
                }
            })
            .map(|x| x.to_owned())
            .collect();
        self.raw_parse_symbol(original, origin, to_insert)
    }
    pub fn expand_diff_symbol(
        &self,
        symbol_map: &Vec<Option<LatexNode>>,
        target: DiffChainNode,
        error_node: usize,
    ) -> DiffChainNode {
        match target {
            DiffChainNode::Chain(v) => {
                // it means activation find it
                let mut sum_it = Vec::new();
                let mut v_clone = v.clone();

                match v[0].clone() {
                    DiffChainNode::Weightable(i, s) => {
                        println!("weightable in chain: {}", i);
                        if i != error_node {
                            DiffChainNode::Not
                        } else {
                            DiffChainNode::Chain(v.clone())
                        }
                    }
                    DiffChainNode::UnWeightable(i, s) => {
                        println!("unwieghtable in chain: {}", i);
                        if i != error_node {
                            let node = symbol_map[i].as_ref().unwrap();
                            let into_node_id = node.outputs[0];

                            let in_node = symbol_map[into_node_id].as_ref().unwrap();
                            let sum = self.expand_diff_symbol(
                                symbol_map,
                                DiffChainNode::Weightable(into_node_id, in_node.symbol.clone()),
                                error_node,
                            );
                            let size_check = if in_node.shape.len() > 1 {
                                in_node.shape[1]
                            } else {
                                in_node.shape[0]
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
                let node = symbol_map[i].as_ref().unwrap();
                let mut result = Vec::new();
                // println!("out length {}", node.outputs.len());

                if node.outputs.len() != 0 {
                    let into_node = node.outputs[0];
                    let symbol_name = symbol_map[into_node].as_ref().unwrap().symbol.clone();
                    result.push(DiffChainNode::UnWeightable(into_node, symbol_name));
                }
                result.push(DiffChainNode::Weightable(i, s));

                self.expand_diff_symbol(symbol_map, DiffChainNode::Chain(result), error_node)
            }
            x @ _ => x,
        }
    }
    // max three
    fn rec_backward(
        &self,
        target: &DiffChainNode,
        back_package: &Vec<(&str, Vec<(&str, &str)>)>,
        level: usize,
        final_model_end: ErrorResultTo,
        pre_chain: Option<String>,
        weight: (usize, usize),
    ) -> String {
        let result = match *target {
            DiffChainNode::Sum(ref d, many) => match final_model_end {
                ErrorResultTo::Innner(i) if i == level => {
                    let s = pre_chain.unwrap();
                    let (p0_str, p1_str) = self.get_p0p1(level, weight, s.clone());
                    let last_node = only_inputs_symbol_parts(
                        back_package[4].clone(),
                        vec![s.clone(), p0_str.clone()],
                    );
                    let e_sym = self.gen_error_symbol(vec!["total".to_string(), last_node]);
                    let a_sym = only_inputs_symbol_parts(
                        back_package[4].clone(),
                        vec![s.clone(), p0_str.clone()],
                    );
                    only_inputs_symbol_parts(back_package[0].clone(), vec![e_sym, a_sym.clone()])
                }
                _ => {
                    let inner = self.rec_backward(
                        &d,
                        back_package,
                        level + 1,
                        final_model_end,
                        pre_chain,
                        weight,
                    );
                    let start_symbol = format!("n_{}", level);
                    let end_symbol = (many - 1).to_string();
                    except_self_symbol_parts(
                        back_package[1].clone(),
                        vec![inner],
                        vec![start_symbol, end_symbol],
                    )
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
                    DiffChainNode::Weightable(i, ref s) => {
                        let (p0_str, p1_str) = self.get_p0p1(level, weight, s.clone());
                        let last_node = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![s.clone(), p0_str.clone()],
                        );
                        let e_symbol = self.gen_error_symbol(vec!["total".to_string(), last_node]);

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
                    DiffChainNode::UnWeightable(i, ref s) => {
                        let (p0_str, p1_str) =
                            self.get_p0p1(level, weight, d1_symbol.clone().unwrap());
                        let last_node = only_inputs_symbol_parts(
                            back_package[4].clone(),
                            vec![s.clone(), p0_str.clone()],
                        );
                        let e_symbol = self.gen_error_symbol(vec!["total".to_string(), last_node]);

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
                            level,
                            final_model_end,
                            d1_symbol.clone(),
                            weight,
                        );

                        if let Some(ref d2) = d2_symbol {
                            let (p0_str, p1_str) = self.get_p0p1(level, weight, d2.clone());
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
                            let (p0_str, p1_str) =
                                self.get_p0p1(level, weight, d1_symbol.clone().unwrap());
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
    fn get_p0p1(
        &self,
        level: usize,
        weight: (usize, usize),
        last_symbol: String,
    ) -> (String, String) {
        if level == 0 {
            let p0_temp = format!("({})", weight.0);
            let p1_temp = self.gen_w_symbol_inner(last_symbol, weight, false);
            (p0_temp, p1_temp)
        } else {
            (
                format!("n_{}", level - 1),
                if level > 1 {
                    format!("n_{}", level - 2)
                } else {
                    format!("({})", weight.0)
                },
            )
        }
    }

    fn get_symbol_if_func(target: &DiffChainNode) -> Option<String> {
        match target {
            DiffChainNode::Weightable(_, s) => Some(s.clone()),
            DiffChainNode::UnWeightable(_, s) => Some(s.clone()),
            _ => None,
        }
    }
    fn gen_error_symbol(&self, target: Vec<String>) -> String {
        let (e_symbol, _, _) = self.symbol_library.get_symbol("Error").unwrap();
        let splits = symbol_split(e_symbol.as_str()).unwrap();
        insert_symbol_parts(splits, target, Vec::new(), "".to_string())
    }

    pub fn gen_backward_value(
        &self,
        target: &DiffChainNode,
        final_model_end: ErrorResultTo,
        weight: (usize, usize),
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

        let vv = vec![d_splits, s_splits, c2_splits, c3_splits, under_splits];
        self.rec_backward(target, &vv, 0, final_model_end, None, weight)
    }
}

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
            x.symbol.clone() + "=" + x.value.as_str()
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
    pub fn erase_slash(&mut self) {
        for n in self.symbol_map.iter_mut() {
            if let Some(r) = n {
                r.erase_slash();
            }
        }
    }
}

pub enum ParseMode {
    Brief,
    Full,
}

#[test]
fn test_expand() {}
