use tract_hir::internal::{OpState, SessionState};

use crate::{prelude::*, tract_hir::infer::InferenceOp};
use std::fmt::Display;
use std::hash::Hash;
use std::fmt::Debug;

use self::node_info::{Formul, FormulNode};

mod node_info;

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
}

#[derive(Default, Clone)]
pub struct SymbolLibrary {
    pub func: Formul,
    pub etc: Formul,
    pub activation: Formul,
}

impl SymbolLibrary {
    fn new() -> Self {
        let func_info = node_info::read_ron("formuls/formul.ron").expect("formul error");
        let etc_info = node_info::read_ron("formuls/etc.ron").expect("etc error");
        let activation_info = node_info::read_ron("formuls/activation.ron").expect("activation error");
        SymbolLibrary {
            func: func_info,
            etc: etc_info,
            activation: activation_info,
        }
    }
    pub fn get_symbol(&self, target: String) -> Option<(String, FormulNode)> {
        let form = [&self.func, &self.etc, &self.activation];
        form.iter()
            .filter(|x| x.entries.contains_key(&target)|| x.entries.iter().any(|(_,f)| f.symbol.is_some()))
            .map(|x| (x.n_type.clone(), x.entries.get(&target).unwrap().clone()))
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
            let form_node=self.symbol_library.get_symbol(op_name.to_string());

            let f_string=match mode{
                ParseMode::Brief=>{
                    self.parse_symbol(form_node.unwrap().1.formul, *n, input_ids.clone())
                },
                ParseMode::Full =>{
                    self.rec_node(node, model)
                }
            };
            if let Some(ref mut x) = self.symbol_map[*n] {
                x.inputs = input_ids.clone();
                x.value=f_string;
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

        if let Some((_, f)) = self.symbol_library.get_symbol(n_name.to_string()) {
            self.raw_parse_symbol(f.formul, n, ins)
        } else {
            "".to_string()
        }
    }
    fn create_new_symbol(&mut self, symbol: String, which: String) -> String {
        match which.as_ref() {
            "activation" => {
                self.activation_count += 1;
                format!("{}_{}", symbol, self.activation_count - 1)
            },
            "function" => {
                self.formul_count += 1;
                format!("{}_{}", symbol, self.formul_count - 1)
            },
            x @ _ => {
                let tt = match x {
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
                format!("\\overline{{{}_{}}}", symbol, tt - 1)
            }
        }
    }
    pub fn configure_node(&mut self, node: &InferenceNode, index: usize){
        if self.symbol_map[index].is_some() {
            return;
        }
        let mut result = LatexNode::default();
        let n_name = node.name.clone();
        let n_name_split: Vec<&str> = n_name.split(".").collect();
        let op_name = node.op().name();
        let form_node = if n_name_split.len() > 1 {
            let temp = op_name + "." + n_name_split[1];
            self.symbol_library.get_symbol(temp.to_string())
        } else {
            self.symbol_library.get_symbol(op_name.to_string())
        };

        //  gen symbol
        if let Some((which, form)) = form_node {
            result.symbol = self.create_new_symbol(form.symbol.unwrap(), which);
            self.symbol_map[index] = Some(result.clone());
        }
    }
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
    fn raw_parse_symbol(&self, original: String, origin: usize, inputs: Vec<String>) -> String {
        let mut ori_copy = original.clone();

        // #part
        let temp_split: Vec<&str> = ori_copy.split('#').collect();

        if temp_split.len() > 0 {
            ori_copy = self.insert_parts(temp_split, InputsType::Multi(inputs));
        }

        // $ part
        let s_name = self.symbol_map[origin].clone().unwrap().symbol.clone();
        let self_split: Vec<&str> = ori_copy.split('$').collect();
        if self_split.len() > 0 {
            ori_copy = self.insert_parts(self_split, InputsType::Single(s_name));
        }
        ori_copy
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
