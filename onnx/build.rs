use std::{env, fs, path};

fn main()-> Result<(),std::io::Error> {
    let workdir = path::PathBuf::from(env::var("OUT_DIR").unwrap()).join("prost");
    let _ = fs::create_dir_all(&workdir);
    prost_build::Config::new()
        .out_dir(workdir.clone())
        .compile_protos(&["protos/onnx/onnx.proto3"], &["protos/"])
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{:?}",e)))?;
    fs::copy(workdir.join("onnx.rs").clone(), path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("onnx.rs"))?;
    Ok(())
}
