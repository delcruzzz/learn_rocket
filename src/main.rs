#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

// ruta principal
#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from Rust and MongoDB")))
}

// módulos de las otras rutas 

// esta macro lanza la función principal para ejecutarse
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", 
        routes![
            hello,
        ]
    )
}
