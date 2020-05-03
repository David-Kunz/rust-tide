use async_std::io;
use async_std::task;
use tide::Server;
use async_std::fs::read_to_string;

use sqlx::SqlitePool;
use std::env;

mod add_routes;
mod entity;
mod get_entities;
mod parse_csn;
mod cqn;
mod url_to_cqn;

pub struct State {
    definitions: parse_csn::Definitions,
    pool: sqlx::SqlitePool
}

fn main() -> io::Result<()> {
    task::block_on(async {
        // let entities = get_entities::get_entities();
        let csn = read_to_string("csn.json").await?;
        let definitions = parse_csn::Definitions::from_str(&csn).expect("Cannot parse csn");
        let pool = SqlitePool::new(&env::var("DATABASE_URL").unwrap()).await.unwrap();
        let state = State { definitions, pool};
        let mut app = Server::with_state(state);
        add_routes::add_routes(&mut app);
        let url = "127.0.0.1:8080";
        println!("Server listening on http://{}", &url);
        app.listen(&url).await?;
        Ok(())
    })
}
