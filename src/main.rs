use async_std::fs::read_to_string;
use async_std::io;
use async_std::task;
use tide::Server;

use sqlx::SqlitePool;
use std::env;

mod add_routes;
mod cqn;
mod cqn_to_result;
mod csn;
mod cds;
mod url_to_cqn;

pub struct State {
    cds: cds::CDS,
    pool: sqlx::SqlitePool,
}


fn main() -> io::Result<()> {
    task::block_on(async {
        let csn = read_to_string("csn.json").await?;
        let cds = cds::CDS { definitions: csn::Definitions::from_str(&csn).expect("Cannot parse csn") };
        let service_names = cds.definitions.get_service_names();
        let pool = SqlitePool::new(
            &env::var("DATABASE_URL").expect("Please set env variable DATABASE_URL"),
        )
        .await
        .unwrap();
        let state = State { cds, pool };
        let mut app = Server::with_state(state);
        add_routes::add_routes(&mut app, service_names);
        let url = "127.0.0.1:8080";
        println!("Server listening on http://{}", &url);
        app.listen(&url).await?;
        Ok(())
    })
}
