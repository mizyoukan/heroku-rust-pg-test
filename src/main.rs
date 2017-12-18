#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate openssl;
extern crate postgres;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, const_service, service_fn};
use openssl::ssl::{SslMethod, SslConnectorBuilder, SSL_VERIFY_NONE};
use postgres::tls::openssl::OpenSsl;
use postgres::{Connection, TlsMode};
use std::env;

const PHRASE: &'static [u8] = b"Hello world!";

error_chain!{
    foreign_links {
        Hyper(::hyper::Error);
        Postgres(::postgres::Error);
        Ssl(::openssl::error::ErrorStack);
        Var(::std::env::VarError);
    }
}

quick_main!(|| -> Result<()> {
    let mut connector = SslConnectorBuilder::new(SslMethod::tls())?;
    connector.set_verify(SSL_VERIFY_NONE);
    let openssl = OpenSsl::from(connector.build());
    let url = env::var("DATABASE_URL")?;
    let _conn = Connection::connect(url, TlsMode::Require(&openssl))?;

    let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(80);
    let addr = ([0, 0, 0, 0], port).into();

    let service = const_service(service_fn(|_| {
        Ok(Response::<hyper::Body>::new()
            .with_header(ContentLength(PHRASE.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(PHRASE))
    }));

    let mut server = Http::new().bind(&addr, service)?;
    server.no_proto();
    println!("Listening on {} with 1 thread.", server.local_addr()?);
    server.run()?;

    Ok(())
});
