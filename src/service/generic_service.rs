use crate::{dto::request::PageQueryParam, repo::repo::Repo};
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{sea_query::IntoCondition, DbErr, DeleteResult, EntityTrait, PrimaryKeyTrait};

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
    async fn find_by_id(&self, id: Pk) -> Result<Option<E::Model>, DbErr> {
        self.dao.find_by_id(id).await
    }

    async fn find_one_condition<F>(&self, filter: F) -> Result<Option<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.find_one_condition(filter).await
    }

    // 集合查询全量列表
    async fn find_list(&self) -> Result<Vec<E::Model>, DbErr> {
        self.dao.find_list().await
    }

    // 集合条件查询列表
    async fn find_by_list_condition<F>(&self, filter: F) -> Result<Vec<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.find_by_list_condition(filter).await
    }

    async fn find_page(&self, param: &PageQueryParam) -> Result<(Vec<E::Model>, u64), DbErr> {
        self.dao.find_page(param).await
    }

    // 分页条件查询
    async fn find_page_condition<F>(
        &self,
        filter: F,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.find_page_condition(filter, param).await
    }

    async fn create(&self, model: E::Model) -> Result<E::Model, DbErr> {
        self.dao.create(model).await
    }

    async fn update(&self, model: E::Model) -> Result<E::Model, DbErr> {
        self.dao.update(model).await
    }

    async fn update_by_condition<F>(
        &self,
        filter: F,
        column_updates: Vec<(E::Column, Value)>,
    ) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send,
    {
        self.dao.update_by_condition(filter, column_updates).await
    }

    async fn delete(&self, id: Pk) -> Result<DeleteResult, DbErr> {
        self.dao.delete(id).await
    }

    async fn delete_batch<C>(&self, condition: C) -> Result<DeleteResult, DbErr>
    where
        C: IntoCondition + Send,
    {
        self.dao.delete_batch(condition).await
    }
}
