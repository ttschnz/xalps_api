mod overview;
mod race_status;
pub mod summary;
mod track_response;

pub use self::{
    overview::Overview,
    race_status::{RaceStatus, RaceStatusReplay},
    summary::AthleteSummary,
    track_response::ApiTrackResponse,
};

use chrono::{DateTime, Utc};
use protobuf;

impl ApiTrackResponse {
    pub async fn request(athlete_id: &str) -> Result<ApiTrackResponse, Box<dyn std::error::Error>> {
        let data = reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/athlete/{}/track/latest.pbf",
            athlete_id
        ))
        .await?
        .bytes()
        .await?
        .to_vec();
        Ok(protobuf::Message::parse_from_bytes(&data)?)
    }

    /// # Request a replay of the track
    /// ## Parameters
    /// * `athlete_id` - the id of the athlete
    /// * `date_time` - the date and time of the track
    /// ## Returns
    /// The track data of the athlete at the given time
    /// the data is in the range of -15min to +3min 59s
    /// ## Restrictions
    /// Only rounded to 5min intervalls are allowed. Returns an error if not, this limitation is due to the api.
    /// They respond with 404 and the following xml:
    /// ```xml
    /// <Error>
    ///     <Code>NoSuchKey</Code>
    ///     <Message>The specified key does not exist.</Message>
    /// </Error>
    ///```
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// use xalps::ApiTrackResponse;
    /// use chrono::{DateTime, TimeZone, Utc};
    /// let response = ApiTrackResponse::request_replay(
    ///     "29",
    ///     Utc.from_utc_datetime(
    ///        &DateTime::parse_from_rfc3339("2023-06-14T16:55:00Z")
    ///            .unwrap()
    ///            .naive_utc(),
    ///     ),
    /// );
    /// assert!(response.await.is_ok());
    /// # });
    ///```
    ///
    pub async fn request_replay(
        athlete_id: &str,
        date_time: DateTime<Utc>,
    ) -> Result<ApiTrackResponse, Box<dyn std::error::Error>> {
        println!(
            "{}",
            format!(
                "https://rbxltdata.redbullxalps.com/race/athlete/{}/track/latest-replay/{}.pbf",
                athlete_id,
                date_time.to_rfc3339()
            )
        );
        let data = reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/athlete/{}/track/latest-replay/{}.pbf",
            athlete_id,
            date_time.format("%Y-%m-%dT%H:%M:%SZ")
        ))
        .await?
        .bytes()
        .await?
        .to_vec();
        Ok(protobuf::Message::parse_from_bytes(&data)?)
    }

    /// # Request a reduced track
    /// This allows a greater time range, but the data is reduced to a certain amount of points per time.
    /// ## Parameters
    /// * `athlete_id` - the id of the athlete
    /// ## Returns
    /// The track data of the whole race with a low resolution
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// use xalps::ApiTrackResponse;
    /// let response = ApiTrackResponse::request_reduced("29").await;
    /// assert!(response.is_ok());
    /// # });
    /// ````
    pub async fn request_reduced(
        athlete_id: &str,
    ) -> Result<ApiTrackResponse, Box<dyn std::error::Error>> {
        let data = reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/athlete/{}/track/reduced.pbf",
            athlete_id
        ))
        .await?
        .bytes()
        .await?
        .to_vec();
        Ok(protobuf::Message::parse_from_bytes(&data)?)
    }
}

#[cfg(test)]
mod test {
    use super::ApiTrackResponse;
    use chrono::{DateTime, TimeZone, Utc};
    #[tokio::test]
    async fn track_response() {
        let response = ApiTrackResponse::request("29").await.unwrap();
        println!("{:#?}", response);
    }
    #[tokio::test]
    async fn track_reduced() {
        let response = ApiTrackResponse::request_reduced("29").await.unwrap();
        println!("{:#?}", response);
    }
    #[tokio::test]
    async fn track_replay() {
        let response = ApiTrackResponse::request_replay(
            "29",
            Utc.from_utc_datetime(
                &DateTime::parse_from_rfc3339("2023-06-14T16:55:00Z")
                    .unwrap()
                    .naive_utc(),
            ),
        )
        .await
        .unwrap();
        println!("{:#?}", response.track_points.first().unwrap());
        println!("{:#?}", response.track_points.last().unwrap());
    }
}
