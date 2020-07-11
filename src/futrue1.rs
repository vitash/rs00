// use futures::{Future, TryFuture};

use std::future::Future;
use std::pin::Pin;

// async fn with_log<R>(s: String, handler: impl Fn() -> R) -> Future<Output = R>
// where
//     R: Future + Send,
// {
//     async move {
//         println!("{}", s);
//         return handler().await;
//     };
//     // fn1
// }

fn block1() {
    let fut = async { Ok::<(), u32>(()) };
    // fut.poll()
}



// fn fn1() -> impl std::future::Future<Output = ()> {
//     let fut = async move |a: u8| {
//         println!("{}", a);
//         //  Ok::<(), u32>(())
//     };

//     fut
// }

use futures::future::{BoxFuture, FutureExt};

fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }
    .boxed()
}

pub async fn some_async_func(arg: &str) -> u8 {
    3
}

// pub fn some_async_func_wrapper<'a>(arg: &'a str)
//     -> Pin<Box<dyn Future<Output=()> + 'a>>
// {
//     Box::pin(some_async_func(arg))
// }

pub fn higher_order_func<'a>(action: &str) -> fn(&'a str) -> Pin<Box<dyn Future<Output = u8> + 'a>> {
    |arg| Box::pin(some_async_func(arg))
}
