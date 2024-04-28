use rocket::fairing::AdHoc;
use rocket::futures;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use futures::{future::TryFutureExt, stream::TryStreamExt};

use rocket_db_pools::{sqlx, Connection, Database};

#[derive(Database)]
#[database("patina")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing)]
    id: i64,
    author: i64,
    reply_to: Option<i64>,
    text: String,
    #[serde(skip_deserializing)]
    likes: i64,
    #[serde(skip_deserializing)]
    timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    #[serde(skip_deserializing)]
    id: i64,
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Follow {
    follower: i64,
    followee: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Like {
    post: i64,
    user: i64,
}

#[post("/post/user", data = "<user>")]
async fn post_user(mut db: Connection<Db>, mut user: Json<User>) -> Result<Created<Json<User>>> {
    let results = sqlx::query!(
        "INSERT INTO users (name) VALUES (?) RETURNING id",
        user.name,
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    user.id = results.first().expect("returning results").id;
    Ok(Created::new("/post/user").body(user))
}

#[post("/post/post", data = "<post>")]
async fn post_post(mut db: Connection<Db>, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time should be after epoch").as_secs() as i64;
    let results = sqlx::query!(
        "INSERT INTO posts (author, text, reply_to, timestamp) VALUES (?, ?, ?, ?) RETURNING id",
        post.author,
        post.text,
        post.reply_to,
        epoch_time,
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    post.id = results.first().expect("returning results").id;
    post.timestamp = epoch_time;
    Ok(Created::new("/post/post").body(post))
}

#[post("/post/follow", data = "<follow>")]
async fn post_follow(
    mut db: Connection<Db>,
    follow: Json<Follow>,
) -> Result<Created<Json<Follow>>> {
    sqlx::query!(
        "INSERT INTO followers (follower, followee) VALUES (?, ?)",
        follow.follower,
        follow.followee,
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    Ok(Created::new("/post/follow").body(follow))
}

#[post("/post/like", data = "<like>")]
async fn post_like(mut db: Connection<Db>, like: Json<Like>) -> Result<Created<Json<Like>>> {
    sqlx::query!(
        "INSERT INTO likes (post, user) VALUES (?, ?)",
        like.post,
        like.user,
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    Ok(Created::new("/post/like").body(like))
}

#[get("/get/name/from/user/<id>")]
async fn get_name_from_user(mut db: Connection<Db>, id: i64) -> Result<Json<String>> {
    let name = sqlx::query!("SELECT name FROM users WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.name)
        .await?;

    Ok(Json(name))
}

#[get("/get/text/from/post/<id>")]
async fn get_text_from_post(mut db: Connection<Db>, id: i64) -> Result<Json<Post>> {
    let post = sqlx::query!("SELECT * FROM posts WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| Post {
            id: r.id,
            author: r.author,
            reply_to: r.reply_to,
            text: r.text,
            likes: r.likes,
            timestamp: r.timestamp,
        })
        .await?;

    Ok(Json(post))
}

#[get("/get/posts/from/user/<id>")]
async fn get_posts_from_user(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT id FROM users WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.id)
        .await?;

    let posts = sqlx::query!("SELECT * FROM posts WHERE author = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.id)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(posts))
}

#[get("/get/followers/from/user/<id>")]
async fn get_followers_from_user(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT id FROM users WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.id)
        .await?;

    let follows = sqlx::query!("SELECT follower FROM followers WHERE followee = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.follower)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(follows))
}

#[get("/get/followees/from/user/<id>")]
async fn get_followees_from_user(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT id FROM users WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.id)
        .await?;

    let follows = sqlx::query!("SELECT followee FROM followers WHERE follower = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.followee)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(follows))
}

#[get("/get/likes/from/user/<id>")]
async fn get_likes_from_user(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT id FROM users WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.id)
        .await?;

    let likes = sqlx::query!("SELECT post FROM likes WHERE user = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.post)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(likes))
}

#[get("/get/likes/from/post/<id>")]
async fn get_likes_from_post(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT id FROM posts WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.id)
        .await?;

    let likes = sqlx::query!("SELECT user FROM likes WHERE post = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.user)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(likes))
}

#[get("/get/replies/from/post/<id>")]
async fn get_replies_from_post(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT id FROM posts WHERE id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.id)
        .await?;

    let likes = sqlx::query!("SELECT id FROM posts WHERE reply_to = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.id)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(likes))
}


#[get("/search/<search>")]
async fn search(mut db: Connection<Db>, search: &str) -> Result<Json<Vec<i64>>> {
    let search_string = format!("%{}%", search);
    let posts = sqlx::query!("SELECT * FROM posts WHERE text LIKE ?", search_string)
        .fetch(&mut **db)
        .map_ok(|r| r.id)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(posts))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            // .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount(
                "/",
                routes![
                    post_user,
                    post_post,
                    post_follow,
                    post_like,
                    get_name_from_user,
                    get_text_from_post,
                    get_posts_from_user,
                    get_followers_from_user,
                    get_followees_from_user,
                    get_likes_from_user,
                    get_likes_from_post,
                    get_replies_from_post,
                    search,
                ],
            )
    })
}
