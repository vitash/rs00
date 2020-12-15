use async_trait::async_trait;
use serde::{ser};

trait Post<JsonData: Send, ResponseData: Send> {
    const URL: &'static str;
}
#[async_trait]
trait WSend<JsonData: Send, ResponseData: Send> {
    async fn send(data: JsonData) -> Result<ResponseData, u8>;
}

#[async_trait]
impl<T: Post<J, R> + 'static, J: Send + 'static, R: Send + 'static> WSend<J, R> for T {
    async fn send(data: J) -> Result<R, u8> {
        Err(3)
    }
}

trait WxPostData<JsonData: Send, ResponseData: Send, PostJsonData: ser::Serialize + Send + Sync> {
    /// 对 url 进行拼接，对 json 进行再处理
    fn get_url_jsondata(access_token: &str, args: JsonData) -> (String, PostJsonData);
}
