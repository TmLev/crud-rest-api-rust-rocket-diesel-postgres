use rocket::serde::{Deserialize, Serialize};

use diesel::{AsChangeset, Insertable, Queryable};

use crate::schema::artists;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "artists"]
pub struct NewArtist {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "artists"]
pub struct UpdatedArtist {
    pub name: Option<String>,
    pub description: Option<String>,
}
