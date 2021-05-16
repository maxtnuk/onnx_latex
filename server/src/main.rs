use actix_multipart::Multipart;
use actix_web::{
    dev::HttpResponseBuilder,
    error, get,
    http::{header, StatusCode},
    post, web, App, Error, HttpResponse, HttpServer, Responder,
};

use derive_more::{Display, Error};
use latex_gen::{LatexEngine, LatexResult};
use std::{
    collections::HashMap,
    io::{Cursor, Seek, SeekFrom, Write},
    usize,
};

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
    mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    //  only one file
    let mut file_list = mutlipart_filelist(&mut payload).await?;

    let mut engine = LatexEngine::new();

    // println!("{:?}",file_list.keys().map(|s| s.to_string()).collect::<Vec<String>>());

    let model = file_list
        .get_mut(&"model".to_string())
        .ok_or(MyError::BadClientData)?;

    let result = engine
        .parse_from_file(model,info.depth)
        .map_err(|_e| MyError::ParseError)?;

    model.seek(SeekFrom::Start(0)).unwrap();

    let original_proto =
        latex_gen::parse_proto_from_file(model).map_err(|_e| MyError::InternalError)?;
    let rr = latex_gen::ParseModelResult::new(original_proto, result);

    let body = once(ok::<_, Error>(web::Bytes::copy_from_slice(
        rr.json().as_bytes(),
    )));

    Ok(HttpResponse::Ok().streaming(body))
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
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let file_list = mutlipart_filelist(&mut payload).await?;

    let mut model_file = file_list
        .get(&"model".to_string())
        .ok_or(MyError::BadClientData)?
        .clone();
    let mut latex_map = file_list
        .get(&"symbols".to_string())
        .ok_or(MyError::BadClientData)?
        .clone();

    let engine = LatexEngine::new();
    let lr = LatexResult::from_reader(&mut latex_map).map_err(|_e| MyError::ParseError)?;

    let model = engine
        .model_from_file(&mut model_file)
        .map_err(|_e| MyError::InternalError)?;

    let last_point = lr.senario.last().cloned().unwrap();
    let (s, v) = engine
        .gen_each_back(
            &model,
            &lr,
            (info.layer_node, last_point),
            (info.layer_idx, info.weight_idx),
            info.depth,
        )
        .map_err(|_x| MyError::ParseError)?;
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
