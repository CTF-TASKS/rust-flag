use tokio::stream;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::stream::{BoxStream, StreamExt};
use rand::{SeedableRng, rngs::StdRng};
use rand::prelude::*;

const PRINT_ENC: bool = false;

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

fn random_stream(seed: u64) -> impl stream::Stream<Item = u8> {
    let rng = StdRng::seed_from_u64(seed);
    stream::iter(rng.sample_iter(rand::distributions::Standard))
}

#[tokio::main]
async fn main() {
    let mut flag = String::new();
    let mut stdin = BufReader::new(tokio::io::stdin());
    println!("Please input the flag:");
    stdin.read_line(&mut flag).await.expect("Did not enter a correct string");

    let enc = const_stream![57, 21, 34, 244, 149, 112, 229, 145, 7, 61, 139, 206, 120, 194, 82, 157, 225, 139, 46, 110];
    let key = random_stream(0xbabe1337);

    let result = key
        .scan(0xcc, |state, x| {
            *state ^= x;
            futures::future::ready(Some(x))
        })
        .zip(stream::iter(flag.as_bytes()))
        .map(|(a, b)| a ^ b)
        .boxed();
    if PRINT_ENC {
        let result: Vec<u8> = result.collect().await;
        println!("Enc: {:?}", result);
    } else {
        println!("Your flag is {}.", if stream_equals(result, enc).await { "right" } else { "wrong" });
    }
}
