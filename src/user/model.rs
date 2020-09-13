use crate::api_error::ApiError;
use crate::db;
use crate::schema::user;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset, Clone)]
#[table_name = "user"]
pub struct UserMessage {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UpdateUserMessage {
    pub user_name: String,
    pub password: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, PartialEq)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::establish_connection();

        let users = user::table
            .load::<User>(&conn)?;

        Ok(users)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let user = user::table
            .filter(user::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let user = User::from(user);
        let user = diesel::insert_into(user::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::establish_connection();

        let update_user = UpdateUserMessage::from(user);

        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(update_user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::establish_connection();

        let res = diesel::delete(
            user::table
                .filter(user::id.eq(id))
        )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: Uuid::new_v4(),
            user_name: user.user_name,
            password: user.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}

impl From<UserMessage> for UpdateUserMessage {
    fn from(user: UserMessage) -> Self {
        UpdateUserMessage {
            user_name: user.user_name,
            password: user.password,
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}

#[test]
fn test_user_rest() {
    let new_user = UserMessage { user_name: "test name".to_string(), password: "test-password".to_string() };
    let user = User::create(new_user.clone()).unwrap();
    assert_eq!(user.user_name, new_user.user_name);
    assert_eq!(user.password, new_user.password);

    let users = User::find_all().unwrap();
    let test_user = users.iter().find(|u| u.user_name == user.user_name).unwrap();
    assert_eq!(test_user.to_owned(), &user);

    let user_id = user.id;
    let update_user = UserMessage { user_name: "new name".to_string(), password: "new-password".to_string() };
    let updated_user = User::update(user_id, update_user).unwrap();

    let get_user = User::find(user_id).unwrap();
    assert_eq!(get_user, updated_user);

    let something = User::delete(user_id).unwrap();
    assert!(something > 0);
}