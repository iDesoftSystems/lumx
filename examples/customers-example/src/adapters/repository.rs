mod schemas;

use crate::domain::port::dto::AddCustomerChange;
use crate::domain::port::spi::CustomerSaver;
use lumx_axum::result::api::ApiError;
use lumx_core::async_trait;
use lumx_sea_orm::sea_orm::{ActiveValue, ConnectionTrait};
use sea_orm::ActiveModelTrait;
use std::sync::Arc;

pub struct CustomerSeaRepository {
    conn: Arc<dyn ConnectionTrait + Send + Sync>,
}

impl CustomerSeaRepository {
    pub fn new(conn: Arc<dyn ConnectionTrait + Send + Sync + 'static>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CustomerSaver for CustomerSeaRepository {
    async fn save_customer(&self, change: &AddCustomerChange) -> Result<i64, ApiError> {
        let active_model = schemas::customer::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::NotSet,
            business_name: ActiveValue::NotSet,
            fiscal_document_id: ActiveValue::NotSet,
            fiscal_number: ActiveValue::NotSet,
            is_natural_person: ActiveValue::NotSet,
        }
        .save(self.conn.as_ref())
        .await?;

        match active_model.id {
            ActiveValue::Set(id) => id,
            _ => Err(ApiError::Internal())
        }

        todo!()
    }
}
