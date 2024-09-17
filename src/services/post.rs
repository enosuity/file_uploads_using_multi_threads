use diesel::prelude::*;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post };
use crate::models::post::Post;
use crate::schema;
use rocket_dyn_templates::{context, Template};
use super::establish_connection_pg;



#[derive(Serialize, Deserialize)]
pub struct NewPost {
    title: String,
    body: String,
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/post", format = "json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
    use self::schema::posts::dsl::*;
    use Post;
    let connection = &mut establish_connection_pg();

    let new_post = Post {
        id: 1,
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: true,
    };

    diesel::insert_into(self::schema::posts::dsl::posts)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(post))
}

#[get("/posts")]
pub fn list() -> Template {
    use Post;
    let connection = &mut establish_connection_pg();
    let results = self::schema::posts::dsl::posts
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("posts", context! {posts: &results, count: results.len()})
}

