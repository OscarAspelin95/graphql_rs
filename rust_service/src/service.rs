use async_graphql::{ID, Object};

// We can have these things relate to each other
// e.g., in a database table
#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub city: String,
    pub country: String,
}

impl User {
    pub fn mock() -> Self {
        Self {
            id: "1".into(),
            name: "Mock".into(),
            email: "Mock".into(),
            city: "Mock".into(),
            country: "Mock".into(),
        }
    }
}

#[Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn city(&self) -> &str {
        &self.city
    }

    async fn country(&self) -> &str {
        &self.country
    }

    /// This is where we'd actually run the database query based on user_id.
    async fn todo_lists(&self) -> Vec<Todo> {
        vec![Todo {
            id: 1,
            user_id: 1,
            name: "TodoList".to_string(),
            created: "Now".to_string(),
        }]
    }
}

#[derive(Clone)]
pub struct Todo {
    pub id: usize,
    pub user_id: usize,
    pub name: String,
    pub created: String,
}

#[Object]
impl Todo {
    async fn id(&self) -> usize {
        self.id
    }

    async fn user_id(&self) -> usize {
        self.user_id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn created(&self) -> &str {
        &self.created
    }

    /// This is where we'd actually run the db query
    /// based on e.g., todo id.
    async fn todo_tasks(&self) -> Vec<TodoTask> {
        vec![TodoTask {
            id: 1,
            todo_id: 1,
            name: "Todo Task 1".to_string(),
            status: "Status".to_string(),
            created: "Now".to_string(),
        }]
    }
}

#[derive(Clone)]
pub struct TodoTask {
    pub id: usize,
    pub todo_id: usize,
    pub name: String,
    pub status: String,
    pub created: String,
}

#[Object]
impl TodoTask {
    async fn id(&self) -> usize {
        self.id
    }

    async fn todo_id(&self) -> usize {
        self.todo_id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn status(&self) -> &str {
        &self.status
    }

    async fn created(&self) -> &str {
        &self.created
    }
}
