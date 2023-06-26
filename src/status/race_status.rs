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
    /// # Request a replay of the status of the race
    /// This fetches all RaceStatus there were at the given date. The data is in a 1min intervall.
    /// ## Parameters
    /// - `date` The local date-time. Only year, month and day are used.
    /// ## Panics
    /// If the date is in the future or outside of the race-range (can be found in `Overview::request().await?.race_dates`).
    /// ## Returns
    /// A vector of `RaceStatusReplay`es. One for each minute of the day, which then contains the status of all athletes at that time.
    /// If the day has passed, the length of the vector is 1440 (24h * 60min), if the date is today, the length is the fraction of the day that has passed.
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// use xalps::RaceStatusReplay;
    /// use chrono::{DateTime, TimeZone, Utc};
    /// let response = RaceStatusReplay::request(
    ///    chrono::offset::Local::now()
    /// );
    /// assert!(response.await.is_ok());
    /// # });
    ///
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

/// # The status of an athlete
/// Describes how the athlete is doing.
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
    /// # Request the status of the race
    /// This is the latest data there is on the whole race (all athletes).
    /// ## Returns
    /// A Vector of `RaceStatus`es, one for each athlete. The vector is not sorted.
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// use xalps::RaceStatus;
    /// let response = RaceStatus::request().await;
    /// assert!(response.is_ok());
    /// # });
    pub async fn request() -> Result<Vec<RaceStatus>, reqwest::Error> {
        Ok(reqwest::get(format!(
            "https://rbxltdata.redbullxalps.com/race/race-status"
        ))
        .await?
        .json::<Vec<RaceStatus>>()
        .await?)
    }
}

/// # The status of an athlete
/// An athlete can be in one of these states.
/// - `Rest` - the athlete is resting whenever he is not moving
/// - `Fly` - the athlete is flying if the distance to the ground is greater than a certain threshold
/// - `Hike` - the athlete is hiking if he is moving on the ground
/// - `Out` - The athlete has ben disqualified or has given up
/// - `Automatic` - I did not find out what this is for
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
    /// # Verbalize the status
    /// This returns a string that describes the status of the athlete in form of a verb.
    /// It is helpful for e.g. notifications.
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
