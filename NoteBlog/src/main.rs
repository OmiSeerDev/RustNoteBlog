#[macro_use]
extern crate diesel;

use diesel::{Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use dotenvy::dotenv;
use std::env;
use::diesel::pg::PgConnection;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::models::{Post, NewPost, SimplifiedPost};

pub mod schema;
pub mod models;

#[get ("/hw")]
async fn hello_world()-> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new().service(hello_world)
    }).bind(("127.0.0.1", 1331)).unwrap().run().await;

    use self::schema::posts::dsl::*;

    let db_url = env::var("DATABASE_URL").expect("Database url was not found");

    let conn = &mut PgConnection::establish(&db_url).expect("Unable to connect to DB.");

/*     let new_post = NewPost {
        title: "Duodecimo post",
        body: "12 Lorem ipsum...",
        slug: "duodecimo-post",
    };
*/
//    diesel::insert_into(posts).values(new_post).get_result::<Post>(conn).expect("Fallo al insertar datos");
    
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
let updated_post = diesel::update(posts.filter(id.eq(1))).set(title.eq("Primer post")).get_result::<Post>(conn).expect("Error updating");
println!("Updated post: {:?}", updated_post);
}
