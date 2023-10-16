use std::time::SystemTime;

use diesel::prelude::*;
use diesel::result::Error as dbError;

use crate::config::db::PgClient;
use crate::model::User;
use crate::schema::users;

// for user
impl PgClient {
    pub async fn register(
        &self,
        account: String,
        nickname: String,
        password: String,
    ) -> Result<(), dbError> {
        let _ = diesel::insert_into(users::table)
            .values((
                users::account.eq(account),
                users::nickname.eq(nickname),
                users::password.eq(password),
            ))
            .execute(&mut self.get_conn())?;

        Ok(())
    }

    pub async fn login(
        &self,
        account: String,
        password: String,
        refresh_token: String,
    ) -> Option<User> {
        diesel::update(users::table)
            .filter(users::account.eq(account).and(users::password.eq(password)))
            .set((
                users::last_login_time.eq(SystemTime::now()),
                users::refresh_token.eq(refresh_token),
            ))
            .get_result::<User>(&mut self.get_conn())
            .optional()
            .unwrap()
    }

    pub async fn find_by_account(&self, account: &String) -> Option<User> {
        users::table
            .filter(users::account.eq(account))
            .first(&mut self.get_conn())
            .optional()
            .unwrap()
    }

    pub async fn find_by_refresh_token(&self, refresh_token: &String) -> Option<User> {
        users::table
            .filter(users::refresh_token.eq(refresh_token))
            .first(&mut self.get_conn())
            .optional()
            .unwrap()
    }

    // pub async fn update_user(&self, user: &User) -> Result<(), diesel::result::Error> {
    //     let _ = diesel::update(users::table)
    //         .filter(users::user_id.eq(user.user_id))
    //         .set(user)
    //         .execute(&mut self.get_conn())?;

    //     Ok(())
    // }

    // pub async fn delete_user(&self, user_id: i32) -> Result<(), diesel::result::Error> {
    //     let _ = diesel::delete(users::table)
    //         .filter(users::user_id.eq(user_id))
    //         .execute(&mut self.get_conn())?;

    //     Ok(())
    // }

    // pub async fn get_user_by_id(
    //     &self,
    //     user_id: i32,
    // ) -> Result<Option<User>, diesel::result::Error> {
    //     users::table
    //         .filter(users::user_id.eq(user_id))
    //         .first(&mut self.get_conn())
    //         .optional()
    // }
}
