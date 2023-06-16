use chrono::{DateTime, Local};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RaceStatusReplay {
    pub timestamp: usize,
    pub status: Vec<RaceStatus>,
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
    pub athlete_id: String,
    pub timestamp: usize,
    pub status: AthleteStatus,
    pub distance_to_goal: f64,
    pub altitude: usize,
    pub speed: usize,
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

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AthleteStatus {
    Rest,
    Fly,
    Hike,
    Automatic, // ? not shure what for
    Out,
}

impl AthleteStatus {
    pub fn verbalize(&self) -> String {
        match self {
            AthleteStatus::Rest => "resting",
            AthleteStatus::Fly => "flying",
            AthleteStatus::Hike => "hiking",
            AthleteStatus::Automatic => "<unknown>",
            AthleteStatus::Out => "is out",
        }
        .to_string()
    }
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
