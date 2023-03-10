#[macro_use] extern crate rocket;

// ruta principal
#[get("/hello")]
fn index() -> &'static str {
    "hello world"
}

// módulos de las otras rutas 

// esta macro lanza la función principal para ejecutarse
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", 
        routes![
            index,
        ]
    )
}
