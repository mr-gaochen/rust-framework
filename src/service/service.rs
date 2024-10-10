use crate::dto::request::PageQueryParam;
use crate::dto::response::ObjCount;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{sea_query::IntoCondition, DbErr, DeleteResult, EntityTrait, PrimaryKeyTrait};

// 定义 Service Trait，泛型 E 是 Entity 类型，Pk 是主键类型
#[async_trait]
pub trait Service<E, Pk>: Send + Sync
where
    E: EntityTrait + Send + Sync,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync,
{
    /// 查找某个实体
    async fn find_by_id(&self, id: Pk) -> Result<Option<E::Model>, DbErr>;

    // 条件查询某个实体
    async fn find_one_condition<F>(&self, filter: F) -> Result<Option<E::Model>, DbErr>
    where
        F: IntoCondition + Send;

    /// 条件统计
    async fn count_condition<F>(&self, filter: F) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send;

    async fn count_condition_group<F>(
        &self,
        filter: F,
        select_columns: Option<Vec<(E::Column, &str)>>,
        group_by_column: Option<E::Column>,
    ) -> Result<Vec<ObjCount>, DbErr>
    where
        F: IntoCondition + Send;

    // 集合查询全量列表
    async fn find_list(&self) -> Result<Vec<E::Model>, DbErr>;

    // 集合条件查询列表
    async fn find_by_list_condition<F>(&self, filter: F) -> Result<Vec<E::Model>, DbErr>
    where
        F: IntoCondition + Send;

    async fn find_page(&self, param: &PageQueryParam) -> Result<(Vec<E::Model>, u64), DbErr>;

    // 分页条件查询
    async fn find_page_condition<F>(
        &self,
        filter: F,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        F: IntoCondition + Send;

    // 创建新实体
    async fn create(&self, model: E::Model) -> Result<E::Model, DbErr>;

    // 更新实体
    async fn update_by_id(&self, model: E::Model) -> Result<E::Model, DbErr>;

    // 条件更新
    async fn update_by_condition<F>(
        &self,
        filter: F,
        column_updates: Vec<(E::Column, Value)>,
    ) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send;

    // 删除实体
    async fn delete(&self, id: Pk) -> Result<DeleteResult, DbErr>;

    // 条件删除
    async fn delete_by_condition<F>(&self, filter: F) -> Result<DeleteResult, DbErr>
    where
        F: IntoCondition + Send;

    async fn delete_batch<C>(&self, condition: C) -> Result<DeleteResult, DbErr>
    where
        C: IntoCondition + Send;
}
