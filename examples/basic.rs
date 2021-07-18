use elevation::Elevation;

#[async_std::main]
async fn main() {
    let e = Elevation::from_coords(45.0, 100.0).await.unwrap();

    println!("({}, {}): {}", e.latitude, e.longitude, e.elevation);
}
