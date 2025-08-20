use crate::models::DataEntry;
use crate::state::AppState;
use tide::Request;

pub async fn update_data(mut req: Request<AppState>) -> tide::Result {
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => return Err(tide::Error::from_str(100, "Invalid id")),
    };
    let entry: DataEntry = req.body_json().await?;

    let state = req.state();
    let mut map = state.lock().unwrap();
    if let std::collections::hash_map::Entry::Occupied(mut e) = map.entry(id) {
        e.insert(entry);
        Ok(tide::Response::new(200))
    } else {
        Ok(tide::Response::new(200))
    }
}
