use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };
use hyper::{ body::HttpBody as _, Client };
use hyper::body::Buf;
use serde::Deserialize;
use tokio::io::{ stdout, AsyncWriteExt as _ };
use tokio::runtime;
use hyper::service::{ make_service_fn, service_fn};
use hyper::{ Body, Method, Request, Response, Server, StatusCode };

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

fn main() {
    println!("Hello, world!");
}

#[derive(Deserialize, Debug)]
struct User {
     id: i32,
     name: String,
}

#[tokio::test]
async fn test_httpClient() {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse().unwrap();
    let mut resp = client.get(uri).await.unwrap();
    println!("Response: {}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk.unwrap()).await.unwrap();
    }
}

#[tokio::test]
async fn test_restapi() {
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();

    let client = Client::new();
    let res = client.get(url).await.unwrap();
    let body = hyper::body:: aggregate(res).await.unwrap();

    let users: Vec<User> = serde_json::from_reader(body.reader()).unwrap();

    println!("사용자: {:#?}", users);
}

#[tokio::test]
async fn test_webserver() -> Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let new_service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                response_examples(req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    let result = server.await?;
    Ok(result)
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    let index_html = String::from("<h1>Hello World!</h>");
    let notfound_html = String::from("<h1>404 not found</h>");

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(index_html.into())),
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(notfound_html.into())
                .unwrap())
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut len_buffer = [0u8; 8];
    stream.read_exact(&mut len_buffer).unwrap();
    let recv_len = i64::from_ne_bytes(len_buffer).try_into().unwrap();

    let mut txt_buffer = vec![0u8; recv_len];
    stream.read_exact(&mut txt_buffer).unwrap();

    let str = String::from_utf8(txt_buffer.to_vec()).unwrap();
    println!("클라이언트: {:?}", str);

    let hello = String::from("안녕! 클라이언트!");
    let bytes = hello.as_bytes();
    let len = bytes.len();

    stream.write_all(&len.to_ne_bytes()).unwrap();
    stream.write_all(&bytes);
}

#[test]
fn test_tcp_server() {
    let listener = TcpListener::bind("localhost:1234").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}

#[test]
fn test_tcp_client() {
    let mut stream = TcpStream::connect("localhost:1234").unwrap();

    let hello = String::from("안녕! 서버!");
    let bytes = hello.as_bytes();
    let len = bytes.len();

    let size_bytes = len.to_ne_bytes();
    let size_bytes_len = size_bytes.len();

    stream.write_all(&len.to_ne_bytes()).unwrap();
    stream.write_all(&bytes);
    stream.flush();

    let mut len_buffer = [0u8; 8];
    stream.read_exact(&mut len_buffer).unwrap();
    let recv_len = i64::from_ne_bytes(len_buffer).try_into().unwrap();

    let mut txt_buffer = vec![0u8; recv_len];
    stream.read_exact(&mut txt_buffer).unwrap();

    let str = String::from_utf8(txt_buffer.to_vec()).unwrap();
    println!("서버: {:?}", str);
}