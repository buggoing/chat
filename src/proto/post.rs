#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPostRequest {
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPostReply {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::std::vec::Vec<get_post_reply::Post>,
}
pub mod get_post_reply {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Post {
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        #[prost(string, tag = "2")]
        pub content: std::string::String,
        #[prost(int64, tag = "3")]
        pub uid: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePostRequest {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub content: std::string::String,
    #[prost(string, tag = "3")]
    pub tag: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePostReply {}
#[doc = r" Generated server implementations."]
pub mod post_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PostServer."]
    #[async_trait]
    pub trait Post: Send + Sync + 'static {
        async fn get_post(
            &self,
            request: tonic::Request<super::GetPostRequest>,
        ) -> Result<tonic::Response<super::GetPostReply>, tonic::Status>;
        async fn create_post(
            &self,
            request: tonic::Request<super::CreatePostRequest>,
        ) -> Result<tonic::Response<super::CreatePostReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PostServer<T: Post> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Post> PostServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for PostServer<T>
    where
        T: Post,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/post.Post/get_post" => {
                    #[allow(non_camel_case_types)]
                    struct get_postSvc<T: Post>(pub Arc<T>);
                    impl<T: Post> tonic::server::UnaryService<super::GetPostRequest> for get_postSvc<T> {
                        type Response = super::GetPostReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = get_postSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/post.Post/create_post" => {
                    #[allow(non_camel_case_types)]
                    struct create_postSvc<T: Post>(pub Arc<T>);
                    impl<T: Post> tonic::server::UnaryService<super::CreatePostRequest> for create_postSvc<T> {
                        type Response = super::CreatePostReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePostRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_post(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = create_postSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Post> Clone for PostServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Post> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Post> tonic::transport::NamedService for PostServer<T> {
        const NAME: &'static str = "post.Post";
    }
}
