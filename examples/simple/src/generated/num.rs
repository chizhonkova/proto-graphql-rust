#[derive(Clone, PartialEq, :: prost :: Message)]
pub struct Int32W {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[allow(non_camel_case_types)]
pub type i32 = std::primitive::i32;
#[allow(clippy::useless_conversion)]
impl From<i32> for Int32W {
    fn from(other: i32) -> Self {
        Int32W { value: other }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Int32W> for i32 {
    fn from(other: Int32W) -> Self {
        other.value
    }
}
#[derive(Clone, PartialEq, :: async_graphql :: SimpleObject)]
#[graphql(name = "Int32W")]
pub struct Int32WGraphQl {
    pub value: i32,
}
#[derive(Clone, PartialEq, :: async_graphql :: InputObject, Default)]
#[graphql(name = "Int32WInput")]
pub struct Int32WGraphQlInput {
    pub value: i32,
}
#[allow(clippy::useless_conversion)]
impl From<Int32W> for Int32WGraphQl {
    fn from(other: Int32W) -> Self {
        let Int32W { value, .. } = other;
        Self {
            value: value.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Int32WGraphQl> for Int32W {
    fn from(other: Int32WGraphQl) -> Self {
        let Int32WGraphQl { value } = other;
        Self {
            value: value.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Int32W> for Int32WGraphQlInput {
    fn from(other: Int32W) -> Self {
        let Int32W { value, .. } = other;
        Self {
            value: value.into(),
        }
    }
}
#[allow(clippy::useless_conversion)]
impl From<Int32WGraphQlInput> for Int32W {
    fn from(other: Int32WGraphQlInput) -> Self {
        let Int32WGraphQlInput { value } = other;
        Self {
            value: value.into(),
        }
    }
}
