use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, QueryFilter};

use crate::{
    dto::request::PageQueryParam,
    repo::{generic_repo::GenericRepo, repo::Repo},
};

use super::user_entity;

/// 如果完全复用base实现 可按照下面的写法
// pub type UserDao = GenericRepo<user_entity::Entity, i64>;

/// 自定义实现
pub struct UserDao {
    generic_dao: GenericRepo<user_entity::Entity, i64>, // Use the GenericDao for CRUD operations
}

impl UserDao {
    pub fn new() -> Self {
        Self {
            generic_dao: GenericRepo::new(),
        }
    }

    // 添加自定实现
    pub async fn find_by_email(
        &self,
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user_entity::Model>, DbErr> {
        user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(db)
            .await
    }
}

#[async_trait]
impl Repo<user_entity::Entity, i64> for UserDao {
    async fn find_by_id(
        &self,
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<Option<user_entity::Model>, DbErr> {
        self.generic_dao.find_by_id(db, id).await
    }

    async fn find_list(&self, db: &DatabaseConnection) -> Result<Vec<user_entity::Model>, DbErr> {
        self.generic_dao.find_list(db).await
    }

    async fn find_page(
        &self,
        db: &DatabaseConnection,
        param: &PageQueryParam,
    ) -> Result<(Vec<user_entity::Model>, u64), DbErr> {
        self.generic_dao.find_page(db, param).await
    }

    async fn create(
        &self,
        db: &DatabaseConnection,
        model: user_entity::Model,
    ) -> Result<user_entity::Model, DbErr> {
        // Custom implementation if needed
        if model.name.is_empty() {
            return Err(DbErr::Custom("Name cannot be empty".into()));
        }
        self.generic_dao.create(db, model).await
    }

    async fn update(
        &self,
        db: &DatabaseConnection,
        model: user_entity::Model,
    ) -> Result<user_entity::Model, DbErr> {
        self.generic_dao.update(db, model).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: i64) -> Result<DeleteResult, DbErr> {
        self.generic_dao.delete(db, id).await
    }
}
