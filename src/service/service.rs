use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult, EntityTrait, PrimaryKeyTrait};

// 定义 Service Trait，泛型 E 是 Entity 类型，Pk 是主键类型
#[async_trait]
pub trait Service<E, Pk>: Send + Sync
where
    E: EntityTrait + Send + Sync,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync,
{
    // 查找某个实体
    async fn find_by_id(&self, db: &DatabaseConnection, id: Pk) -> Result<Option<E::Model>, DbErr>;

    // 创建新实体
    async fn create(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr>;

    // 更新实体
    async fn update(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr>;

    // 删除实体
    async fn delete(&self, db: &DatabaseConnection, id: Pk) -> Result<DeleteResult, DbErr>;
}
