use async_std::io;
use async_std::task;
use tide::Server;

mod add_routes;
mod entity;
mod get_entities;

fn main() -> io::Result<()> {
    let entities = get_entities::get_entities();
    task::block_on(async {
        let mut app = Server::with_state(entities);
        add_routes::add_routes(&mut app);
        let url = "127.0.0.1:8080";
        println!("Server listening on http://{}", &url);
        app.listen(&url).await?;
        Ok(())
    })
}
