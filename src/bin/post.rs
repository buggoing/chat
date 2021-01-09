use tonic::{transport::Server, Request, Response, Status};

use chat::proto::post::{
    post_server::{Post, PostServer},
    CreatePostReply, CreatePostRequest, GetPostReply, GetPostRequest,
};

#[derive(Debug, Default)]
pub struct MPost;

#[tonic::async_trait]
impl Post for MPost {
    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetPostReply { posts: Vec::new() };

        Ok(Response::new(reply))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = CreatePostReply {};
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mpost = MPost::default();
    Server::builder()
        .add_service(PostServer::new(mpost))
        .serve(addr)
        .await?;
    println!("server listening in {}", addr);
    Ok(())
}
