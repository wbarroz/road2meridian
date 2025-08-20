mod handlers;
mod models;
mod state;

use handlers::create::create_data;
use handlers::delete::delete_data;
use handlers::read::{read_all_data, read_data};
use handlers::update::update_data;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = state::new_state();
    let mut app = tide::with_state(state);
    app.at("/data").post(create_data);
    app.at("/data").get(read_all_data);
    app.at("/data/:id").get(read_data);
    app.at("/data/:id").put(update_data);
    app.at("/data/:id").delete(delete_data);

    //  let addr = "127.0.0.1:8080";
    let addr = "0.0.0.0:8080";
    println!("Servidor Tide rodando em http://{addr}");

    app.listen(addr).await?;
    Ok(())
}
