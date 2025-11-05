use std::net::ToSocketAddrs;

use clap::{value_t, Arg};
use ntex::http::client::Client;
use ntex::util::Bytes;
use ntex::web::{self, middleware, App, Error, HttpRequest, HttpResponse};
use url::Url;

async fn forward(
    req: HttpRequest,
    body: Bytes,
    url: web::types::State<Url>,
    client: web::types::State<Client>,
) -> Result<HttpResponse, Error> {
    // Direct respond for OPTIONS requests.
    if req.method() == ntex::http::Method::OPTIONS {
        let mut client_resp = HttpResponse::build(ntex::http::StatusCode::NO_CONTENT);

        client_resp.header(ntex::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*");
        client_resp.header(ntex::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true");
        client_resp.header(ntex::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Authorization,Accept,Origin,DNT,X-CustomHeader,Keep-Alive,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Content-Range,Range");
        client_resp.header(
            ntex::http::header::ACCESS_CONTROL_ALLOW_METHODS,
            "GET,POST,OPTIONS,PUT,DELETE,PATCH",
        );
        client_resp.header(ntex::http::header::ACCESS_CONTROL_MAX_AGE, 1728000);
        client_resp.header(ntex::http::header::CONTENT_TYPE, "text/plain charset=UTF-8");
        client_resp.header(ntex::http::header::CONTENT_LENGTH, 0);

        return Ok(client_resp.finish());
    }

    let mut new_url = url.get_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    // TODO: This forwarded implementation is incomplete as it only handles the inofficial
    // X-Forwarded-For header but not the official Forwarded one.
    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr() {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());

    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    client_resp.header(ntex::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*");

    Ok(client_resp.body(res.body().await?))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let _guard = clia_tracing_config::build()
        .filter_level("info")
        .with_ansi(true)
        .to_stdout(true)
        .directory("./logs")
        .file_name("clia-cors-proxy.log")
        .rolling("daily")
        .init();

    let matches = clap::App::new("HTTP Proxy")
        .arg(
            Arg::with_name("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("listen_port")
                .takes_value(true)
                .value_name("LISTEN PORT")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("forward_addr")
                .takes_value(true)
                .value_name("FWD ADDR")
                .index(3)
                .required(true),
        )
        .arg(
            Arg::with_name("forward_port")
                .takes_value(true)
                .value_name("FWD PORT")
                .index(4)
                .required(true),
        )
        .get_matches();

    let listen_addr = matches.value_of("listen_addr").unwrap();
    let listen_addr = if listen_addr == "localhost" {
        "127.0.0.1"
    } else {
        listen_addr
    };
    let listen_port = value_t!(matches, "listen_port", u16).unwrap_or_else(|e| e.exit());

    let forwarded_addr = matches.value_of("forward_addr").unwrap();
    let forwarded_addr = if forwarded_addr == "localhost" {
        "127.0.0.1"
    } else {
        forwarded_addr
    };
    let forwarded_port = value_t!(matches, "forward_port", u16).unwrap_or_else(|e| e.exit());

    let forward_url = if forwarded_port == 443 {
        Url::parse(&format!(
            "https://{}",
            (forwarded_addr, forwarded_port)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap()
        ))
        .unwrap()
    } else {
        Url::parse(&format!(
            "http://{}",
            (forwarded_addr, forwarded_port)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap()
        ))
        .unwrap()
    };

    web::server(move || {
        App::new()
            .state(Client::new())
            .state(forward_url.clone())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    })
    .bind((listen_addr, listen_port))?
    .stop_runtime()
    .run()
    .await
}
