#[macro_use]
extern crate diesel;

use diesel::{Connection, RunQueryDsl, QueryDsl, ExpressionMethods, connection, Insertable};
use dotenvy::dotenv;
use std::env;
use::diesel::pg::PgConnection;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

pub mod schema;
pub mod models;
use crate::models::{Post, NewPost, SimplifiedPost};
use diesel::r2d2::{self,ConnectionManager};
use diesel::r2d2::Pool;

pub type Dbpool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::schema::posts;
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
async fn create_post(pool :web::Data<Dbpool>)-> impl Responder {
    let mut conn = pool.get().expect("No se pudo conectar a la base de datos");
    
    let new_post = NewPost {
        title: "Décimo tercer post", body:"13",
         slug: "decimotercer-post"
        };

    match web::block(move || {
        diesel::insert_into(posts).values(new_post).get_result::<Post>(&mut conn)}).await {
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
/*
    
    let conn = &mut PgConnection::establish(&db_url).expect("Unable to connect to DB.");


     let new_post = NewPost {
        title: "Decimo tercer post",
        body: "13 Lorem ipsum...",
        slug: "decimo tercer-post",
    };

    diesel::insert_into(posts).values(new_post).get_result::<Post>(conn).expect("Fallo al insertar datos");
    
// Muestra todos los registros de posts
   let all_posts = posts.order(id).load::<Post>(conn).expect("Consulta incorrecta");
   for post in all_posts {
    println!("{:?}", post);
   }
// Muestra un número limitado de registros
let limited_posts = posts.limit(2).load::<Post>(conn).expect("Error al mostrar un registro'");
for post in limited_posts {
    println!("El registo pedido es: {:?}", post);
}

// Hacer queries de columnas usando una tupla ( , )
let column_select = posts.order(id).select((slug, body)).load::<SimplifiedPost>(conn).expect("Error al mostrar columnas");
for post in column_select {
    println!("Los registros por slug y body: {:?}", post);
}

//Query con where id = 9
let where_limited_query = posts.filter(id.eq(9)).limit(1).load::<Post>(conn).expect("");
for post in where_limited_query {
    println!("The where asserted post is: {:?}", post);
}

//Update posts
let updated_post = diesel::update(posts.filter(id.eq(1))).set(title.eq("Primer post")).get_result::<Post>(conn).expect("Error al actualizar registros");
println!("Updated post: {:?}", updated_post);*/
}
