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
        "0".to_string(),
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
        "0".to_string(),
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
        "0".to_string(),
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

    let mut journey = Journey::new("0".to_string(), vec![]);
    for i in 0..=5 {
        journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30 * i)));
    }

    // once the 6th position is received, the system sets the user's status as "not moving".
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30 * 6)));

    // the 7th position will be counted as a break.
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30 * 7)));

    let positions = vec![journey];
    let (t_moving, t_not_moving) = get_movement_stats(&positions);

    assert_eq!(
        t_moving,
        Duration::from(TimeDelta::try_seconds(180).unwrap())
    );
    assert_eq!(
        t_not_moving,
        Duration::from(TimeDelta::try_seconds(30).unwrap())
    );
}

#[tokio::test]
async fn movement_and_pause() {
    let dt = "2015-06-03T12:34:56Z".parse::<DateTime<Utc>>().unwrap();

    let mut journey = Journey::new("0".to_string(), vec![]);

    // user moves for 60s...
    journey.push(Coordinates::new(10.0, 10.0, dt));
    journey.push(Coordinates::new(10.1, 10.0, dt + Duration::seconds(30)));
    journey.push(Coordinates::new(10.1, 10.1, dt + Duration::seconds(60)));

    // then they stand still (150s counted as movement)
    for i in 3..8 {
        journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30 * i)));
    }

    // now 180s have passed since the first (10.0, 10.0), the user is marked as not moving
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30 * 8)));

    // this is counted as a break
    journey.push(Coordinates::new(10.0, 10.0, dt + Duration::seconds(30 * 9)));

    // finally they start moving again (t_moving += 30)
    journey.push(Coordinates::new(
        10.0,
        12.0,
        dt + Duration::seconds(30 * 10),
    ));

    let positions = vec![journey];
    let (t_moving, t_not_moving) = get_movement_stats(&positions);

    assert_eq!(
        t_moving,
        Duration::from(TimeDelta::try_seconds(60 + 180 + 30).unwrap())
    );
    assert_eq!(
        t_not_moving,
        Duration::from(TimeDelta::try_seconds(30).unwrap())
    );
}
