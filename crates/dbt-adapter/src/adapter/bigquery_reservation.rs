use crate::adapter::adapter_impl::AdapterImpl;
use crate::errors::AdapterError;
use crate::query_ctx::QueryContext;
use crate::relation::Relation;
use serde::Deserialize;

/// Configuration for BigQuery reservation assignment.
#[derive(Debug, Deserialize, Clone)]
pub struct BigQueryReservationConfig {
    pub reservation: Option<String>,
}

impl AdapterImpl {
    /// Apply reservation configuration to the BigQuery job.
    pub fn apply_reservation(
        &self,
        query_ctx: &mut QueryContext,
        relation: &Relation,
    ) -> Result<(), AdapterError> {
        if let Some(reservation) = &relation.config.reservation {
            query_ctx.set_reservation(reservation.clone());
        }
        Ok(())
    }
}