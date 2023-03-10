#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

// ruta principal
#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from Rust and MongoDB")))
}

// módulos
mod api; // es para modularizar los controladores de la API
mod models; // es para modularizar las lógicas de datos
mod repository; // es para modularizar lógicas de bases de datos

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
