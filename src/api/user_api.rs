/*
  en segundo lugar, necesitamos crer un controlador que use el método 
  create_user de la carpeta repository para crear un usuario. Para hacer esto, 
  necesitamos navegar hacia la carpta api, y en esta carpeta, se crea un 
  archivo user_api.rs
*/

// se importan las dependencias necesarias
use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

/*
  se utiliza la macro de enrutamiento para especificar el método 
  HTTP, la ruta e indica que elncontrolador espera datos del cuerpo 
  después crea un manejador create_user, que toma el db, un tipo 
  para el MongoRepo y un new_user como parámetros. Dentro del controlador, 
  se crea una variable data la cual sirve para crear el usuario, se 
  inserta en la base de datos utilizando el método db.create_user, y devuelve 
  la respuesta correcta, si fue exitoso o hubo error al ingreso de los datos.

  PD: las estructuras &State y Json para definir el parámetro es para 
  administrar el estado de aplicación a través de rutas y extraer datos JSON 
  de cargas útiles de solicitud, respectivamente.
*/
#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user_detail = db.create_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        _ => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
      Ok(users) => Ok(Json(users)),
      _ => Err(Status::InternalServerError),
    }
}