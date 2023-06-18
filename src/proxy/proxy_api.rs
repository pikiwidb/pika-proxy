use super::proxy::Proxy;
use actix_web::{get, post, route, web, App, HttpResponse, HttpServer, Responder};
use std::cell::Cell;

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

async fn overview() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

struct Info {
    xauth: String,
    flags: String,
    value: String,
}

struct Router {}
impl Router {
    async fn overview(data: web::Data<ApiServer>) -> impl Responder {
        HttpResponse::Ok().body("ok")
    }

    async fn model(data: web::Data<ApiServer>) -> impl Responder {
        HttpResponse::Ok().body("model")
    }

    async fn stats_no_xauth(data: web::Data<ApiServer>) -> impl Responder {
        HttpResponse::Ok().body("stats_no_xauth")
    }

    async fn slots_no_xauth(data: web::Data<ApiServer>) -> impl Responder {
        HttpResponse::Ok().body("slots_no_xauth")
    }

    async fn x_ping(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("x_ping")
    }

    async fn stats(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("stats")
    }

    async fn slots(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("slots")
    }

    async fn start(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("start")
    }

    async fn reset_stats(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("reset_stats")
    }

    async fn force_gc(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("force_gc")
    }

    async fn shut_down(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("shut_down")
    }

    async fn log_level(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("log_level")
    }

    async fn fill_slots(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("fill_slots")
    }

    async fn set_sentinels(data: web::Data<ApiServer>, info: web::Path<Info>) -> impl Responder {
        HttpResponse::Ok().body("set_sentinels")
    }
}

// Proxy 表示 pika-proxy 的唯一元数据, 应该是作为全局静态的
async fn server_admin(proxy: &'static Proxy) -> Result<()> {
    // let server = new_api_server();
    HttpServer::new(move || {
        App::new()
            .app_data(ApiServer::from_proxy(proxy))
            .service(web::resource("proxy").route(web::get().to(overview)))
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
