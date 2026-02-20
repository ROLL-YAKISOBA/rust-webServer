use actix_web::{get, App, HttpResponse, HttpServer, Responder,post};
use std::fs;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}
#[get("/bye")]
async fn bye() -> impl Responder {
    HttpResponse::Ok().body("さようなら！また来てね！")
}

#[get("/html-file")]
async fn html_file() -> impl Responder {
match fs::read_to_string("main.html") {
  Ok(content) => {
                
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(content)
            }
    Err(_e) => {


println!("{}",_e);
        HttpResponse::InternalServerError().body("ファイルが見つかりませんでした...")
            }
        }
    }


#[post("/ping")]
async fn ping() -> impl Responder {
   HttpResponse::Ok().body("pong!")
}


/*#[get("/home")]
async fn home() -> impl Responder {
HttpResponse::OK().
}
*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on 0.0.0.0:5000");
    HttpServer::new(|| {
        App::new().service(hello)
        .service(bye)
        .service(ping)
     .service(html_file)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
