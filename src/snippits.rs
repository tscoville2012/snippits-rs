extern crate redis;
extern crate uuid;

use std::collections::HashMap;
use self::redis::Commands;
use rocket_contrib::Template;
use rocket::request::Form;
use rocket::response::Redirect;
use self::uuid::Uuid;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain!{
    foreign_links {
        RedisError(::snippits::redis::RedisError);
    }
}

#[derive(Serialize, FromForm, Debug)]
pub struct SnippitForm {
    title: String,
    body: String,
    snippit_language: String,
}

#[derive(FromForm, Debug)]
pub struct UpvoteForm {
    snippit_key: String,
}

#[derive(Serialize)]
struct Snippit {
    title: Option<String>,
    code_snippit: Option<String>,
    snippit_language: Option<String>,
    snippit_key: String,
    vote_count: u32,
}

#[derive(Serialize)]
struct SnippitIndexContext {
    snippits: Vec<Snippit>,
}

#[get("/")]
pub fn index() -> Result<Template> {

    let mut v = vec![];

    // TODO: use r2d2 to get a connection pool
    let conn = get_redis_conn().chain_err(|| "problem getting redis connection")?;

    let iter: Vec<(String, u32)> =
        get_redis_keys(&conn).chain_err(|| "problem getting redis keys")?;

    for x in iter {
        let map = get_redis_hash(&x.0, &conn).chain_err(|| "problem getting redis hash")?;

        v.push(Snippit {
            title: map.get("created_by").cloned(),
            code_snippit: map.get("code_snippit").cloned(),
            snippit_language: map.get("snippit_language").cloned(),
            snippit_key: x.0,
            vote_count: x.1,
        })
    }
    println!("There are {} snippits", v.len());
    let context = SnippitIndexContext { snippits: v };

    Ok(Template::render("index", &context))
}

// This should probably be a put but going with post
// because im lazy
#[post("/upvote", data = "<post>")]
pub fn up_vote(post: Form<UpvoteForm>) -> Result<Redirect> {
    let conn = get_redis_conn().chain_err(|| "problem getting redis connection")?;

    let post = post.get();
    println!("upvoting key: {}", post.snippit_key);
    do_up_vote(&post.snippit_key, &conn).chain_err(|| "problem upvoting")?;
    Ok(Redirect::to("/"))
}

#[get("/new_snippit")]
pub fn new_snippit() -> Template {
    let context = SnippitForm {
        body: "".to_string(),
        title: "".to_string(),
        snippit_language: "".to_string(),
    };

    Template::render("new_snippit", &context)
}

#[post("/new_snippit", data = "<post>")]
pub fn new_post_submit(post: Form<SnippitForm>) -> Result<Redirect> {
    let post = post.get();

    // println!("{}", post.body.as_str());

    let conn = get_redis_conn().chain_err(|| "problem getting redis connection")?;

    let tuples = [
        ("created_by", post.title.as_str()),
        ("code_snippit", post.body.as_str()),
        ("snippit_language", post.snippit_language.as_str()),
    ];

    save_new_snippit(&conn, &tuples).expect("could not save new snippit to redis");

    Ok(Redirect::to("/"))
}

fn get_redis_conn() -> Result<redis::Connection> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let conn = try!(client.get_connection());

    Ok(conn)
}

fn do_up_vote<'a>(member: &'a str, conn: &redis::Connection) -> Result<()> {
    let _: () = try!(conn.zincr("upvotes", member, 1));

    Ok(())
}

// TODO: convet this to sorted set
fn get_redis_hash<'a>(
    key: &'a str,
    conn: &redis::Connection,
) -> redis::RedisResult<HashMap<String, String>> {
    let map: HashMap<String, String> = try!(conn.hgetall(key));

    Ok(map)
}

fn get_redis_keys(conn: &redis::Connection) -> redis::RedisResult<Vec<(String, u32)>> {
    Ok(try!(conn.zrevrange_withscores("upvotes", 0, -1)))
}

fn generate_new_snippit_key() -> String {
    return format!("snippit:{}", Uuid::new_v4());
}

fn save_new_snippit<'a>(conn: &redis::Connection, tuples: &[(&'a str, &'a str)]) -> Result<()> {
    let key = generate_new_snippit_key();

//pipe line this
    let _: () = try!(conn.zadd("upvotes", &key, 1));
    let _: () = try!(conn.hset_multiple(&key, tuples));
    Ok(())
}
