use async_trait::async_trait;
use sea_orm::{
    sea_query::IntoCondition, DatabaseConnection, DbErr, DeleteResult, EntityTrait, PrimaryKeyTrait,
};

use crate::dto::request::PageQueryParam;

// 定义 Service Trait，泛型 E 是 Entity 类型，Pk 是主键类型
#[async_trait]
pub trait Service<E, Pk>: Send + Sync
where
    E: EntityTrait + Send + Sync,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync,
{
    /// 查找某个实体
    async fn find_by_id(&self, db: &DatabaseConnection, id: Pk) -> Result<Option<E::Model>, DbErr>;

    // 条件查询某个实体
    async fn find_one_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
    ) -> Result<Option<E::Model>, DbErr>
    where
        F: IntoCondition + Send;

    // 集合查询全量列表
    async fn find_list(&self, db: &DatabaseConnection) -> Result<Vec<E::Model>, DbErr>;

    // 集合条件查询列表
    async fn find_by_list_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
    ) -> Result<Vec<E::Model>, DbErr>
    where
        F: IntoCondition + Send;

    async fn find_page(
        &self,
        db: &DatabaseConnection,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr>;

    // 分页条件查询
    async fn find_page_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        F: IntoCondition + Send;

    // 创建新实体
    async fn create(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr>;

    // 更新实体
    async fn update(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr>;

    // 条件更新
    async fn update_by_condition<F>(
        &self,
        db: &DatabaseConnection,
        filter: F,
        model: E::Model,
    ) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send;

    // 删除实体
    async fn delete(&self, db: &DatabaseConnection, id: Pk) -> Result<DeleteResult, DbErr>;

    async fn delete_batch<C>(
        &self,
        db: &DatabaseConnection,
        condition: C,
    ) -> Result<DeleteResult, DbErr>
    where
        C: IntoCondition + Send;
}
