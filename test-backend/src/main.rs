use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use casper_client::{self, GlobalStateStrParams, SessionStrParams};
use casper_node::types::Deploy;

use awc::Client;
use jsonrpc_lite::{JsonRpc, Params};
use log::{info, trace, warn};
use serde_json::{Map, Value};

const NODE_IP: &str = "http://94.130.10.55:7777";

#[get("/state/{key_hash}")]
async fn get_global_state(args: web::Path<String>) -> impl Responder {
    let key_hash = args.into_inner();
    warn!("Key hash: {}", key_hash);
    // panic!("Key hash: {}", key_hash);
    // let root_hash = casper_client::get_state_root_hash("", NODE_IP, 1, "").await;

    let params = GlobalStateStrParams {
        is_block_hash: false,
        hash_value: "ca08a1bdb4f22713d7fcb19315c508576dc9dee1feea49956985aa8c53802cec",
    };
    // ca08a1bdb4f22713d7fcb19315c508576dc9dee1feea49956985aa8c53802cec
    let global_state =
        casper_client::query_global_state("", NODE_IP, 0, params, &key_hash, "counter").await;
    HttpResponse::Ok().body(format!("{:#?}", global_state))
}

#[post("/accept_deploy")]
async fn accept_deploy(req_body: String) -> impl Responder {
    let params: Params = serde_json::from_str(&req_body).unwrap();
    let rpc_req = JsonRpc::request_with_params(String::from(""), "account_put_deploy", params);

    let client = Client::default(); // ! can be moved to App state
    let res = client
        .post("http://94.130.10.55:7777/rpc")
        .send_json(&rpc_req)
        .await
        .expect("should get response")
        .body().await;
    HttpResponse::Ok().body(format!("{:?}", res))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(get_global_state)
            .service(accept_deploy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
