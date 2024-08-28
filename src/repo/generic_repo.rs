use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Order, PaginatorTrait,
    PrimaryKeyTrait, QueryOrder,
};
use sea_orm::{DeleteResult, IntoActiveModel};

use crate::dto::request::{Direction, PageQueryParam};

use super::repo::Repo;

// 实现一个泛型的 repo
pub struct GenericRepo<E, Pk>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
{
    _entity: std::marker::PhantomData<E>,
    _pk: std::marker::PhantomData<Pk>,
}

impl<E, Pk> GenericRepo<E, Pk>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
{
    pub fn new() -> Self {
        Self {
            _entity: std::marker::PhantomData,
            _pk: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<E, Pk> Repo<E, Pk> for GenericRepo<E, Pk>
where
    E: EntityTrait + Send + Sync,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    E::Model: Send + Sync + IntoActiveModel<E::ActiveModel>,
    E::ActiveModel: ActiveModelTrait<Entity = E> + Send + Sync + From<E::Model>,
{
    async fn find_by_id(&self, db: &DatabaseConnection, id: Pk) -> Result<Option<E::Model>, DbErr> {
        let id_value = id.into();
        E::find_by_id(id_value).one(db).await
    }

    async fn find_list(&self, db: &DatabaseConnection) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(db).await
    }

    async fn find_page(
        &self,
        db: &DatabaseConnection,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr> {
        let mut select = E::find();
        if let Some(sort_by) = &param.sort_by {
            let order_expr = sea_orm::sea_query::Expr::expr(
                sea_orm::sea_query::SimpleExpr::Custom(format!("{}", sort_by)),
            );
            match param.sort_direction.unwrap_or(Direction::ASC) {
                Direction::DESC => select = select.order_by(order_expr, Order::Desc),
                _ => select = select.order_by(order_expr, Order::Asc),
            }
        }
        let paginator = select.paginate(db, param.page_size);
        let items_total = paginator.num_items().await.unwrap();
        let models = paginator.fetch_page(param.page_num).await?;
        Ok((models, items_total))
    }

    async fn create(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr> {
        // 将 E::Model 转换为 ActiveModel
        let active_model = E::ActiveModel::from(model);
        active_model.insert(db).await
    }

    async fn update(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr> {
        let active_model: E::ActiveModel = model.into_active_model();
        active_model.update(db).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Pk) -> Result<DeleteResult, DbErr> {
        let id_value = id.into();
        E::delete_by_id(id_value).exec(db).await
    }
}
