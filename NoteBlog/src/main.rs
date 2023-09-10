#[macro_use]
extern crate diesel;

use diesel::{RunQueryDsl, QueryDsl};
use dotenvy::dotenv;
use std::env;
use::diesel::pg::PgConnection;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub mod schema;
pub mod models;
use crate::models::{Post, NewPostHandler};
use diesel::r2d2::{self,ConnectionManager};
use diesel::r2d2::Pool;

pub type Dbpool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::schema::posts::dsl::*;

#[get ("/posts/")]
async fn index(pool: web::Data<Dbpool>)-> impl Responder {
    let mut conn = pool.get().expect("No se pudo conectar a la base de datos");
    match web::block(move || {posts.order(id).load::<Post>(&mut conn)}).await {
        Ok(data) => HttpResponse::Ok().body(
            format!("{:?}\n", data)),
        Err(err) => HttpResponse::Ok().body(format!("{:?}", err))
}
}

#[post ("/posts/new-post/")]
async fn create_post(pool :web::Data<Dbpool>, item: web::Json<NewPostHandler>)-> impl Responder {
    let mut conn = pool.get().expect("No se pudo conectar a la base de datos");
    
    println!("{:?}", item);

    match web::block(move || {Post::create_post(&mut conn, &item)}).await {
        Ok(data) => HttpResponse::Ok().body(
            format!("{:?}\n", data)),
        Err(err) => HttpResponse::Ok().body(format!("{:?}", err))
    }
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable de entorno DATABASE_URL no existe.");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    // El POOL sirve para compartir la conexión con otros servicios
    let pool = Pool::builder().build(connection).expect("No se pudo construir el Pool.");

    HttpServer::new(move || {
        // Compartimos el pool de conexión a cada endpoint
        App::new()
        .service(index)
        .service(create_post)
        .app_data(web::Data::new(pool.clone()))
    }).bind(("localhost", 1333)).unwrap().run().await

}
