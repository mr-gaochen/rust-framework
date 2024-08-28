use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult, EntityTrait, PrimaryKeyTrait};

use crate::repo::repo::Repo;

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

    async fn create(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr> {
        self.dao.create(db, model).await
    }

    async fn update(&self, db: &DatabaseConnection, model: E::Model) -> Result<E::Model, DbErr> {
        self.dao.update(db, model).await
    }

    async fn delete(&self, db: &DatabaseConnection, id: Pk) -> Result<DeleteResult, DbErr> {
        self.dao.delete(db, id).await
    }
}
