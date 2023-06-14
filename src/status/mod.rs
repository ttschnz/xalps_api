mod overview;
mod race_status;
mod track_response;

pub use self::{
    overview::Overview,
    race_status::{RaceStatus, RaceStatusReplay},
    track_response::ApiTrackResponse,
};

use quick_protobuf::deserialize_from_slice;

impl<'a> ApiTrackResponse<'a> {
    pub async fn request(athlete_id: &str) -> Result<Vec<u8>, reqwest::Error> {
        Ok(reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/athlete/{}/track/latest.pbf",
            athlete_id
        ))
        .await?
        .bytes()
        .await?
        .to_vec())
    }
    pub fn parse(data: &'a Vec<u8>) -> Result<ApiTrackResponse<'a>, quick_protobuf::Error> {
        let bytes: &'a [u8] = data;
        Ok(deserialize_from_slice(bytes)?)
    }
}

#[cfg(test)]
mod test {
    use super::track_response::ApiTrackResponse;
    use base64::{engine::general_purpose, Engine as _};
    use std::fs;

    use chrono::Local;

    #[tokio::test]
    async fn track_latest() {
        let data = ApiTrackResponse::request("29").await.unwrap();
        // write to log file
        let s = general_purpose::STANDARD.encode(&data);

        fs::write(format!("./logs/{:?}.pbf", Local::now()), s).unwrap();
        // let mut f = File::create(format!("./logs/{:?}.pbf", Local::now())).unwrap();
        // f.write

        let response = ApiTrackResponse::parse(&data).unwrap();

        println!("{:?}", response);
    }
    // #[tokio::test]
    // async fn parse_track_reduced() {
    //     let data =
    //         reqwest::get("https://rbxltdata.redbullxalps.com/race/athlete/29/track/reduced.pbf")
    //             .await
    //             .unwrap()
    //             .bytes()
    //             .await
    //             .unwrap()
    //             .to_vec();

    //     let bytes: &[u8] = &data;
    //     println!("{}", bytes.len());

    //     let response: ApiTrackResponse = deserialize_from_slice(bytes).unwrap();

    //     println!("{:?}", response);
    // }
}
