/*
    se importan las dependencias necesarias
*/
use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{extjson::de::Error},
    results::{InsertOneResult},
    sync::{Client, Collection}
};
use crate::models::user_model::User;

/*
    se crea un MongoRepo con un campo col para acceder a la colección MongoDB
*/
pub struct MongoRepo {
    col: Collection<User>,
}

/*
    se crea un bloque de implementación que agrega métodos a la estructura MongoRepo
*/
impl MongoRepo {
    /*
        se agrega un método init al bloque de implementación para cargar la variable de entorno, 
        crea una conexión con la base de datos y devuelve una instancia de la estructura MongoRepo
    */
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            _ => format!("error loading env variable")
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rust-api");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    /*
        se agrega un método create_user que toma un self y new_user como parámetros y devuelve el usuario 
        creado o un error. Dentro del método, creamos un nuevo documento usando la estructura User. Entonces 
        usamos el self haciendo una referencia a la estructura MongoRepo para acceder a la función insert_one 
        de la colección para crear un nuevo usuario y manejar errores. Finalmente devolvimos la información 
        de usuario creada
    */
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("error creating user");
        Ok(user)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("error getting users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }
}