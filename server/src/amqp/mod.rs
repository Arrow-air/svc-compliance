//! provides AMQP/RabbitMQ implementations for queuing layer

#[macro_use]
pub mod macros;
pub mod pool;
use crate::config::Config;
use lapin::{options::BasicPublishOptions, BasicProperties};

/// Name of the AMQP exchange for flightplan messages
pub const EXCHANGE_NAME_FLIGHTPLAN: &str = "flightplan";

/// Name of the AMQP queue for CARGO messages
pub const QUEUE_NAME_CARGO: &str = "cargo";

/// Routing key for CARGO messages
pub const ROUTING_KEY_CARGO: &str = "cargo";

/// Custom Error type for MQ errors
#[derive(thiserror::Error, Debug, Copy, Clone)]
pub enum AMQPError {
    /// Could Not Publish
    #[error("error: Could not publish to queue.")]
    CouldNotPublish,

    /// Could not connect to the AMQP pool.
    #[error("error: Could not connect to amqp pool.")]
    CouldNotConnect,

    /// Missing configuration
    #[error("error: Missing configuration for amqp pool connection.")]
    MissingConfiguration,

    /// Could not create channel
    #[error("error: Could not create channel.")]
    CouldNotCreateChannel,

    /// Could not declare queue
    #[error("error: Could not declare queue.")]
    CouldNotDeclareQueue,

    /// Could not declare exchange
    #[error("error: Could not declare exchange.")]
    CouldNotDeclareExchange,
}

/// Wrapper struct to allow unit testing on un-connected amqp_channel
#[derive(Debug)]
pub struct AMQPChannel {
    /// The lapin::Channel if available
    pub channel: Option<lapin::Channel>,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "test_util")] {
        impl AMQPChannel {
            /// Wrapper function for lapin::Channel basic_publish
            pub async fn basic_publish(
                &self,
                exchange: &str,
                routing_key: &str,
                options: BasicPublishOptions,
                payload: &[u8],
                properties: BasicProperties,
            ) -> Result<(), AMQPError> {
                if let Some(channel) = &self.channel {
                    match channel
                        .basic_publish(exchange, routing_key, options, payload, properties)
                        .await
                    {
                        Ok(_) => Ok(()),
                        Err(_) => Err(AMQPError::CouldNotPublish)
                    }
                } else {
                    Ok(())
                }
            }
        }
    } else {
        use lapin::publisher_confirm::PublisherConfirm;
        impl AMQPChannel {
            /// Wrapper function for lapin::Channel basic_publish
            pub async fn basic_publish(&self, exchange: &str, routing_key: &str, options: BasicPublishOptions, payload: &[u8], properties: BasicProperties) -> lapin::Result<PublisherConfirm> {
                if let Some(channel) = &self.channel {
                    channel.basic_publish(exchange, routing_key, options, payload, properties).await
                } else {
                    amqp_error!("(basic_publish) No channel set AMQPChannel.");
                    Err(lapin::Error::InvalidChannelState(lapin::ChannelState::Error))
                }
            }
        }
    }
}

/// Initializes the AMQP connection. Creates the flightplan exchange and queues.
#[cfg(not(tarpaulin_include))]
pub async fn init_mq(config: Config) -> Result<lapin::Channel, AMQPError> {
    // Establish connection to RabbitMQ node
    let pool = pool::AMQPPool::new(config.clone())?;

    let amqp_connection = pool.get_connection().await?;

    // Create channel
    amqp_info!("(init_mq) Creating channel...");
    let amqp_channel = match amqp_connection.create_channel().await {
        Ok(channel) => channel,
        Err(e) => {
            amqp_error!("(init_mq) Could not create channel: {}", e);
            return Err(AMQPError::CouldNotCreateChannel);
        }
    };

    // Declare CARGO Queue
    {
        amqp_info!("(init_mq) Creating '{QUEUE_NAME_CARGO}' queue...");
        let result = amqp_channel
            .queue_declare(
                QUEUE_NAME_CARGO,
                lapin::options::QueueDeclareOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await;

        if let Err(e) = result {
            amqp_error!(
                "(init_mq) Could not declare queue '{QUEUE_NAME_CARGO}': {}",
                e
            );
            return Err(AMQPError::CouldNotDeclareQueue);
        }
    }

    //
    // Declare a topic exchange
    //
    {
        amqp_info!("(init_mq) Declaring exchange '{EXCHANGE_NAME_FLIGHTPLAN}'...");
        let result = amqp_channel
            .exchange_declare(
                EXCHANGE_NAME_FLIGHTPLAN,
                lapin::ExchangeKind::Topic,
                lapin::options::ExchangeDeclareOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await;

        if let Err(e) = result {
            amqp_error!(
                "(init_mq) Could not declare exchange '{EXCHANGE_NAME_FLIGHTPLAN}': {}",
                e
            );
            return Err(AMQPError::CouldNotDeclareExchange);
        }
    }

    //
    // Bind the CARGO queue to the exchange
    //
    {
        amqp_info!("(init_mq) Binding queue '{QUEUE_NAME_CARGO}' to exchange '{EXCHANGE_NAME_FLIGHTPLAN}'...");
        let result = amqp_channel
            .queue_bind(
                QUEUE_NAME_CARGO,
                EXCHANGE_NAME_FLIGHTPLAN,
                ROUTING_KEY_CARGO,
                lapin::options::QueueBindOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await;

        if let Err(e) = result {
            amqp_error!(
                "(init_mq) Could not bind queue '{QUEUE_NAME_CARGO}' to exchange: {}",
                e
            );
        }
    }

    Ok(amqp_channel)
}
