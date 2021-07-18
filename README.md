# elevation
Get elevations from coordinates and names of locations.

Get an elevation from coordinates:
```rust
let e = Elevation::from_coords(45.0, 100.0).await.unwrap();

println!("({}, {}): {}", e.latitude, e.longitude, e.elevation);
```
```
(45, 100): 2057
```

Get elevations from multiple coordinates:
```rust
    let e = Elevation::from_multiple_coords(vec![[40.828, -73.9206], [41.1955, -82.424]])
        .await
        .unwrap();

    for x in e {
        println!("({}, {}): {}", x.latitude, x.longitude, x.elevation);
    }
```
```
(40.828, -73.9206): 12
(41.1955, -82.424): 277
```

Get an elevation from the name of a location:
```rust
let e = Elevation::from_location("Seattle, Washington")
    .await
    .unwrap();

println!("({}, {}): {}", e.latitude, e.longitude, e.elevation);
```
```
(47.6038321, -122.3300624): 77.5
```