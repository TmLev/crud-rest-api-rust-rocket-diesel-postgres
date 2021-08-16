use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
};

use diesel::prelude::*;

use rest_api::{
    models::{Artist, NewArtist, UpdatedArtist},
    schema::artists,
    ApiError, PgConnection,
};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        // State
        .attach(PgConnection::fairing())
        // Routes
        .mount(
            "/artists",
            rocket::routes![list, retrieve, create, update, destroy],
        )
}

#[rocket::get("/")]
async fn list(connection: PgConnection) -> Json<Vec<Artist>> {
    connection
        .run(|c| artists::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch artists")
}

#[rocket::get("/<id>")]
async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Artist>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| artists::table.filter(artists::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::post("/", data = "<artist>")]
async fn create(
    connection: PgConnection,
    artist: Json<NewArtist>,
) -> Result<Created<Json<Artist>>, Json<ApiError>> {
    connection
        .run(move |c| {
            diesel::insert_into(artists::table)
                .values(&artist.into_inner())
                .get_result(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

#[rocket::patch("/<id>", data = "<artist>")]
async fn update(
    connection: PgConnection,
    id: i32,
    artist: Json<UpdatedArtist>,
) -> Result<Json<Artist>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            diesel::update(artists::table.find(id))
                .set(&artist.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::delete("/<id>")]
async fn destroy(connection: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let affected = diesel::delete(artists::table.filter(artists::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
