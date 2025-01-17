#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Product {
    #[prost(string, tag = "1")]
    pub upc: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub price: i32,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ProductRequest {
    #[prost(string, tag = "1")]
    pub upc: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ProductsResponse {
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
}
#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct ProductResponse {
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
}
/// Generated client implementations.
pub mod products_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// graphql: extends
    #[derive(Debug, Clone)]
    pub struct ProductsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProductsClient<tonic::transport::Channel> {
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
    impl<T> ProductsClient<T>
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
        ) -> ProductsClient<InterceptedService<T, F>>
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
            ProductsClient::new(InterceptedService::new(inner, interceptor))
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
        /// graphql: output(repeated Product products)
        pub async fn top_products(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::ProductsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/federation.products.Products/TopProducts");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// graphql: entity, inputs(string upc), output(optional Product product)
        pub async fn find_product_by_upc(
            &mut self,
            request: impl tonic::IntoRequest<super::ProductRequest>,
        ) -> Result<tonic::Response<super::ProductResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/federation.products.Products/FindProductByUpc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated gateway implementations.
pub mod products_graphql {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub type ProductsSchema<T> = ::async_graphql::Schema<
        ProductsQuery<T>,
        ::async_graphql::EmptyMutation,
        ::async_graphql::EmptySubscription,
    >;
    /// Create a GraphQL schema builder.
    pub fn build_graphql_schema<T>() -> ::async_graphql::SchemaBuilder<
        ProductsQuery<T>,
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
            <ProductsQuery<T>>::default(),
            ::async_graphql::EmptyMutation,
            ::async_graphql::EmptySubscription,
        )
    }
    pub struct ProductsQuery<T> {
        _grpc_client: ::std::marker::PhantomData<super::products_client::ProductsClient<T>>,
    }
    #[::async_graphql::Object(name = "Query", extends)]
    impl<T> ProductsQuery<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody> + Send + Sync + Clone + 'static,
        T::Future: Send,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        /// graphql: output(repeated Product products)
        pub async fn top_products(
            &self,
            ctx: &::async_graphql::Context<'_>,
        ) -> ::async_graphql::Result<::prost::alloc::vec::Vec<super::ProductGraphQl>> {
            let mut grpc_client = ctx
                .data::<super::products_client::ProductsClient<T>>()?
                .clone();
            let response = grpc_client
                .top_products(())
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ProductsResponseGraphQl>::from(response.into_inner());
            Ok(response.products)
        }
        /// graphql: entity, inputs(string upc), output(optional Product product)
        #[graphql(entity)]
        pub async fn find_product_by_upc(
            &self,
            ctx: &::async_graphql::Context<'_>,
            upc: ::prost::alloc::string::String,
        ) -> ::async_graphql::Result<::core::option::Option<super::ProductGraphQl>> {
            let request = super::ProductRequestGraphQlInput { upc };
            let mut grpc_client = ctx
                .data::<super::products_client::ProductsClient<T>>()?
                .clone();
            let response = grpc_client
                .find_product_by_upc(<super::ProductRequest>::from(request))
                .await
                .map_err(|e| ::async_graphql::Error::new(e.to_string()))?;
            let response = <super::ProductResponseGraphQl>::from(response.into_inner());
            Ok(response.product)
        }
    }
    impl<T> Default for ProductsQuery<T> {
        fn default() -> Self {
            Self {
                _grpc_client: ::std::marker::PhantomData,
            }
        }
    }
    impl<T> Clone for ProductsQuery<T> {
        fn clone(&self) -> Self {
            Self {
                _grpc_client: self._grpc_client,
            }
        }
    }
    impl<T> ::std::fmt::Debug for ProductsQuery<T> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "ProductsQuery {{ ... }}")
        }
    }
}
/// Generated server implementations.
pub mod products_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ProductsServer.
    #[async_trait]
    pub trait Products: Send + Sync + 'static {
        /// graphql: output(repeated Product products)
        async fn top_products(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::ProductsResponse>, tonic::Status>;
        /// graphql: entity, inputs(string upc), output(optional Product product)
        async fn find_product_by_upc(
            &self,
            request: tonic::Request<super::ProductRequest>,
        ) -> Result<tonic::Response<super::ProductResponse>, tonic::Status>;
    }
    /// graphql: extends
    #[derive(Debug)]
    pub struct ProductsServer<T: Products> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Products> ProductsServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProductsServer<T>
    where
        T: Products,
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
                "/federation.products.Products/TopProducts" => {
                    #[allow(non_camel_case_types)]
                    struct TopProductsSvc<T: Products>(pub Arc<T>);
                    impl<T: Products> tonic::server::UnaryService<()> for TopProductsSvc<T> {
                        type Response = super::ProductsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).top_products(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TopProductsSvc(inner);
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
                "/federation.products.Products/FindProductByUpc" => {
                    #[allow(non_camel_case_types)]
                    struct FindProductByUpcSvc<T: Products>(pub Arc<T>);
                    impl<T: Products> tonic::server::UnaryService<super::ProductRequest> for FindProductByUpcSvc<T> {
                        type Response = super::ProductResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_product_by_upc(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FindProductByUpcSvc(inner);
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
    impl<T: Products> Clone for ProductsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Products> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Products> tonic::transport::NamedService for ProductsServer<T> {
        const NAME: &'static str = "federation.products.Products";
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
#[graphql(name = "Product")]
pub struct ProductGraphQl {
    pub upc: ::prost::alloc::string::String,
    pub name: ::prost::alloc::string::String,
    pub price: i32,
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
#[graphql(name = "ProductInput")]
pub struct ProductGraphQlInput {
    pub upc: ::prost::alloc::string::String,
    pub name: ::prost::alloc::string::String,
    pub price: i32,
}
#[allow(clippy::useless_conversion)]
impl From<Product> for ProductGraphQl {
    fn from(other: Product) -> Self {
        let Product {
            upc, name, price, ..
        } = other;
        Self {
            upc: upc.into(),
            name: name.into(),
            price: price.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductGraphQl> for Product {
    fn from(other: ProductGraphQl) -> Self {
        let ProductGraphQl { upc, name, price } = other;
        Self {
            upc: upc.into(),
            name: name.into(),
            price: price.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Product> for ProductGraphQlInput {
    fn from(other: Product) -> Self {
        let Product {
            upc, name, price, ..
        } = other;
        Self {
            upc: upc.into(),
            name: name.into(),
            price: price.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductGraphQlInput> for Product {
    fn from(other: ProductGraphQlInput) -> Self {
        let ProductGraphQlInput { upc, name, price } = other;
        Self {
            upc: upc.into(),
            name: name.into(),
            price: price.into(),
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
#[graphql(name = "ProductRequest")]
pub struct ProductRequestGraphQl {
    pub upc: ::prost::alloc::string::String,
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
#[graphql(name = "ProductRequestInput")]
pub struct ProductRequestGraphQlInput {
    pub upc: ::prost::alloc::string::String,
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequest> for ProductRequestGraphQl {
    fn from(other: ProductRequest) -> Self {
        let ProductRequest { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequestGraphQl> for ProductRequest {
    fn from(other: ProductRequestGraphQl) -> Self {
        let ProductRequestGraphQl { upc } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequest> for ProductRequestGraphQlInput {
    fn from(other: ProductRequest) -> Self {
        let ProductRequest { upc, .. } = other;
        Self { upc: upc.into() }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductRequestGraphQlInput> for ProductRequest {
    fn from(other: ProductRequestGraphQlInput) -> Self {
        let ProductRequestGraphQlInput { upc } = other;
        Self { upc: upc.into() }
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
#[graphql(name = "ProductsResponse")]
pub struct ProductsResponseGraphQl {
    pub products: ::prost::alloc::vec::Vec<ProductGraphQl>,
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
#[graphql(name = "ProductsResponseInput")]
pub struct ProductsResponseGraphQlInput {
    pub products: ::prost::alloc::vec::Vec<ProductGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ProductsResponse> for ProductsResponseGraphQl {
    fn from(other: ProductsResponse) -> Self {
        let ProductsResponse { products, .. } = other;
        Self {
            products: products.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductsResponseGraphQl> for ProductsResponse {
    fn from(other: ProductsResponseGraphQl) -> Self {
        let ProductsResponseGraphQl { products } = other;
        Self {
            products: products.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductsResponse> for ProductsResponseGraphQlInput {
    fn from(other: ProductsResponse) -> Self {
        let ProductsResponse { products, .. } = other;
        Self {
            products: products.into_iter().map(Into::into).collect(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductsResponseGraphQlInput> for ProductsResponse {
    fn from(other: ProductsResponseGraphQlInput) -> Self {
        let ProductsResponseGraphQlInput { products } = other;
        Self {
            products: products.into_iter().map(Into::into).collect(),
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
#[graphql(name = "ProductResponse")]
pub struct ProductResponseGraphQl {
    pub product: ::core::option::Option<ProductGraphQl>,
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
#[graphql(name = "ProductResponseInput")]
pub struct ProductResponseGraphQlInput {
    pub product: ::core::option::Option<ProductGraphQlInput>,
}
#[allow(clippy::useless_conversion)]
impl From<ProductResponse> for ProductResponseGraphQl {
    fn from(other: ProductResponse) -> Self {
        let ProductResponse { product, .. } = other;
        Self {
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductResponseGraphQl> for ProductResponse {
    fn from(other: ProductResponseGraphQl) -> Self {
        let ProductResponseGraphQl { product } = other;
        Self {
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductResponse> for ProductResponseGraphQlInput {
    fn from(other: ProductResponse) -> Self {
        let ProductResponse { product, .. } = other;
        Self {
            product: product.map(Into::into),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<ProductResponseGraphQlInput> for ProductResponse {
    fn from(other: ProductResponseGraphQlInput) -> Self {
        let ProductResponseGraphQlInput { product } = other;
        Self {
            product: product.map(Into::into),
        }
    }
}
