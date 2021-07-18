use elevation::Elevation;

#[async_std::main]
async fn main() {
    let e = Elevation::from_multiple_coords(vec![[40.828, -73.9206], [41.1955, -82.424]])
        .await
        .unwrap();

    for x in e {
        println!("({}, {}): {}m", x.latitude, x.longitude, x.elevation);
    }
}
