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
}
