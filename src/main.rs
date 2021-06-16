use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

async fn greet() -> impl Responder {
    let html = "
     <!DOCTYPE html>
<html>
  <head>
    <meta charset='UTF-8'>
    <title>Andaman Effect</title>
    <script>
        class State {
              constructor(){
                this.camera = false;
                this.loop = false;
               }
              get_camera(){
                this.set_camera();
                return this.camera;
              }
              run(){
                fetch('/' +this.get_camera())
                   .catch(console.log(error));
              }
              set_camera(){
                this.camera = !this.camera;
              }
              set_loop(){
                this.loop = !this.loop;
             }
             toggle_loop(){
                 state.set_loop();
                 while (state.loop) {
                     state.run();
                     }
            }

        }
             let state = new State();
    </script>
    <style>
        * {
            margin: 0px;
            padding: 0px;
            box-sizing: border-box;
        }
        button {
            display: inline-block;
            width: 80%;
            height: 30vh;
            background-color: white;
            border: 1px solid black;
            font-size: 5rem;
        }
        button:hover {
            background: yellow;
            cursor: grab;
        }
    </style>
    </head>
    <body>
       <button onclick='state.run()'>Toggle Camera</button>
    </body>
</html>
        ";
    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn start() -> impl Responder {
    Command::new("andaman")
        .args(&["-s"])
        .spawn()
        .expect("failed to start");
    HttpResponse::Ok()
}

async fn stop() -> impl Responder {
    Command::new("andaman").output().expect("failed to stop");
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 0.0.0.0:5000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/true", web::get().to(start))
            .route("/false", web::get().to(stop))
            .wrap(
                actix_cors::Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
                    .send_wildcard()
                    .finish(),
            )
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
