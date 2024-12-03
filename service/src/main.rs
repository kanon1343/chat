use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

pub mod chat {
    tonic::include_proto!("chat");
}

use chat::chat_service_server::{ChatService, ChatServiceServer};
use chat::{ChatMessage, StreamRequest, StreamResponse};

#[derive(Default)]
pub struct ChatServiceImpl {}

#[tonic::async_trait]
impl ChatService for ChatServiceImpl {
    type StreamMessagesStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn send_message(
        &self,
        request: Request<ChatMessage>,
    ) -> Result<Response<StreamResponse>, Status> {
        println!("Received message: {:?}", request.into_inner());
        Ok(Response::new(StreamResponse {
            status: "Message sent".into(),
        }))
    }

    async fn stream_messages(
        &self,
        _request: Request<StreamRequest>,
    ) -> Result<Response<Self::StreamMessagesStream>, Status> {
        let (tx, rx) = mpsc::channel(10);
        tokio::spawn(async move {
            let messages = vec![
                ChatMessage {
                    user: "Alice".into(),
                    message: "Hello!".into(),
                    timestamp: 123456789,
                },
                ChatMessage {
                    user: "Bob".into(),
                    message: "Hi, Alice!".into(),
                    timestamp: 123456790,
                },
            ];
            for msg in messages {
                tx.send(Ok(msg)).await.unwrap();
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let chat_service = ChatServiceImpl::default();

    println!("ChatService listening on {}", addr);

    Server::builder()
        .add_service(ChatServiceServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}
