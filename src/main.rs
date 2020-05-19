use tokio::stream;
use tokio::io::{AsyncBufReadExt, BufReader};
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

async fn stream_equals<T, S1, S2>(mut s1: S1, mut s2: S2) -> bool
where
    T: PartialEq,
    S1: stream::Stream<Item=T> + Unpin,
    S2: stream::Stream<Item=T> + Unpin,
{
    loop {
        match (s1.next().await, s2.next().await) {
            (None, None) => return true,
            (a, b) if a != b => return false,
            _ => (),
        }
    }
}

#[tokio::main]
async fn main() {
    let mut flag = String::new();
    let mut stdin = BufReader::new(tokio::io::stdin());
    println!("Please input the flag:");
    stdin.read_line(&mut flag).await.expect("Did not enter a correct string");

    let enc = const_stream![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let key = const_stream![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let result = stream::iter(flag.as_bytes())
        .zip(key)
        .map(|(a, b)| a ^ b)
        .boxed();
    println!("Your flag is {}.", if stream_equals(result, enc).await { "right" } else { "wrong" });
}
