use serde_json::Value;
use thiserror::Error;

/// The main struct for elevation
pub struct Elevation {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub resolution: String,
}

impl Elevation {
    /// Get elevation from coordinates.
    pub async fn from_coords(lat: f64, lon: f64) -> Result<Self, ElevationError> {
        let res = match surf::get(&format!(
            "https://elevation-api.io/api/elevation?points={},{}",
            lat, lon
        ))
        .recv_string()
        .await
        {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Http(error.to_string())),
        };

        let json: Value = match serde_json::from_str(&res) {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Json(error.to_string())),
        };

        let (latitude, longitude, elevation) = match Self::parse_json(&json["elevations"][0]) {
            Ok(data) => data,
            Err(error) => return Err(error),
        };

        let resolution = match &json["resolution"] {
            Value::String(s) => s,
            _ => {
                return Err(ElevationError::Json(
                    "couldn't find the resolution".to_string(),
                ))
            }
        };

        return Ok(Self {
            latitude,
            longitude,
            elevation,
            resolution: resolution.to_string(),
        });
    }

    /// Get elevations from multiple coordinates in one request.
    pub async fn from_multiple_coords(coords: Vec<[f64; 2]>) -> Result<Vec<Self>, ElevationError> {
        let mut uri = String::from("https://elevation-api.io/api/elevation?points=");

        for coord in &coords {
            uri.push_str(&format!("({},{}),", coord[0], coord[1]));
        }

        let res = match surf::get(&uri).recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Http(error.to_string())),
        };

        let json: Value = match serde_json::from_str(&res) {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Json(error.to_string())),
        };

        let mut results: Vec<Self> = Vec::new();

        for (x, _) in coords.into_iter().enumerate() {
            let (latitude, longitude, elevation) = match Self::parse_json(&json["elevations"][x]) {
                Ok(data) => data,
                Err(error) => return Err(error),
            };

            let resolution = match &json["resolution"] {
                Value::String(s) => s,
                _ => {
                    return Err(ElevationError::Json(
                        "couldn't find the resolution".to_string(),
                    ))
                }
            };

            let s = Self {
                latitude,
                longitude,
                elevation,
                resolution: resolution.to_string(),
            };

            results.push(s);
        }

        return Ok(results);
    }

    /// Get elevation from the name of a location.
    pub async fn from_location(name: &str) -> Result<Self, ElevationError> {
        let uri = &format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json",
            name.replace(" ", "+")
        );

        let geocode = match surf::get(uri).recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Http(error.to_string())),
        };

        let geocode_json: Value = match serde_json::from_str(&geocode) {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Json(error.to_string())),
        };

        let lat = match &geocode_json[0]["lat"] {
            Value::String(s) => s,
            _ => {
                return Err(ElevationError::Json(
                    "couldn't find the geocoded latitude".to_string(),
                ))
            }
        };

        let lon = match &geocode_json[0]["lon"] {
            Value::String(s) => s,
            _ => {
                return Err(ElevationError::Json(
                    "couldn't find the geocoded longitude".to_string(),
                ))
            }
        };

        let lat: f64 = match lat.parse() {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Json(error.to_string())),
        };

        let lon: f64 = match lon.parse() {
            Ok(data) => data,
            Err(error) => return Err(ElevationError::Json(error.to_string())),
        };

        return Self::from_coords(lat, lon).await;
    }

    fn parse_json(json: &Value) -> Result<(f64, f64, f64), ElevationError> {
        let latitude = match &json["lat"] {
            Value::Number(s) => match s.as_f64() {
                Some(data) => data,
                None => return Err(ElevationError::Json("latitude isn't a number".to_string())),
            },
            _ => {
                return Err(ElevationError::Json(
                    "couldn't find the latitude".to_string(),
                ))
            }
        };

        let longitude = match &json["lon"] {
            Value::Number(s) => match s.as_f64() {
                Some(data) => data,
                None => return Err(ElevationError::Json("longitude isn't a number".to_string())),
            },
            _ => {
                return Err(ElevationError::Json(
                    "couldn't find the longitude".to_string(),
                ))
            }
        };

        let elevation = match &json["elevation"] {
            Value::Number(s) => match s.as_f64() {
                Some(data) => data,
                None => return Err(ElevationError::Json("elevation isn't a number".to_string())),
            },
            _ => {
                return Err(ElevationError::Json(
                    "couldn't find the elevation".to_string(),
                ))
            }
        };

        return Ok((latitude, longitude, elevation));
    }
}

/// An error enum for Elevation.
#[derive(Error, Debug)]
pub enum ElevationError {
    #[error("http error {0}")]
    Http(String),
    #[error("json error {0}")]
    Json(String),
}
