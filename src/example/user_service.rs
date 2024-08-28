use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult};

use crate::service::{generic_service::GenericService, service::Service};

use super::{user_dao::UserDao, user_entity};

pub struct UserService {
    generic_service: GenericService<user_entity::Entity, i64, UserDao>,
}

impl UserService {
    pub fn new() -> Self {
        let user_dao = UserDao::new();
        Self {
            generic_service: GenericService::new(user_dao),
        }
    }
}

#[async_trait]
impl Service<user_entity::Entity, i64> for UserService {
    async fn find_by_id(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<Option<user_entity::Model>, DbErr> {
        self.generic_service.find_by_id(db, id).await
    }

    async fn create(
        &self,
        db: &DatabaseConnection,
        model: user_entity::Model,
    ) -> Result<user_entity::Model, DbErr> {
        self.generic_service.create(db, model).await
    }

    async fn update(
        &self,
        db: &DatabaseConnection,
        model: user_entity::Model,
    ) -> Result<user_entity::Model, DbErr> {
        self.generic_service.update(db, model).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: i64) -> Result<DeleteResult, DbErr> {
        self.generic_service.delete(db, id).await
    }
}
