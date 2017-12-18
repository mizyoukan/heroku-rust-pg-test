#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate native_tls;
extern crate postgres;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, const_service, service_fn};
use postgres::tls::native_tls::NativeTls;
use postgres::{Connection, TlsMode};
use std::env;

const PHRASE: &'static [u8] = b"Hello world!";

error_chain!{
    foreign_links {
        Hyper(::hyper::Error);
        NativeTls(::native_tls::Error);
        Postgres(::postgres::Error);
        Var(::std::env::VarError);
    }
}

quick_main!(|| -> Result<()> {
    let url = env::var("DATABASE_URL")?;
    let negotiator = NativeTls::new()?;
    let _conn = Connection::connect(url, TlsMode::Require(&negotiator))?;

    let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8000);
    let addr = ([127, 0, 0, 1], port).into();

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
