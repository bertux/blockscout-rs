#![allow(unused_variables)]
use super::utils::insert_counter_double_data;
use crate::{counters_list, UpdateError};
use async_trait::async_trait;
use entity::sea_orm_active_enums::{ChartType, ChartValueType};
use sea_orm::prelude::*;

#[derive(Default, Debug)]
pub struct TotalTokens {}

#[async_trait]
impl crate::Chart for TotalTokens {
    fn name(&self) -> &str {
        counters_list::TOTAL_TOKENS
    }

    async fn create(&self, db: &DatabaseConnection) -> Result<(), DbErr> {
        crate::chart::create_chart(
            db,
            self.name().into(),
            ChartType::Counter,
            ChartValueType::Double,
        )
        .await
    }

    async fn update(
        &self,
        db: &DatabaseConnection,
        blockscout: &DatabaseConnection,
    ) -> Result<(), UpdateError> {
        let chart_id = crate::chart::find_chart(db, self.name())
            .await?
            .ok_or_else(|| UpdateError::NotFound(self.name().into()))?;
        // TODO: remove mock
        insert_counter_double_data(
            db,
            chart_id,
            chrono::offset::Local::now().date_naive(),
            84392233.47282,
        )
        .await?;
        Ok(())
    }
}
