use elevation::Elevation;

#[async_std::main]
async fn main() {
    let e = Elevation::from_location("Seattle, Washington")
        .await
        .unwrap();

    println!("({}, {}): {}", e.latitude, e.longitude, e.elevation);
}
