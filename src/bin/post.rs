use chat::db::mongo;
use chat::proto::post::{
    get_post_reply,
    post_server::{Post, PostServer},
    CreatePostReply, CreatePostRequest, GetPostReply, GetPostRequest,
};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use tonic::{transport::Server, Code, Request, Response, Status};

#[derive(Debug)]
pub struct MPost {
    mongo: mongo::DB,
}

impl MPost {
    fn new(mongo: mongo::DB) -> Self {
        MPost { mongo: mongo }
    }
}

#[tonic::async_trait]
impl Post for MPost {
    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostReply>, Status> {
        let oid = ObjectId::with_string(&request.into_inner().id).unwrap();
        match self.mongo.get_post(doc!("_id": oid)).await {
            Ok(post) => {
                let mut posts = Vec::new();
                let p = get_post_reply::Post {
                    id: post.id.unwrap().to_hex(),
                    uid: post.uid,
                    content: post.content,
                };
                posts.push(p);
                let reply = GetPostReply { posts: posts };
                return Ok(Response::new(reply));
            }
            Err(e) => return Err(Status::new(Code::Internal, e.to_string())),
        }
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostReply>, Status> {
        let req = request.into_inner();
        let post = mongo::Post {
            title: req.title,
            content: req.content,
            tag: req.tag,
            uid: 1,
            id: None,
        };
        match self.mongo.create_post(&post).await {
            Ok(_) => return Ok(Response::new(CreatePostReply {})),
            Err(e) => return Err(Status::new(Code::Internal, e.to_string())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mongo_uri = "mongodb://chat:chat@localhost:27017";
    let mongo_db_name = "chat";
    let mongo_client = mongo::DB::new(mongo_uri, mongo_db_name).await;

    let mpost = MPost::new(mongo_client);
    Server::builder()
        .add_service(PostServer::new(mpost))
        .serve(addr)
        .await?;
    println!("server listening in {}", addr);
    Ok(())
}
