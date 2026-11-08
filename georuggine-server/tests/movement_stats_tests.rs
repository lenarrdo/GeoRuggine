use chrono::{DateTime, Duration, TimeDelta, Utc};
use georuggine_server::messages::position::{Coordinates, Journey, service::get_movement_stats};

#[tokio::test]
async fn no_positions() {
    let journeys: Vec<Journey> = vec![];
    let (t_moving, t_not_moving) = get_movement_stats(&journeys);

    assert_eq!(t_moving, Duration::zero());
    assert_eq!(t_not_moving, Duration::zero());
}

#[tokio::test]
async fn one_position() {
    let journeys: Vec<Journey> = vec![Journey::new(
        1,
        vec![Coordinates::new(10.0, 10.0, Utc::now())],
    )];
    let (t_moving, t_not_moving) = get_movement_stats(&journeys);

    // one position is not enough data to compute time spent moving
    assert_eq!(t_moving, Duration::zero());
    assert_eq!(t_not_moving, Duration::zero());
}

#[tokio::test]
async fn user_moving_two_positions() {
    let dt = "2015-06-03T12:34:56Z".parse::<DateTime<Utc>>().unwrap();
    let journeys: Vec<Journey> = vec![Journey::new(
        1,
        vec![
            Coordinates::new(10.0, 10.0, dt),
            Coordinates::new(10.2, 10.0, dt + Duration::seconds(30)),
        ],
    )];
    let (t_moving, t_not_moving) = get_movement_stats(&journeys);

    assert_eq!(
        t_moving,
        Duration::from(TimeDelta::try_seconds(30).unwrap())
    );
    assert_eq!(t_not_moving, Duration::zero());
}

#[tokio::test]
async fn user_moving_many_positions() {
    let dt = "2015-06-03T12:34:56Z".parse::<DateTime<Utc>>().unwrap();
    let joruneys: Vec<Journey> = vec![Journey::new(
        1,
        vec![
            Coordinates::new(10.0, 10.0, dt),
            Coordinates::new(10.2, 10.0, dt + Duration::seconds(30)),
            Coordinates::new(10.2, 10.0, dt + Duration::seconds(60)),
            Coordinates::new(10.2, 10.0, dt + Duration::seconds(90)),
            Coordinates::new(10.5, 10.0, dt + Duration::seconds(120)),
        ],
    )];
    let (t_moving, t_not_moving) = get_movement_stats(&joruneys);

    assert_eq!(
        t_moving,
        Duration::from(TimeDelta::try_seconds(120).unwrap())
    );
    assert_eq!(t_not_moving, Duration::zero());
}

#[tokio::test]
async fn pause_is_detected() {
    let dt = "2015-06-03T12:34:56Z".parse::<DateTime<Utc>>().unwrap();

    let mut journey = Journey::new(1, vec![]);

    // starts moving
    journey.push(Coordinates::new(10.0, 10.0, dt));
    assert_eq!(
        (Duration::seconds(0), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // 30 seconds spent moving
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30)));
    assert_eq!(
        (Duration::seconds(30), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // 60 seconds (1 minute spent moving)
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(60)));
    assert_eq!(
        (Duration::seconds(60), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // 90 seconds
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(90)));
    assert_eq!(
        (Duration::seconds(90), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // 120 seconds (2 minutes spent moving)
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(120)));
    assert_eq!(
        (Duration::seconds(120), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // 150 seconds
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(150)));
    assert_eq!(
        (Duration::seconds(150), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // 180 seconds (3 minutes spent moving. Now the user is marked as stopped)
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(180)));
    assert_eq!(
        (Duration::seconds(180), Duration::seconds(0)),
        journey.get_movement_stats()
    );

    // first 30 seconds spent not moving
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(210)));
    assert_eq!(
        (Duration::seconds(180), Duration::seconds(30)),
        journey.get_movement_stats()
    );

    // Now they have started moving again, so the state has gone from stopped -> moving.
    // The thing is that these last 30 seconds are counted as not moving, since they
    // were obtained when the user was in that state.
    journey.push(Coordinates::new(11.0, 10.0, dt + Duration::seconds(240)));
    assert_eq!(
        (Duration::seconds(180), Duration::seconds(60)),
        journey.get_movement_stats()
    );
}
