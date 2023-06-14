mod overview;
mod race_status;
mod track_response;

pub use self::{
    overview::Overview,
    race_status::{RaceStatus, RaceStatusReplay},
    track_response::ApiTrackResponse,
};

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
}

#[cfg(test)]
mod test {
    use super::ApiTrackResponse;
    #[tokio::test]
    async fn track_response() {
        let response = ApiTrackResponse::request("29").await.unwrap();
        println!("{:?}", response);
    }
}
