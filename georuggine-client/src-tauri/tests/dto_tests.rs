use georuggine_client_lib::messages::chatmessage::dto::TextMessage;
use georuggine_client_lib::messages::dto::ClientIncomingMessage;
use georuggine_client_lib::messages::position::dto::{CoordinatePoint, PositionUpdate};

#[test]
fn position_without_timestamp_omits_the_field() {
    let position = PositionUpdate {
        lat: 45.0,
        lon: 7.0,
        timestamp: None,
    };

    let json = serde_json::to_value(&position).expect("serialize");

    assert_eq!(json["lat"], 45.0);
    assert_eq!(json["lon"], 7.0);
    assert!(json.get("timestamp").is_none());
}

#[test]
fn position_with_timestamp_serializes_it() {
    let position = PositionUpdate {
        lat: 45.0,
        lon: 7.0,
        timestamp: Some("2026-01-01T10:00:00Z".to_string()),
    };

    let json = serde_json::to_value(&position).expect("serialize");

    assert_eq!(json["timestamp"], "2026-01-01T10:00:00Z");
}

#[test]
fn position_roundtrips() {
    let position = PositionUpdate {
        lat: -12.34,
        lon: 56.78,
        timestamp: Some("2026-01-01T10:00:00Z".to_string()),
    };

    let json = serde_json::to_string(&position).expect("serialize");
    let decoded: PositionUpdate = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(decoded.lat, position.lat);
    assert_eq!(decoded.lon, position.lon);
    assert_eq!(decoded.timestamp, position.timestamp);
}

#[test]
fn coordinate_point_accepts_x_y_fields() {
    let point: CoordinatePoint =
        serde_json::from_str(r#"{"x":1.5,"y":2.5}"#).expect("deserialize");

    assert_eq!(point.x, 1.5);
    assert_eq!(point.y, 2.5);
}

#[test]
fn coordinate_point_accepts_lat_lon_aliases() {
    let point: CoordinatePoint =
        serde_json::from_str(r#"{"lat":1.5,"lon":2.5}"#).expect("deserialize");

    assert_eq!(point.x, 1.5);
    assert_eq!(point.y, 2.5);
}

#[test]
fn coordinate_file_can_be_deserialized_as_a_list() {
    let points: Vec<CoordinatePoint> = serde_json::from_str(
        r#"[{"lat":45.0,"lon":7.0},{"x":46.0,"y":8.0}]"#,
    )
    .expect("deserialize");

    assert_eq!(points.len(), 2);
    assert_eq!(points[0].x, 45.0);
    assert_eq!(points[1].y, 8.0);
}

#[test]
fn client_incoming_message_serializes_text_untagged() {
    let message = ClientIncomingMessage::Text(TextMessage {
        contents: "hello".to_string(),
        timestamp: "2026-01-01T10:00:00Z".to_string(),
    });

    let json = serde_json::to_value(&message).expect("serialize");

    assert_eq!(json["contents"], "hello");
    assert_eq!(json["timestamp"], "2026-01-01T10:00:00Z");
    assert!(json.get("Text").is_none());
    assert!(json.get("Position").is_none());
}

#[test]
fn client_incoming_message_serializes_position_untagged() {
    let message = ClientIncomingMessage::Position(PositionUpdate {
        lat: 45.0,
        lon: 7.0,
        timestamp: None,
    });

    let json = serde_json::to_value(&message).expect("serialize");

    assert_eq!(json["lat"], 45.0);
    assert_eq!(json["lon"], 7.0);
    assert!(json.get("Position").is_none());
    assert!(json.get("timestamp").is_none());
}

#[test]
fn text_message_deserialization_requires_both_fields() {
    assert!(serde_json::from_str::<TextMessage>(r#"{"contents":"hi"}"#).is_err());
    assert!(
        serde_json::from_str::<TextMessage>(r#"{"timestamp":"2026-01-01T10:00:00Z"}"#).is_err()
    );
}
