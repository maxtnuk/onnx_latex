use actix_multipart::Multipart;
use actix_web::{
    dev::HttpResponseBuilder,
    error, get,
    http::{header, StatusCode},
    post, web, App, Error, HttpResponse, HttpServer, Responder,
};
use futures::{StreamExt, TryStreamExt};

use derive_more::{Display, Error};
use latex_gen::{LatexEngine, LatexResult};
use std::{
    io::{Cursor, Read, Seek, SeekFrom, Write},
    usize,
};

use serde::Deserialize;
use serde::Serialize;

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

#[post("/parse_model")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    //  only one file
    let mut field = payload.try_next().await?.ok_or(MyError::BadClientData)?;

    let content_type = field.content_disposition().unwrap();
    let mut f = Cursor::new(Vec::new());
    // let mut f = web::block(|| std::fs::File::create(tmp).map_err(|_| MyError::NewFile)).await?;
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // println!("byte chunk {}",data.len());
        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
    // println!("file size {}", f.metadata().unwrap().len());

    // parse section
    f.seek(SeekFrom::Start(0)).unwrap();

    let mut engine = LatexEngine::new();

    let mut result = engine
        .parse_from_file(&mut f)
        .map_err(|e| MyError::ParseError)?;
    // result.erase_slash();
    Ok(HttpResponse::Ok().json(result))
}

#[derive(Deserialize)]
struct BackwardParam {
    layer_node: usize,
    layer_idx: usize,
    weight_idx: usize,
    depth: Option<usize>,
}

#[derive(Serialize)]
struct BackwardAnswer {
    node: usize,
    layer_idx: usize,
    weight_idx: usize,
    symbol: String,
    value: String,
}

#[post("/backward")]
async fn backward(
    web::Query(info): web::Query<BackwardParam>,
    req_body: web::Json<LatexResult>,
) -> Result<HttpResponse, Error> {
    let engine = LatexEngine::new();
    let lr = req_body.into_inner();
    let last_point = lr.senario.last().cloned().unwrap();

    let (s, v) = engine
        .gen_each_back(
            &lr,
            (info.layer_node, last_point),
            (info.layer_idx, info.weight_idx),
            info.depth,
        )
        .map_err(|x| MyError::ParseError)?;
    // let r = |s: &String| ->String{s.replace(r#"\\"#,r#"\"#)};

    let result = BackwardAnswer {
        node: info.layer_node,
        layer_idx: info.layer_idx,
        weight_idx: info.weight_idx,
        symbol: s,
        value: v,
    };
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