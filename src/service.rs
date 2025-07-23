use async_graphql::{ID, Object};

#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub city: String,
    pub country: String,
}

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn city(&self) -> &str {
        &self.city
    }

    async fn country(&self) -> &str {
        &self.country
    }

    async fn email(&self) -> &str {
        &self.email
    }
}
