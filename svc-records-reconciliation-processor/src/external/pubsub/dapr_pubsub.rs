use crate::internal::{
    entities::{app_error::AppError, app_error::AppErrorKind, file_upload_chunk::FileUploadChunk},
    interfaces::pubsub_repo::PubSubRepositoryInterface,
};
use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
//use std::collections::HashMap;
use tonic::transport::Channel as TonicChannel;

pub struct DaprPubSubRepositoryManager {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr pub sub component name
    pub dapr_pubsub_name: String,

    //the dapr pub sub topic
    pub dapr_pubsub_topic: String,
}

#[async_trait]
impl PubSubRepositoryInterface for DaprPubSubRepositoryManager {
    async fn get_next_comparison_file_upload_chunk(&self) -> Result<FileUploadChunk, AppError> {
        //create a dapr client
        let mut _client = self.get_dapr_connection().await?;

        //call the binding
        todo!()
    }
}

impl DaprPubSubRepositoryManager {
    async fn get_dapr_connection(&self) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = self.dapr_grpc_server_address.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        match client_connect_result {
            //connection succeeded
            Ok(s) => return Ok(s),
            //connection failed
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}
