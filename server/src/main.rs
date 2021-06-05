use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{
    dev::HttpResponseBuilder,
    error, get,
    http::{header, StatusCode},
    post,
    web::{self, Buf},
    App, Error, HttpResponse, HttpServer, Responder,
};

use derive_more::{Display, Error};
use latex_gen::{Indexes, LatexEngine, LatexResult};

use std::{
    collections::HashMap,
    io::{Cursor, Seek, SeekFrom, Write},
    usize,
};

use serde::Deserialize;
use serde::Serialize;

use futures::{future::ok, stream::once, StreamExt, TryStreamExt};

// define network error 
#[derive(Debug, Display, Error)]
enum NetworkError {
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

impl error::ResponseError for NetworkError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            NetworkError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            NetworkError::BadClientData => StatusCode::BAD_REQUEST,
            NetworkError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            NetworkError::ParseError => StatusCode::CONFLICT,
            NetworkError::NewFile => StatusCode::NOT_ACCEPTABLE,
        }
    }
}
type OnFile = Cursor<Vec<u8>>;

// get file from multipart 
async fn mutlipart_filelist(payload: &mut Multipart) -> Result<HashMap<String, OnFile>, Error> {
    let mut file_list = HashMap::new();

    while let Ok(Some(field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_name().unwrap();

        // File::create is blocking operation, use threadpool

        // Field in turn is stream of *Bytes* object
        let data: Vec<u8> = field
            .into_stream()
            .filter_map(move |chunk| async { chunk.ok() })
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(x.bytes());
                async { acc }
            })
            .await;

        let mut f = Cursor::new(data);

        f.seek(SeekFrom::Start(0)).unwrap();
        file_list.insert(filename.to_string(), f);
    }
    Ok(file_list)
}

// parse model param
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

    let mut file_list = mutlipart_filelist(&mut payload).await?;

    let mut engine = LatexEngine::new();

    let model = file_list
        .get_mut(&"model".to_string())
        .ok_or(NetworkError::BadClientData)?;

    // parsing model file with depth 
    let result = engine
        .parse_from_file(model, info.depth)
        .map_err(|_e| NetworkError::ParseError)?;

    model.seek(SeekFrom::Start(0)).unwrap();
    //  to json 
    let body = once(ok::<_, Error>(web::Bytes::copy_from_slice(
        result.gen_json().as_bytes(),
    )));

    Ok(HttpResponse::Ok().streaming(body))
}

// backward params
#[derive(Deserialize)]
struct BackwardParam {
    layer_node: usize,
    layer_idxs: Vec<usize>,
    weight_idxs: Vec<usize>,
    depth: Option<usize>,
}

// response json struct
#[derive(Serialize, Debug)]
struct BackwardAnswer {
    node: usize,
    layer_idxs: Vec<usize>,
    weight_idxs: Vec<usize>,
    symbol: String,
    value: String,
}

// generate backprapogation fomula
#[post("/backward")]
async fn backward(
    web::Query(info): web::Query<BackwardParam>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let file_list = mutlipart_filelist(&mut payload).await?;

    // get model file
    let mut model_file = file_list
        .get(&"model".to_string())
        .ok_or(NetworkError::BadClientData)?
        .clone();
    // get symbol map 
    let mut raw_symbol = file_list
        .get(&"symbol".to_string())
        .ok_or(NetworkError::BadClientData)?
        .clone();

    // reset file seek  
    model_file.seek(SeekFrom::Start(0)).unwrap();
    raw_symbol.seek(SeekFrom::Start(0)).unwrap();

    let engine = LatexEngine::new();

    //  get inference model 
    let model = engine
        .model_from_file(&mut model_file)
        .map_err(|_e| NetworkError::InternalError)?;

    let symbol =
        LatexResult::from_reader(raw_symbol.into_inner()).map_err(|_e| NetworkError::InternalError)?;

    // generate math ops 
    let math_ops = latex_gen::LatexEngine::math_op_vecs(&model);

    let indexs = Indexes::new(info.weight_idxs.clone(), info.layer_idxs.clone());
    let last_point = symbol.senario.last().cloned().unwrap();
    // launch back propagation 
    let (s, v) = engine
        .gen_each_back(
            &math_ops,
            &model,
            &symbol,
            (info.layer_node, last_point),
            &indexs,
            info.depth,
        )
        .map_err(|_x| NetworkError::ParseError)?;
    

    let result = BackwardAnswer {
        node: info.layer_node,
        layer_idxs: info.layer_idxs,
        weight_idxs: info.weight_idxs,
        symbol: s,
        value: v,
    };
    // to_json
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
        // cors for react client 
        let cors = Cors::default().allowed_origin("http://localhost:3000");
        App::new()
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(backward)
            .service(parse_file)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
