use async_trait::async_trait;
use sea_orm::{
    sea_query::IntoCondition, DatabaseConnection, DbErr, DeleteResult, EntityTrait, PrimaryKeyTrait,
};

use crate::{dto::request::PageQueryParam, repo::repo::Repo};

use super::service::Service;

pub struct GenericService<E, Pk, D>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    D: Repo<E, Pk>,
{
    dao: D,
    _entity: std::marker::PhantomData<E>,
    _pk: std::marker::PhantomData<Pk>,
}

impl<E, Pk, D> GenericService<E, Pk, D>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    D: Repo<E, Pk>,
{
    pub fn new(dao: D) -> Self {
        Self {
            dao,
            _entity: std::marker::PhantomData,
            _pk: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<E, Pk, D> Service<E, Pk> for GenericService<E, Pk, D>
where
    E: EntityTrait + Send + Sync,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    D: Repo<E, Pk> + Send + Sync,
{
    async fn find_by_id(&self, db: &DatabaseConnection, id: Pk) -> Result<Option<E::Model>, DbErr> {
        self.dao.find_by_id(db, id).await
    }

    async fn find_one_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
    ) -> Result<Option<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.find_one_condition(db, filter).await
    }

    // 集合查询全量列表
    async fn find_list(&self, db: &DatabaseConnection) -> Result<Vec<E::Model>, DbErr> {
        self.dao.find_list(db).await
    }

    // 集合条件查询列表
    async fn find_by_list_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
    ) -> Result<Vec<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.find_by_list_condition(db, filter).await
    }

    async fn find_page(
        &self,
        db: &DatabaseConnection,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr> {
        self.dao.find_page(db, param).await
    }

    // 分页条件查询
    async fn find_page_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.find_page_condition(db, filter, param).await
    }

    async fn create(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr> {
        self.dao.create(db, model).await
    }

    async fn update(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr> {
        self.dao.update(db, model).await
    }

    async fn update_by_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
        model: E::Model,
    ) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.update_by_condition(db, filter, model).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Pk) -> Result<DeleteResult, DbErr> {
        self.dao.delete(db, id).await
    }

    async fn delete_batch<C>(
        &self,
        db: &DatabaseConnection,
        condition: C,
    ) -> Result<DeleteResult, DbErr>
    where
        C: IntoCondition + Send,
    {
        self.dao.delete_batch(db, condition).await
    }
}
