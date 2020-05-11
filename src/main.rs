use tokio;
use tokio::prelude::*;
use futures::stream::{BoxStream, StreamExt};

macro_rules! const_stream {
    ( $( $x:expr ),* ) => {
        {
            let mut stream: BoxStream<'static, _> = tokio::stream::empty().boxed();
            $(
                stream = stream.chain(tokio::stream::once($x)).boxed();
            )*
            stream
        }
    };
}

#[tokio::main]
async fn main() {
    println!("Hello world");

    let mut a = const_stream![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,14,15];
    while let Some(i) = a.next().await {
        print!("{:?}, ", i);
    }
}
