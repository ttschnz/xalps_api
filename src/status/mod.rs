mod overview;
mod race_status;
mod track_response;

pub use self::{
    overview::Overview,
    race_status::{RaceStatus, RaceStatusReplay},
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

    /// # Only 5 Minute gaps allowed
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
        println!("{:?}", response);
    }
    #[tokio::test]
    async fn track_reduced() {
        let response = ApiTrackResponse::request_reduced("29").await.unwrap();
        println!("{:?}", response);
    }
    #[tokio::test]
    async fn track_replay() {
        let response = ApiTrackResponse::request_replay(
            "29",
            Utc.from_utc_datetime(
                &DateTime::parse_from_rfc3339("2023-06-14T16:50:00Z")
                    .unwrap()
                    .naive_utc(),
            ),
        )
        .await
        .unwrap();
        println!("{:?}", response);
    }
}
