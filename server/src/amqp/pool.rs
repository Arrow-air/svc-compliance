//! AMQP connection pool implementation

use super::AMQPError;
use deadpool_lapin::{Object, Pool, Runtime};

/// Represents a pool of connections to a amqp server
///
/// The [`AMQPPool`] struct provides a managed pool of connections to a amqp/rabbitmq server.
/// It allows clients to acquire and release connections from the pool and handles
/// connection management, such as connection pooling and reusing connections.
#[derive(Clone, Debug)]
pub struct AMQPPool {
    /// The underlying pool of AMQP connections.
    pool: Pool,
}

impl AMQPPool {
    /// Create a new AMQP pool
    pub fn new(config: crate::config::Config) -> Result<Self, AMQPError> {
        let cfg: deadpool_lapin::Config = config.amqp.clone();
        let Some(details) = cfg.url.clone() else {
            amqp_error!("(new) No connection address found.");
            amqp_debug!("(new) Available config: {:?}", &config.amqp);
            return Err(AMQPError::MissingConfiguration);
        };

        amqp_info!("(new) Creating pool at [{:?}]...", details);
        match cfg.create_pool(Some(Runtime::Tokio1)) {
            Ok(pool) => {
                amqp_info!("(new) Pool created.");
                Ok(AMQPPool { pool })
            }
            Err(e) => {
                amqp_error!("(new) Could not create pool: {}", e);
                Err(AMQPError::CouldNotConnect)
            }
        }
    }

    /// Get a connection from the pool
    #[cfg(not(tarpaulin_include))]
    //
    pub async fn get_connection(&self) -> Result<Object, AMQPError> {
        match self.pool.get().await {
            Ok(connection) => Ok(connection),
            Err(e) => {
                amqp_error!("(get_connection) Could not connect to deadpool: {}", e);
                Err(AMQPError::CouldNotConnect)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "stub_backends")]
    async fn test_amqp_pool_new_failure() {
        crate::get_log_handle().await;
        ut_info!("(test_amqp_pool_new_failure) Start.");

        let config = crate::config::Config::default();
        let result = AMQPPool::new(config.clone());
        assert!(result.is_err());

        ut_info!("(test_amqp_pool_new_failure) Success.");
    }

    #[tokio::test]
    #[cfg(not(feature = "stub_backends"))]
    async fn test_amqp_pool_new() {
        crate::get_log_handle().await;
        ut_info!("(test_amqp_pool_new) Start.");

        let config = crate::config::Config::default();
        let result = AMQPPool::new(config.clone());
        assert!(result.is_ok());

        ut_info!("(test_amqp_pool_new) Success.");
    }
}
