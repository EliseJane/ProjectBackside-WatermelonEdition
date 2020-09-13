use crate::api_error::ApiError;
use crate::db;
use crate::schema::image;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
pub struct ImageMessage {
    pub file_name: String,
    pub file_size: i32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "image"]
pub struct UpdateImage {
    pub file_name: String,
    pub file_size: i32,
    pub data: Vec<u8>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "image"]
pub struct Image {
    pub id: Uuid,
    pub file_name: String,
    pub file_size: i32,
    pub data: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub owner: String,
}

impl Image {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::establish_connection();
        dotenv().ok();
        let owner = env::var("USERNAME").expect("Username not set");

        let images = image::table
            .filter(image::owner.eq(owner))
            .load::<Image>(&conn)?;

        Ok(images)
    }

    // pub fn find(id: Uuid) -> Result<Self, ApiError> {
    //     let conn = db::establish_connection();
    //
    //     let user = user::table
    //         .filter(user::id.eq(id))
    //         .first(&conn)?;
    //
    //     Ok(user)
    // }

    // pub fn create(image: ImageMessage) -> Result<Self, ApiError> {
    //     let conn = db::establish_connection();
    //
    //     let image = Image::from(image);
    //     let user = diesel::insert_into(user::table)
    //         .values(user)
    //         .get_result(&conn)?;
    //
    //     Ok(user)
    // }

    // curl -X POST -F 'image=@/path/to/pictures/picture.jpg' http://domain.tld/upload

    // pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
    //     let conn = db::establish_connection();
    //
    //     let update_user = UpdateUserMessage::from(user);
    //
    //     let user = diesel::update(user::table)
    //         .filter(user::id.eq(id))
    //         .set(update_user)
    //         .get_result(&conn)?;
    //
    //     Ok(user)
    // }

    // pub fn delete(id: Uuid) -> Result<usize, ApiError> {
    //     let conn = db::establish_connection();
    //
    //     let res = diesel::delete(
    //         user::table
    //             .filter(user::id.eq(id))
    //     )
    //         .execute(&conn)?;
    //
    //     Ok(res)
    // }
}

impl From<ImageMessage> for Image {
    fn from(image: ImageMessage) -> Self {
        dotenv().ok();
        let owner = env::var("USERNAME").expect("Username not set");
        Image {
            id: Uuid::new_v4(),
            file_name: image.file_name,
            file_size: image.file_size,
            data: image.data,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            owner,
        }
    }
}

impl From<ImageMessage> for UpdateImage {
    fn from(image: ImageMessage) -> Self {
        UpdateImage {
            file_name: image.file_name,
            file_size: image.file_size,
            data: image.data,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

#[test]
fn test_image_rest() {
    let images = Image::find_all().unwrap();
    assert!(images.is_empty());
}