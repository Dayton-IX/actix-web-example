use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};

struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn cool_fact() -> String {
    return String::from("Ferris is a Rustacean");
}

#[get("/users/{name}")]
async fn get_user_by_name(path: web::Path<(String,)>) -> HttpResponse {
    let user_name: String = path.into_inner().0;
    println!("get_user_by_name name: {}", user_name);
    HttpResponse::Ok().body(format!("User info: {}", user_name))
}

#[get("/params/{int}/{string}")]
async fn multi_params_example(req: HttpRequest) -> Result<String> {
    let p1: u8 = req.match_info().get("int").unwrap().parse().unwrap();
    let p2: String = req.match_info().get("string").unwrap().parse().unwrap();
    Ok(format!("values: int: {}, string: {}", p1, p2))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                app_name: String::from("actix-web-example"),
            }))
            .service(web::scope("/app").route("/fact", web::get().to(cool_fact)))
            .service(index)
            .service(echo)
            .service(get_user_by_name)
            .service(multi_params_example)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
