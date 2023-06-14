use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
// GET https://rbxltdata.redbullxalps.com/race/race-status-replay_{yyyy}-{mm}-{dd} => Vec<RaceStatusReplay>
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RaceStatusReplay {
    timestamp: usize,
    status: Vec<RaceStatus>,
}

impl RaceStatusReplay {
    pub async fn request(date: DateTime<Local>) -> Result<Vec<RaceStatusReplay>, reqwest::Error> {
        Ok(reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/race-status-replay_{}",
            date.format("%Y-%m-%d")
        ))
        .await?
        .json::<Vec<RaceStatusReplay>>()
        .await?)
    }

    pub async fn request_today() -> Result<Vec<RaceStatusReplay>, reqwest::Error> {
        RaceStatusReplay::request(chrono::offset::Local::now()).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RaceStatus {
    athlete_id: String,
    timestamp: usize,
    status: AthleteStatus,
    distance_to_goal: f64,
    altitude: usize,
}

impl RaceStatus {
    pub async fn request() -> Result<Vec<RaceStatus>, reqwest::Error> {
        Ok(reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/race-status"
        ))
        .await?
        .json::<Vec<RaceStatus>>()
        .await?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum AthleteStatus {
    Rest,
    Fly,
    Hike,
}

#[cfg(test)]
mod test {
    use super::{RaceStatus, RaceStatusReplay};

    #[tokio::test]
    async fn replay_today() {
        assert_eq!(true, RaceStatusReplay::request_today().await.is_ok())
    }

    #[tokio::test]
    async fn status() {
        assert_eq!(true, RaceStatus::request().await.is_ok())
    }
}
