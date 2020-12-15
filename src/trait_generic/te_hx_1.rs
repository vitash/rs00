use async_trait::async_trait;
trait PostType: Send {
    type JsonData: Send;
    type ResponseData: Send;
}

trait Post: PostType {
    const URL: &'static str;
}
#[async_trait]
trait WSend<T: PostType> {
    async fn send(data: T::JsonData) -> Result<T::ResponseData, u8>;
}

#[async_trait]
impl<T: Post + 'static> WSend<T> for T {
    async fn send(data: T::JsonData) -> Result<T::ResponseData, u8> {
        Err(3)
        // todo!()
    }
}

#[allow(non_camel_case_types)]
struct get_aa;

impl PostType for get_aa {
    type JsonData = Vec<u8>;
    type ResponseData = Vec<char>;
}
impl Post for get_aa {
    const URL: &'static str = "sdd";
}

// #[test]
async fn test_post() {
    let a = get_aa::send(vec![1, 2]).await;
    println!("{:?}", &a);
}
