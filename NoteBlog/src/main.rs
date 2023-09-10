#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub type Dbpool = r2d2::Pool<ConnectionManager<PgConnection>>;

use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use dotenvy::dotenv;
use std::env;
use::diesel::pg::PgConnection;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;
use crate::models::{Post, NewPostHandler};
use diesel::r2d2::{self,ConnectionManager};
use diesel::r2d2::Pool;
use self::schema::posts::dsl::*;

#[get ("/posts/")]
async fn index(pool: web::Data<Dbpool>, template: web::Data<tera::Tera>)-> impl Responder {
    
    let mut conn = pool.get().expect("No se pudo conectar a la base de datos");
    return match web::block(move || {posts.order(id).load::<Post>(&mut conn)}).await {
        Ok(data) => {

            let data = data.unwrap();
            let mut context = tera::Context::new();
            context.insert("gotten_posts", &data);

            HttpResponse::Ok().content_type("text/html").body(
                template.render("index.html", &context).unwrap())
            },
        Err(err) => HttpResponse::Ok().body(format!("{:?}", err))
};
}

#[get ("/watch/{post_slug}")]
async fn watch_post(
    pool: web::Data<Dbpool>,
    template: web::Data<tera::Tera>,
    post_slug: web::Path<String>
)-> impl Responder {

    let slug_url = post_slug.into_inner();

    let mut conn = pool.get().expect("No se pudo conectar a la base de datos");
    return match web::block(move || {posts.filter(slug.eq(slug_url)).load::<Post>(&mut conn)}).await {
        Ok(data) => {
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];
            
            let mut context = tera::Context::new();
            context.insert("gotten_post", data);

            HttpResponse::Ok().content_type("text/html").body(
                template.render("watchpost.html", &context).unwrap())
            },
        Err(err) => HttpResponse::Ok().body(format!("{:?}", err))
};
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

    let pool = Pool::builder().build(connection).expect("No se pudo construir el Pool.");

    HttpServer::new(move || {
        
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
        .service(index)
        .service(create_post)
        .service(watch_post)
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(tera))
    }).bind(("localhost", 1333)).unwrap().run().await

}
