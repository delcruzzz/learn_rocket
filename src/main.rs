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

// api y lógica de la base de datos
use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;

// esta macro lanza la función principal para ejecutarse
#[launch]
fn rocket() -> _ {

    /*
        se necesita modificar nuestro punto de entrada de la aplicación para 
        incluir el controlador create_user
    */

    /*
        se crea una variable db para establecer conxión con MongoDB llamando 
        al método init() y lo agrega a la función manage para hacer que el 
        estado de la base de datos esté disponible en todo el alcance de la 
        aplicación
    */
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/api",
        routes![
            hello,
            create_user
        ]
    )
}
