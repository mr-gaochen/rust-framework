use crate::dto::request::{Direction, PageQueryParam};
use async_trait::async_trait;
use sea_orm::sea_query::IntoCondition;
use sea_orm::{prelude::*, Set, TransactionTrait};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Order, PaginatorTrait,
    PrimaryKeyTrait, QueryFilter, QueryOrder,
};
use sea_orm::{DeleteResult, IntoActiveModel};
use std::sync::Arc;

use super::repo::Repo;

// 实现一个泛型的 repo
pub struct GenericRepo<E, Pk>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
{
    _entity: std::marker::PhantomData<E>,
    _pk: std::marker::PhantomData<Pk>,
    db: Arc<DatabaseConnection>,
}

impl<E, Pk> GenericRepo<E, Pk>
where
    E: EntityTrait,
    Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
{
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            _entity: std::marker::PhantomData,
            _pk: std::marker::PhantomData,
            db,
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
    async fn find_by_id(&self, id: Pk) -> Result<Option<E::Model>, DbErr> {
        let id_value = id.into();
        E::find_by_id(id_value).one(self.db.as_ref()).await
    }

    async fn find_one_condition<F>(&self, filter: F) -> Result<Option<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        E::find().filter(filter).one(self.db.as_ref()).await
    }

    async fn count_condition<F>(&self, filter: F) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send,
    {
        E::find().filter(filter).count(self.db.as_ref()).await
    }

    async fn find_list(&self) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(self.db.as_ref()).await
    }

    async fn find_by_list_condition<F>(&self, filter: F) -> Result<Vec<E::Model>, DbErr>
    where
        F: IntoCondition + Send,
    {
        E::find()
            .filter(filter.into_condition())
            .all(self.db.as_ref())
            .await
    }

    async fn find_page(&self, param: &PageQueryParam) -> Result<(Vec<E::Model>, u64), DbErr> {
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
        let paginator = select.paginate(self.db.as_ref(), param.page_size);
        let items_total = paginator.num_items().await.unwrap();
        let models = paginator.fetch_page(param.page_num).await?;
        Ok((models, items_total))
    }

    async fn find_page_condition<F>(
        &self,
        filter: F,
        param: &PageQueryParam,
    ) -> Result<(Vec<E::Model>, u64), DbErr>
    where
        F: IntoCondition + Send,
    {
        let paginator = E::find()
            .filter(filter)
            .paginate(self.db.as_ref(), param.page_size);
        let items_total = paginator.num_items().await.unwrap();
        let models = paginator.fetch_page(param.page_num).await?;
        Ok((models, items_total))
    }

    async fn create(&self, model: E::Model) -> Result<E::Model, DbErr> {
        // 将 E::Model 转换为 ActiveModel
        let active_model = E::ActiveModel::from(model);
        // 启动事务
        let txn = self.db.begin().await?;
        // 插入记录
        let inserted_model = match active_model.insert(&txn).await {
            Ok(model) => model,
            Err(e) => {
                // 出现错误时回滚事务
                txn.rollback().await?;
                return Err(e);
            }
        };
        // 提交事务
        txn.commit().await?;
        // 返回插入的模型
        Ok(inserted_model)
    }

    async fn update_by_id(&self, updated_model: E::Model) -> Result<E::Model, DbErr> {
        // 启动事务
        let txn = self.db.begin().await?;
        // 将更新后的模型转换为 ActiveModel
        let mut updated_active_model: E::ActiveModel = updated_model.into_active_model();
        updated_active_model.reset_all();
        // 尝试更新模型
        match updated_active_model.update(&txn).await {
            Ok(model) => {
                // 提交事务
                txn.commit().await?;
                // 返回更新后的模型
                Ok(model)
            }
            Err(e) => {
                // 如果更新失败，则回滚事务
                txn.rollback().await?;
                Err(e)
            }
        }
    }

    async fn update_by_condition<F>(
        &self,
        filter: F,
        column_updates: Vec<(E::Column, Value)>,
    ) -> Result<u64, DbErr>
    where
        F: IntoCondition + Send,
        E: EntityTrait,
    {
        // 开启事务
        let txn = self.db.begin().await?;

        let mut update_query = E::update_many().filter(filter.into_condition());

        for (column, value) in column_updates {
            update_query = update_query.col_expr(column, Expr::value(value));
        }
        // 执行更新操作
        let result = match update_query.exec(&txn).await {
            Ok(res) => res,
            Err(e) => {
                // 更新失败，回滚事务
                txn.rollback().await?;
                return Err(e);
            }
        };
        // 提交事务
        txn.commit().await?;
        Ok(result.rows_affected)
    }

    async fn delete(&self, id: Pk) -> Result<DeleteResult, DbErr> {
        // 开启事务
        let txn = self.db.begin().await?;
        let id_value = id.into();
        // 执行删除操作
        let result = match E::delete_by_id(id_value).exec(&txn).await {
            Ok(res) => res,
            Err(e) => {
                // 删除失败，回滚事务
                txn.rollback().await?;
                return Err(e);
            }
        };
        // 提交事务
        txn.commit().await?;
        Ok(result)
    }

    async fn delete_batch<C>(&self, condition: C) -> Result<DeleteResult, DbErr>
    where
        C: IntoCondition + Send,
    {
        // 开启事务
        let txn = self.db.begin().await?;
        // 执行批量删除操作
        let result = match E::delete_many().filter(condition).exec(&txn).await {
            Ok(res) => res,
            Err(e) => {
                // 删除失败，回滚事务
                txn.rollback().await?;
                return Err(e);
            }
        };
        // 提交事务
        txn.commit().await?;
        Ok(result)
    }
}
