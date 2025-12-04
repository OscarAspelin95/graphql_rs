use crate::context::Context;
use juniper::{graphql_object, FieldResult};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn mock() -> Self {
        Self {
            id: uuid::Uuid::now_v7().to_string(),
            first_name: "Mock".into(),
            last_name: "Mock".into(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> &str {
        &self.id
    }

    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }

    fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }

    async fn todo_lists(&self, context: &Context) -> FieldResult<Vec<Todo>> {
        let pool = &context.db;

        let user_uuid =
            uuid::Uuid::parse_str(&self.id).map_err(|e| format!("Invalid UUID: {}", e))?;

        let todo_lists = sqlx::query_as!(
            Todo,
            "SELECT id, user_id, name, created_at, updated_at FROM public.todo_lists WHERE user_id = $1",
            user_uuid
        )
        .fetch_all(pool)
        .await?;

        Ok(todo_lists.into_iter().map(Todo::from).collect())
    }
}

pub struct Todo {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[graphql_object(context = Context)]
impl Todo {
    fn id(&self) -> &str {
        &self.id
    }

    fn user_id(&self) -> &str {
        &self.user_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }

    fn todo_tasks(&self) -> Vec<TodoTask> {
        vec![TodoTask {
            id: 1,
            todo_id: 1,
            name: "Todo Task 1".to_string(),
            status: "Status".to_string(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }]
    }
}

#[derive(Clone)]
pub struct TodoTask {
    pub id: i32,
    pub todo_id: i32,
    pub name: String,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[graphql_object(context = Context)]
impl TodoTask {
    fn id(&self) -> i32 {
        self.id
    }

    fn todo_id(&self) -> i32 {
        self.todo_id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn status(&self) -> &str {
        &self.status
    }

    fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}
