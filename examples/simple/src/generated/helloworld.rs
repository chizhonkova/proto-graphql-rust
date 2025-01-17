#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub number: ::core::option::Option<super::num::Int32W>,
    #[prost(enumeration = "Language", repeated, tag = "3")]
    pub languages: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag = "4")]
    pub strings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct HelloReply {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, :: prost :: Enumeration)]
#[repr(i32)]
pub enum Language {
    En = 0,
    Ru = 1,
}
/// Generated client implementations.
pub mod greeter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct GreeterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GreeterClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GreeterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GreeterClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            GreeterClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn say_hello(
            &mut self,
            request: impl tonic::IntoRequest<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/helloworld.Greeter/SayHello");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated gateway implementations.
pub mod greeter_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type GreeterSchema<T> = ::async_graphql::Schema<
        GreeterQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        GreeterQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        ::async_graphql::Schema::build(
            <GreeterQuery<T>>::default(),
            ::async_graphql::EmptyMutation,
            ::async_graphql::EmptySubscription,
        )
    }
    pub struct GreeterQuery<T> {
        _grpc_client: ::std::marker::PhantomData<super::greeter_client::GreeterClient<T>>,
    }
    #[::async_graphql::Object(name = "GreeterQuery")]
    impl<T> GreeterQuery<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub async fn say_hello(
            &self,
            ctx: &::async_graphql::Context<'_>,
            request: super::HelloRequestGraphQlInput,
        ) -> ::async_graphql::Result<super::HelloReplyGraphQl> {
            let mut grpc_client = ctx
                .data::<super::greeter_client::GreeterClient<T>>()?
                .clone();
            let response = grpc_client
                .say_hello(<super::HelloRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::HelloReplyGraphQl>::from(response.into_inner());
            Ok(response)
        }
    }
    impl<T> Default for GreeterQuery<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for GreeterQuery<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client,
            }
        }
    }
    impl<T> ::std::fmt::Debug for GreeterQuery<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "GreeterQuery {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod greeter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with GreeterServer.
    #[async_trait]
    pub trait Greeter: Send + Sync + 'static {
        async fn say_hello(
            &self,
            request: tonic::Request<super::HelloRequest>,
        ) -> Result<tonic::Response<super::HelloReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GreeterServer<T: Greeter> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Greeter> GreeterServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GreeterServer<T>
    where
        T: Greeter,
        B: Body + Send + Sync + 'static,
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
                "/helloworld.Greeter/SayHello" => {
                    #[allow(non_camel_case_types)]
                    struct SayHelloSvc<T: Greeter>(pub Arc<T>);
                    impl<T: Greeter> tonic::server::UnaryService<super::HelloRequest> for SayHelloSvc<T> {
                        type Response = super::HelloReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HelloRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).say_hello(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SayHelloSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Greeter> Clone for GreeterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Greeter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Greeter> tonic::transport::NamedService for GreeterServer<T> {
        const NAME: &'static str = "helloworld.Greeter";
    }
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "HelloRequest")]
pub struct HelloRequestGraphQl {
    pub name: ::prost::alloc::string::String,
    pub number: ::core::option::Option<super::num::i32>,
    pub languages: ::prost::alloc::vec::Vec<LanguageGraphQl>,
    pub strings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    Default,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "HelloRequestInput")]
pub struct HelloRequestGraphQlInput {
    pub name: ::prost::alloc::string::String,
    pub number: ::core::option::Option<super::num::i32>,
    pub languages: ::prost::alloc::vec::Vec<::core::option::Option<LanguageGraphQlInput>>,
    pub strings: ::prost::alloc::vec::Vec<::core::option::Option<::prost::alloc::string::String>>,
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequest> for HelloRequestGraphQl {
    fn from(other: HelloRequest) -> Self {
        let languages = other.languages().map(Into::into).collect();
        let HelloRequest {
            name,
            number,
            strings,
            ..
        } = other;
        Self {
            name: name.into(),
            number: number.map(Into::into),
            languages,
            strings: strings.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequestGraphQl> for HelloRequest {
    fn from(other: HelloRequestGraphQl) -> Self {
        let HelloRequestGraphQl {
            name,
            number,
            languages,
            strings,
        } = other;
        Self {
            name: name.into(),
            number: number.map(Into::into),
            languages: languages.into_iter().map(|b| b as i32).collect(),
            strings: strings.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequest> for HelloRequestGraphQlInput {
    fn from(other: HelloRequest) -> Self {
        let languages = other.languages().map(Into::into).map(|b| Some(b)).collect();
        let HelloRequest {
            name,
            number,
            strings,
            ..
        } = other;
        Self {
            name: name.into(),
            number: number.map(Into::into),
            languages,
            strings: strings
                .into_iter()
                .map(Into::into)
                .map(|b| Some(b))
                .collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloRequestGraphQlInput> for HelloRequest {
    fn from(other: HelloRequestGraphQlInput) -> Self {
        let HelloRequestGraphQlInput {
            name,
            number,
            languages,
            strings,
        } = other;
        Self {
            name: name.into(),
            number: number.map(Into::into),
            languages: languages
                .into_iter()
                .map(|b| b.unwrap_or_default() as i32)
                .collect(),
            strings: strings
                .into_iter()
                .map(|b| b.unwrap_or_default())
                .map(Into::into)
                .collect(),
        }
    }
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: SimpleObject,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "HelloReply")]
pub struct HelloReplyGraphQl {
    pub message: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq,
    :: async_graphql :: InputObject,
    Default,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "HelloReplyInput")]
pub struct HelloReplyGraphQlInput {
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<HelloReply> for HelloReplyGraphQl {
    fn from(other: HelloReply) -> Self {
        let HelloReply { message, .. } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloReplyGraphQl> for HelloReply {
    fn from(other: HelloReplyGraphQl) -> Self {
        let HelloReplyGraphQl { message } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloReply> for HelloReplyGraphQlInput {
    fn from(other: HelloReply) -> Self {
        let HelloReply { message, .. } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<HelloReplyGraphQlInput> for HelloReply {
    fn from(other: HelloReplyGraphQlInput) -> Self {
        let HelloReplyGraphQlInput { message } = other;
        Self {
            message: message.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Language> for LanguageGraphQl {
    fn from(other: Language) -> Self {
        match other {
            Language::En => Self::En,
            Language::Ru => Self::Ru,
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<LanguageGraphQl> for Language {
    fn from(other: LanguageGraphQl) -> Self {
        match other {
            LanguageGraphQl::En => Self::En,
            LanguageGraphQl::Ru => Self::Ru,
        }
    }
}
impl Default for LanguageGraphQl {
    fn default() -> Self {
        LanguageGraphQl::Ru
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
#[derive(
    :: async_graphql :: Enum,
    :: proto_graphql :: serde :: Serialize,
    :: proto_graphql :: serde :: Deserialize,
)]
#[serde(crate = "::proto_graphql::serde")]
#[graphql(name = "Language")]
pub enum LanguageGraphQl {
    En = 0,
    Ru = 1,
}
pub use self::LanguageGraphQl as LanguageGraphQlInput;
