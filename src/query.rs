use crate::db::DB;
use crate::service::User;
use async_graphql::Object;

pub struct Query {
    pub db: DB,
}

#[Object]
impl Query {
    async fn get_user(&self, id: String) -> Option<User> {
        self.db
            .mock_data()
            .iter()
            .find(|&user| user.id.0 == id)
            .cloned()
    }

    async fn get_users(&self) -> Vec<User> {
        self.db.mock_data()
    }

    async fn get_users_by_filter(&self, country: Option<String>) -> Vec<User> {
        let users: Vec<User> = self
            .db
            .mock_data()
            .into_iter()
            .filter(|user| match &country {
                Some(c) => &user.country == c,
                None => false,
            })
            .collect();

        users
    }
}
