use crate::context::Context;
use juniper::{graphql_object, FieldResult};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn mock() -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            first_name: "Mock".into(),
            last_name: "Mock".into(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[graphql_object(context = Context)]
impl User {
    fn id(&self) -> &Uuid {
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

        let todo_lists = sqlx::query_as!(
            Todo,
            "SELECT id, user_id, name, created_at, updated_at FROM public.todo_lists WHERE user_id = $1",
            self.id
        )
        .fetch_all(pool)
        .await?;

        Ok(todo_lists)
    }
}

pub struct Todo {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[graphql_object(context = Context)]
impl Todo {
    fn id(&self) -> &Uuid {
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
            id: Uuid::now_v7(),
            todo_id: Uuid::now_v7(),
            name: "Todo Task 1".to_string(),
            status: "Status".to_string(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }]
    }
}

#[derive(Clone)]
pub struct TodoTask {
    pub id: Uuid,
    pub todo_id: Uuid,
    pub name: String,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[graphql_object(context = Context)]
impl TodoTask {
    fn id(&self) -> Uuid {
        self.id
    }

    fn todo_id(&self) -> Uuid {
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
