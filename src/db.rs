use crate::service::User;
use async_graphql::ID;
pub struct DB {}

impl DB {
    /// Mock data for demonstration purposes.
    pub fn mock_data(&self) -> Vec<User> {
        let mocked_data: Vec<User> = (0..10)
            .map(|user_id| {
                return User {
                    id: ID::from(user_id),
                    // This is not very efficient, but is only for mocking purposes.
                    name: "user_".to_string() + &user_id.to_string() + "_name",
                    city: "user_".to_string() + &user_id.to_string() + "_city",
                    country: "user_".to_string() + &user_id.to_string() + "_country",
                    email: "user_".to_string() + &user_id.to_string() + "@email.com",
                };
            })
            .collect();

        return mocked_data;
    }
}
