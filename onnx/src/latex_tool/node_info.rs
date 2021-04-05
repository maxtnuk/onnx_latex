use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::Result;
use std::{io::ErrorKind, path::Path};

use ron::de::from_reader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default,Clone)]
pub struct Formul {
    pub symbol: String,
    pub n_type: String,
    pub entries: HashMap<String, FormulNode>,
}
#[derive(Serialize, Deserialize,Clone)]
pub struct FormulNode {
    pub inputs: usize,
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

pub fn read_ron<P: AsRef<Path>>(path: P) -> Result<Formul> {
    let f = File::open(path)?;
    let result: Formul =
        from_reader(f).map_err(|e| Error::new(ErrorKind::InvalidInput, "parse error"))?;
    Ok(result)
}
