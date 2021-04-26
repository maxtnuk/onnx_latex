use actix_multipart::Multipart;
use actix_web::{App, Error, HttpResponse, HttpServer, Responder, dev::HttpResponseBuilder, error, get, http::{StatusCode, header}, post, web};
use futures::{StreamExt, TryStreamExt};
use tract_onnx::{prelude::*, tract_hir::infer::InferenceOp};
use tract_onnx::latex_tool::*;

use std::{fs::File, io::{Read, Write}};
use derive_more::{Display, Error};
use rand::prelude::*;


type InferencePlan =
    SimplePlan<InferenceFact, Box<dyn InferenceOp>, Graph<InferenceFact, Box<dyn InferenceOp>>>;
#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
    #[display(fmt = "parse error")]
    ParseError,

    #[display(fmt = "new file failed")]
    NewFile
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::ParseError => StatusCode::CONFLICT,
            MyError::NewFile => StatusCode::NOT_ACCEPTABLE
        }
    }
}

#[post("/parse_model")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    //  only one file 
    let mut field = payload.try_next().await?.ok_or(MyError::BadClientData)?;
    
    let content_type = field.content_disposition().unwrap();
    let filename = content_type.get_filename().unwrap();
    println!("file name {}", filename);
    // let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| tempfile::tempfile().map_err(|_|MyError::NewFile)).await?;

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // println!("byte chunk {}",data.len());
        // filesystem operations are blocking, we have to use threadpool
        f =web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
    println!("file size {}",f.metadata().unwrap().len());
    
    // parse section
    let model = read_model(&mut f).map_err(|x| MyError::ParseError )?;
    let mm=model.model();
    let input_shape: Vec<usize> = mm.node(0).outputs[0].fact
                    .shape
                    .dims()
                    .map(|s| format!("{}", s).as_str().parse().unwrap())
                    .collect();
    let total_elements:usize  = input_shape.iter().product();

    let mut rng = thread_rng();
    let vals: Vec<_> = (0..total_elements).map(|_| rng.gen::<f32>()).collect();
    let input = tract_ndarray::arr1(&vals).into_shape(input_shape.as_slice()).unwrap();
    let mut engine=LatexEngine::new();

    let result=engine.parse_plan(&model, tvec![input.into()],ParseMode::Full);
    
    Ok(HttpResponse::Ok().json(result))
}

fn read_model(tmpfile: &mut dyn Read)->TractResult<InferencePlan>{
    let model = tract_onnx::onnx().
    // load the model
    model_for_read(tmpfile)?
    // specify input type and shap
    // optimize the model
    // make the model runnable and fix its inputs and outputs
    .into_runnable()?;
     // println!("{}",result.gen_json());
    Ok(model)
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("hello ");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("hello echo");
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(parse_file)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}