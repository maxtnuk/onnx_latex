use std::{borrow::Cow, collections::HashMap};
use std::fs::File;
use std::io::Error;
use std::io::Result;
use std::{io::ErrorKind, path::Path};

use ron::de::from_reader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct Formul {
    pub symbol: String,
    pub n_type: FormulKind,
    pub entries: HashMap<String, FormulNode>,
}
impl Default for Formul{
    fn default() -> Self {
        Formul{
            symbol: "".to_owned(),
            n_type: FormulKind::Not,
            entries: HashMap::new()
        }
    }
}
#[derive(Serialize, Deserialize,Clone)]
pub struct FormulNode {
    #[serde(default)]
    pub inputs: usize,
    #[serde(default)]
    pub formul: String,
    #[serde(default)]
    pub diff: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
}
impl Default for FormulNode {
    fn default() -> Self {
        FormulNode {
            symbol: None,
            inputs: 0,
            formul: "".to_owned(),
            diff: None,
        }
    }
}
#[derive(Serialize,Deserialize,Clone,PartialEq)]
pub enum FormulKind{
    Activation,
    Function,
    Base,
    Not 
}

impl Formul {
    pub fn gen_symbol(&self,target: &str)-> Result<(String,FormulKind,FormulNode)>{
        if let Some(x) =self.entries.get(target){
            if let Some(ref y)= x.symbol{
                Ok((y.clone(),self.n_type.clone(),x.clone()))
            }else{
                Ok((self.symbol.clone(),self.n_type.clone(),x.clone()))
            }
        }else{
            Err(Error::new(ErrorKind::NotFound,"not found"))
        }  
    }
}

pub fn read_ron<P: AsRef<Path>>(path: P) -> Result<Formul> {
    let f = File::open(path)?;
    let result: Formul =
        from_reader(f).map_err(|e| Error::new(ErrorKind::InvalidInput, format!("{:?}",e)))?;
    Ok(result)
}
