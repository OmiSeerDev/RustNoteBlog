#[macro_use]
extern crate diesel;

use diesel::{Connection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;
use::diesel::pg::PgConnection;
use::diesel::prelude;

use crate::models::Post;

pub mod schema;
pub mod models;

fn main() {
    dotenv().ok();

    use self::schema::posts::dsl::*;

    let db_url = env::var("DATABASE_URL").expect("Database url was not found");

    let conn = &mut PgConnection::establish(&db_url).expect("Unable to connect to DB.");
    

   let post_result = posts.load::<Post>(conn).expect("Wrong query");

   for post in post_result {
    println!("{}", post.title);
   }
    println!("C");


}
