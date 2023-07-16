use super::proxy::Proxy;
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use std::io::Result;

#[get("/t1")]
async fn get_request() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub struct ApiServer {
    proxy: &'static Proxy,
}

impl ApiServer {
    fn from_proxy(proxy: &'static Proxy) -> ApiServer {
        ApiServer { proxy: proxy }
    }
}

#[derive(Deserialize)]
struct Info {
    xauth: String,
    flags: String,
    value: String,
}

#[get("")]
async fn overview() -> HttpResponse {
    HttpResponse::Ok().body("model")
}

#[get("/model")]
async fn model(_data: web::Data<ApiServer>) -> impl Responder {
    HttpResponse::Ok().body("model")
}

#[get("/stats")]
async fn stats_no_xauth(_data: web::Data<ApiServer>) -> impl Responder {
    HttpResponse::Ok().body("stats_no_xauth")
}

#[get("/slots")]
async fn slots_no_xauth(_data: web::Data<ApiServer>) -> impl Responder {
    HttpResponse::Ok().body("slots_no_xauth")
}

#[get("/xping/{xauth}")]
async fn x_ping(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("x_ping")
}

// todo: 路由或许要做一些适配
// #[get("stats/{xauth}")]
#[get("/stats/{xauth}/{flags}")]
async fn stats(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("stats")
}

#[get("/slots/{xauth}")]
async fn slots(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("slots")
}

#[put("/start/{xauth}")]
async fn start(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("start")
}

#[put("/stats/reset/{xauth}")]
async fn reset_stats(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("reset_stats")
}

#[put("/forcegc/{xauth}")]
async fn force_gc(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("force_gc")
}

#[put("/shutdown/{xauth}")]
async fn shutdown(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("shut_down")
}

#[put("/loglevel/{xauth}/{value}")]
async fn log_level(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("log_level")
}

#[put("/fillslots/{xauth}")]
async fn fill_slots(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("fill_slots")
}

#[put("/sentinels/{xauth}")]
async fn set_sentinels(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("set_sentinels")
}

#[put("/sentinels/{xauth}")]
async fn rewatch_sentinels(_data: web::Data<ApiServer>, _info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body("rewatch_sentinels")
}

// Proxy 表示 pika-proxy 的唯一元数据, 应该是作为全局静态的
async fn server_admin(proxy: &'static Proxy) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(ApiServer::from_proxy(proxy))
            .service(
                web::scope("/proxy")
                    .service(overview)
                    .service(model)
                    .service(stats_no_xauth)
                    .service(slots_no_xauth),
            )
            .service(
                web::scope("/api/proxy")
                    .service(model)
                    .service(x_ping)
                    .service(stats)
                    .service(slots)
                    .service(start)
                    .service(reset_stats)
                    .service(force_gc)
                    .service(shutdown)
                    .service(log_level)
                    .service(fill_slots)
                    .service(set_sentinels)
                    .service(rewatch_sentinels),
            )
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
