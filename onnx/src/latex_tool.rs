use ron::Value;
use tract_hir::{internal::{OpState, SessionState}, tract_core::itertools::Itertools};

use crate::{prelude::*, tract_hir::infer::InferenceOp};
use nom::error::ErrorKind;
use std::fmt::Debug;
use std::hash::Hash;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

use self::{
    node_info::{Formul, FormulKind, FormulNode},
    parse_struct::{insert_symbol_parts, op_parse, symbol_split, DebugValue},
};

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
#[derive(Debug, Clone, Default)]
pub struct LatexNode {
    pub inputs: Vec<usize>,
    pub symbol: String,
    pub value: String,
    pub shape: String,
    pub prefix: String,
    pub backward: String,
    pub op_attributes: DebugValue,
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
        let func_info =
            node_info::read_ron(concat!(env!("CARGO_MANIFEST_DIR"), "/formuls/formul.ron"))
                .expect("formul error");
        let etc_info = node_info::read_ron(concat!(env!("CARGO_MANIFEST_DIR"), "/formuls/etc.ron"))
            .expect("etc error");
        let activation_info = node_info::read_ron(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/formuls/activation.ron"
        ))
        .expect("activation error");
        SymbolLibrary {
            func: func_info,
            etc: etc_info,
            activation: activation_info,
        }
    }
    // (symbol,form)
    pub fn get_symbol(&self, target: String) -> Option<(String, FormulKind, FormulNode)> {
        let form = [&self.func, &self.etc, &self.activation];
        form.iter()
            .filter_map(|x| x.gen_symbol(&target).ok())
            .next()
    }
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
enum InputsType {
    Multi(Vec<String>),
    Single(String),
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
        *self = Self::new();
    }

    pub fn parse_plan(
        &mut self,
        original: &InferencePlan,
        inputs: TVec<Tensor>,
        mode: ParseMode,
    ) -> LatexResult {
        let mut state = SimpleState::new(original).unwrap();
        state.set_inputs(inputs);

        let &mut SimpleState {
            ref mut session_state,
            ref mut states,
            ref mut values,
            ..
        } = &mut state;
        let plan = original;
        let model = original.model();

        let mut latex_result = LatexResult::new(model.nodes.len());
        self.symbol_map.resize(model.nodes.len(), None);

        for (step, n) in plan.order.iter().enumerate() {
            let node = model.node(*n);
            // println!("node {}",*n);
            self.configure_node(node, *n);

            let op_name = node.op().name();
            let mut inputs: TVec<Arc<Tensor>> = tvec![];
            let input_ids: Vec<usize> = node.inputs.iter().map(|x| x.node).collect();
            for i in input_ids.iter() {
                let undefined_node = model.node(*i);
                self.configure_node(undefined_node, *i);
            }

            for i in &node.inputs {
                let prec_node = model.node(i.node);
                let prec = values[i.node].as_ref().ok_or_else(|| "error").unwrap();
                inputs.push(prec[i.slot].clone().into())
            }
            // println!("opname {}",op_name);
            let form = self.symbol_map[*n].clone().unwrap();

            let f_string = match mode {
                ParseMode::Brief => self.parse_symbol(form.prefix, *n, input_ids.clone()),
                ParseMode::Full => self.rec_node(node, model),
            };
            if let Some(ref mut x) = self.symbol_map[*n] {
                x.inputs = input_ids.clone();
                x.value = f_string;
            }

            // node formul

            let vs = eval(
                session_state,
                states[node.id].as_mut().map(|s| &mut **s),
                node,
                inputs,
            )
            .unwrap();
            values[node.id] = Some(vs);
        }
        latex_result.symbol_map = self.symbol_map.clone();
        latex_result
    }
    fn rec_node(&self, node: &InferenceNode, model: &InferenceModel) -> String {
        let input_ids: Vec<usize> = node.inputs.iter().map(|x| x.node).collect();
        if input_ids.len() == 0 {
            return self.symbol_map[node.id].clone().unwrap().symbol;
        }

        let ins = input_ids.iter().fold(Vec::new(), |mut acc, x| {
            let i_node = model.node(*x);
            acc.push(self.rec_node(i_node, model));
            acc
        });
        let n = node.id;
        let n_name = node.op().name();

        if let Some((_, _, f)) = self.symbol_library.get_symbol(n_name.to_string()) {
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
                    if sharp_split.len() > 1 {
                        format!("{}{}{}", sharp_split[0], tt - 1, sharp_split[1])
                    } else {
                        format!("{}_{}", sharp_split[0], tt - 1)
                    }
                } else {
                    format!("{}", sharp_split[0])
                }
            }
            _ => "".to_owned(),
        }
    }
    pub fn configure_node(&mut self, node: &InferenceNode, index: usize) {
        if self.symbol_map[index].is_some() {
            return;
        }
        let mut result = LatexNode::default();
        let n_name = node.name.clone();
        let n_name_split: Vec<&str> = n_name.split(".").collect();
        let op_name = node.op().name();
        // println!("node id: {}",node.id);

        let debug_op = format!("{:?}", node.op());
        if let Ok((s, inner)) = op_parse::<(&str, ErrorKind)>(debug_op.as_str()) {
            result.op_attributes = inner;
        } else {
            result.op_attributes = DebugValue::Undefined("".to_owned());
        }
        // find symbol
        if let Some((symbol, n_type, form)) = self.symbol_library.get_symbol(op_name.to_string()) {
            result.symbol = self.create_new_symbol(symbol.clone(), n_type.clone(), None);
            result.prefix = form.formul;
        } else {
            let temp = op_name + "." + n_name_split[1];
            if let Some((symbol, n_type, form)) = self.symbol_library.get_symbol(temp.to_string()) {
                result.symbol = self.create_new_symbol(
                    symbol.clone(),
                    n_type.clone(),
                    Some(n_name_split[1].to_string()),
                );
                result.prefix = form.formul;
            }
        }
        self.symbol_map[index] = Some(result.clone());
    }
    #[deprecated]
    fn insert_parts(&self, splits: Vec<&str>, inputs: InputsType) -> String {
        let mut temp = String::new();
        temp += splits[0];
        for (i, ts) in (1..splits.len()).enumerate() {
            match inputs {
                InputsType::Multi(ref x) => {
                    temp += &(x[i].to_owned() + splits[ts]);
                }
                InputsType::Single(ref x) => {
                    temp += &(x.clone() + splits[ts]);
                }
            }
        }
        temp
    }
    //  # input,$ self, @: attribute
    fn raw_parse_symbol(&self, original: String, origin: usize, inputs: Vec<String>) -> String {
        let sym_node = self.symbol_map[origin].clone().unwrap();

        let splits = symbol_split(original.as_str()).unwrap();

        let attributes: Vec<String> = match sym_node.op_attributes {
            DebugValue::Tuple(v) => v.iter().map(|s| s.shallow_to_string()).collect(),
            DebugValue::Object(v) => v
                .iter()
                .sorted_by_key(|(k,dv)| k.chars().nth(0).unwrap())
                .map(|(l, a)| a.shallow_to_string())
                .collect(),
            _ => Vec::new(),
        };
        let s_name = self.symbol_map[origin].clone().unwrap().symbol.clone();

        let parsing_result = insert_symbol_parts(splits, inputs, attributes, s_name);

        // // #part
        // let temp_split: Vec<&str> = ori_copy.split('#').collect();

        // if temp_split.len() > 0 {
        //     if temp_split.len() != inputs.len() {
        //         let t: Vec<String> = inputs
        //             .iter()
        //             .cycle()
        //             .cloned()
        //             .take(temp_split.len())
        //             .collect();
        //         ori_copy = self.insert_parts(temp_split, InputsType::Multi(t));
        //     } else {
        //         ori_copy = self.insert_parts(temp_split, InputsType::Multi(inputs));
        //     }
        // }

        // // $ part

        // let self_split: Vec<&str> = ori_copy.split('$').collect();
        // if self_split.len() > 0 {
        //     ori_copy = self.insert_parts(self_split, InputsType::Single(s_name));
        // }
        // ori_copy
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
}

#[derive(Default)]
pub struct LatexResult {
    pub symbol_map: Vec<Option<LatexNode>>,
}

impl LatexResult {
    pub fn new(graph_size: usize) -> Self {
        let input = vec![None; graph_size];
        LatexResult {
            symbol_map: input.clone(),
        }
    }
    pub fn get_node_formul(&self, i: usize) -> String {
        if let Some(ref x) = self.symbol_map[i] {
            x.symbol.clone() + "=" + x.value.as_str()
        } else {
            "".to_owned()
        }
    }
}

pub enum ParseMode {
    Brief,
    Full,
}
