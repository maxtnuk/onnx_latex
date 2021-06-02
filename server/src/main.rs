use actix_multipart::Multipart;
use actix_web::{
    dev::HttpResponseBuilder,
    error, get,
    http::{header, StatusCode},
    post, web, App, Error, HttpResponse, HttpServer, Responder,
};

use derive_more::{Display, Error};
use latex_gen::{Indexes, LatexEngine, LatexResult};
use serde_json::value::Index;
use std::{
    collections::HashMap,
    io::{Cursor, Seek, SeekFrom, Write},
    usize,
};
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde::Serialize;

use futures::{future::ok, stream::once, StreamExt, TryStreamExt};

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
    NewFile,
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
            MyError::NewFile => StatusCode::NOT_ACCEPTABLE,
        }
    }
}
type OnFile = Cursor<Vec<u8>>;

async fn mutlipart_filelist(payload: &mut Multipart) -> Result<HashMap<String, OnFile>, Error> {
    let mut file_list = HashMap::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_name().unwrap();

        // File::create is blocking operation, use threadpool
        let mut f = Cursor::new(Vec::new());

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
        f.seek(SeekFrom::Start(0)).unwrap();
        file_list.insert(filename.to_string(), f);
    }
    Ok(file_list)
}

#[derive(Deserialize)]
struct ParseParam {
    depth: Option<usize>,
}

#[post("/parse_model")]
async fn parse_file(
    web::Query(info): web::Query<ParseParam>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    //  only one file
    let start=Instant::now();
    let mut file_list = mutlipart_filelist(&mut payload).await?;

    let end = start.elapsed();
    println!("file :{:?}",end);
    let mut engine = LatexEngine::new();

    // println!("{:?}",file_list.keys().map(|s| s.to_string()).collect::<Vec<String>>());

    let model = file_list
        .get_mut(&"model".to_string())
        .ok_or(MyError::BadClientData)?;

    let result = engine
        .parse_from_file(model, info.depth)
        .map_err(|_e| MyError::ParseError)?;
    
    let end = start.elapsed();
    println!("parse :{:?}",end);

    model.seek(SeekFrom::Start(0)).unwrap();

    // let original_proto =
    //     latex_gen::parse_proto_from_file(model).map_err(|_e| MyError::InternalError)?;
    // let rr = latex_gen::ParseModelResult::new(original_proto, result);

    let body = once(ok::<_, Error>(web::Bytes::copy_from_slice(
        result.gen_json().as_bytes(),
    )));

    let end = start.elapsed();
    println!("json :{:?}",end);

    Ok(HttpResponse::Ok().streaming(body))
}

#[derive(Deserialize)]
struct BackwardParam {
    layer_node: usize,
    layer_idxs: Vec<usize>,
    weight_idxs: Vec<usize>,
    depth: Option<usize>,
}

#[derive(Serialize, Debug)]
struct BackwardAnswer {
    node: usize,
    layer_idxs: Vec<usize>,
    weight_idxs: Vec<usize>,
    symbol: String,
    value: String,
}

#[post("/backward")]
async fn backward(
    web::Query(info): web::Query<BackwardParam>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let file_list = mutlipart_filelist(&mut payload).await?;

    let mut model_file = file_list
        .get(&"model".to_string())
        .ok_or(MyError::BadClientData)?
        .clone();
    let mut raw_symbol = file_list
        .get(&"symbol".to_string())
        .ok_or(MyError::BadClientData)?
        .clone();

    model_file.seek(SeekFrom::Start(0)).unwrap();
    raw_symbol.seek(SeekFrom::Start(0)).unwrap();

    let engine = LatexEngine::new();

    let model = engine
        .model_from_file(&mut model_file)
        .map_err(|_e| MyError::InternalError)?;

    let symbol =
        LatexResult::from_reader(raw_symbol.into_inner()).map_err(|_e| MyError::InternalError)?;

    let math_ops = latex_gen::LatexEngine::math_op_vecs(&model);

    let indexs = Indexes::new(info.weight_idxs.clone(),info.layer_idxs.clone());
    let last_point = symbol.senario.last().cloned().unwrap();
    let (s, v) = engine
        .gen_each_back(
            &math_ops,
            &model,
            &symbol,
            (info.layer_node, last_point),
            &indexs,
            info.depth,
        )
        .map_err(|_x| MyError::ParseError)?;
    // let r = |s: &String| ->String{s.replace(r#"\\"#,r#"\"#)};

    let result = BackwardAnswer {
        node: info.layer_node,
        layer_idxs: info.layer_idxs,
        weight_idxs: info.weight_idxs,
        symbol: s,
        value: v,
    };
    // println!("{:?}",result);
    Ok(HttpResponse::Ok().json(result))
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
            .service(backward)
            .service(parse_file)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
