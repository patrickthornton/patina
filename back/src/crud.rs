use rocket::fairing::AdHoc;
use rocket::futures;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use futures::{future::TryFutureExt, stream::TryStreamExt};

use rocket_db_pools::{sqlx, Connection, Database};

// number of seconds in 36 hours
const GRADIENT_DAILY_SPAN: i64 = 60 * 60 * 36;

#[derive(Database)]
#[database("patina")]
struct Db(sqlx::SqlitePool);

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing)]
    post_id: i64,
    author: String,
    text: String,
    hue: i64,
    reply_to: Option<i64>,
    #[serde(skip_deserializing)]
    likes: i64,
    #[serde(skip_deserializing)]
    timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    #[serde(skip_deserializing)]
    user_id: i64,
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
    liked: bool,
}

#[post("/post/user", data = "<user>")]
async fn post_user(mut db: Connection<Db>, mut user: Json<User>) -> Result<Created<Json<User>>> {
    let results = sqlx::query!(
        "INSERT INTO users (name) VALUES (?) RETURNING user_id",
        user.name,
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    user.user_id = results.first().expect("returning results").user_id;
    Ok(Created::new("/post/user").body(user))
}

#[post("/post/post", data = "<post>")]
async fn post_post(mut db: Connection<Db>, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    let epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_secs() as i64;

    let user_id = sqlx::query!("SELECT user_id FROM users WHERE name = ?", post.author)
        .fetch_one(&mut **db)
        .map_ok(|r| r.user_id)
        .await?;

    let results = sqlx::query!(
        "INSERT INTO posts (author, text, hue, reply_to, timestamp) VALUES (?, ?, ?, ?, ?) RETURNING post_id",
        user_id,
        post.text,
        post.hue,
        post.reply_to,
        epoch_time,
    )
    .fetch(&mut **db)
    .try_collect::<Vec<_>>()
    .await?;

    post.post_id = results.first().expect("returning results").post_id;
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
    //if post and user exists, then change the bool value
    //if it doesn't:
    let existing_like = sqlx::query!(
        "SELECT liked FROM likes WHERE post = ? AND user = ?",
        like.post,
        like.user
    )
    .fetch_optional(&mut **db)
    .await?;

    match existing_like {
        Some(_) => {
            sqlx::query!(
                "UPDATE likes SET liked = ? WHERE post = ? AND user = ?",
                like.liked,
                like.post,
                like.user,
            )
            .execute(&mut **db)
            .await?;
        }
        None => {
            sqlx::query!(
                "INSERT INTO likes (post, user, liked) VALUES (?, ?, ?)",
                like.post,
                like.user,
                like.liked
            )
            .fetch(&mut **db)
            .try_collect::<Vec<_>>()
            .await?;
        }
    }
    Ok(Created::new("/post/like").body(like))
}

#[get("/get/user/from/name/<name>")]
async fn get_user_from_name(mut db: Connection<Db>, name: &str) -> Result<Json<i64>> {
    let user_id = sqlx::query!("SELECT user_id FROM users WHERE name = ?", name)
        .fetch_one(&mut **db)
        .map_ok(|r| r.user_id)
        .await?;

    Ok(Json(user_id))
}

#[get("/get/followers/from/user/<id>")]
async fn get_followers_from_user(mut db: Connection<Db>, id: i64) -> Result<Json<Vec<i64>>> {
    let id = sqlx::query!("SELECT user_id FROM users WHERE user_id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.user_id)
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
    let id = sqlx::query!("SELECT user_id FROM users WHERE user_id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.user_id)
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
    let id = sqlx::query!("SELECT user_id FROM users WHERE user_id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.user_id)
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
    let id = sqlx::query!("SELECT post_id FROM posts WHERE post_id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.post_id)
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
    let id = sqlx::query!("SELECT post_id FROM posts WHERE post_id = ?", id)
        .fetch_one(&mut **db)
        .map_ok(|r| r.post_id)
        .await?;

    let likes = sqlx::query!("SELECT post_id FROM posts WHERE reply_to = ?", id)
        .fetch(&mut **db)
        .map_ok(|r| r.post_id)
        .try_collect::<Vec<_>>()
        .await?;

    Ok(Json(likes))
}

// higher level API functions; called by each page

// returns all posts within the last GRADIENT_DAILY_SPAN seconds of <time>,
// sorted into a gradient;
// to be enhanced with some kind of rec algo
#[get("/gradient/<time>")]
async fn gradient(mut db: Connection<Db>, time: i64) -> Result<Json<Vec<Post>>> {
    let posts = sqlx::query!(
        "SELECT * FROM posts
        JOIN users ON users.user_id = posts.author
        WHERE ? - posts.timestamp < ?
        ORDER BY posts.hue",
        time,
        GRADIENT_DAILY_SPAN
    )
    .fetch(&mut **db)
    .map_ok(|r| Post {
        post_id: r.post_id,
        author: r.name,
        text: r.text,
        hue: r.hue,
        reply_to: r.reply_to,
        likes: r.likes,
        timestamp: r.timestamp,
    })
    .try_collect::<Vec<_>>()
    .await?;

    Ok(Json(posts))
}

// returns all posts from a given user, sorted by time
#[get("/user/<name>")]
async fn user(mut db: Connection<Db>, name: &str) -> Result<Json<Vec<Post>>> {
    let posts = sqlx::query!(
        "SELECT * FROM posts
            JOIN users ON users.user_id = posts.author
            WHERE author =
            (
                SELECT users.user_id FROM users
                WHERE users.name = ?
            ) ORDER BY timestamp DESC",
        name
    )
    .fetch(&mut **db)
    .map_ok(|r| Post {
        post_id: r.post_id,
        author: r.name,
        text: r.text,
        hue: r.hue,
        reply_to: r.reply_to,
        likes: r.likes,
        timestamp: r.timestamp,
    })
    .try_collect::<Vec<_>>()
    .await?;

    Ok(Json(posts))
}

// returns all liked posts by a given user, sorted by time
#[get("/user/<name>/liked")]
async fn liked(mut db: Connection<Db>, name: &str) -> Result<Json<Vec<Post>>> {
    let id = sqlx::query!("SELECT user_id FROM users WHERE name = ?", name)
        .fetch_one(&mut **db)
        .await?;

    let posts = sqlx::query!(
        "SELECT posts.*, users.name FROM posts
        JOIN users ON users.user_id = posts.author
        WHERE post_id IN
        (
            SELECT post FROM likes
            WHERE user = ?
        ) ORDER BY timestamp DESC",
        id.user_id
    )
    .fetch(&mut **db)
    .map_ok(|r| Post {
        post_id: r.post_id,
        author: r.name,
        text: r.text,
        hue: r.hue,
        reply_to: r.reply_to,
        likes: r.likes,
        timestamp: r.timestamp,
    })
    .try_collect::<Vec<_>>()
    .await?;

    Ok(Json(posts))
}

// implements a search
#[get("/search/<search>")]
async fn search(mut db: Connection<Db>, search: &str) -> Result<Json<Vec<i64>>> {
    let search_string = format!("%{}%", search);
    let posts = sqlx::query!(
        "SELECT * FROM posts WHERE text LIKE ? ORDER BY hue",
        search_string
    )
    .fetch(&mut **db)
    .map_ok(|r| r.post_id)
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
                    get_user_from_name,
                    get_followers_from_user,
                    get_followees_from_user,
                    get_likes_from_user,
                    get_likes_from_post,
                    get_replies_from_post,
                    search,
                    gradient,
                    user,
                    liked,
                ],
            )
    })
}
