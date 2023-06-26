use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    #[serde(rename = "3dModels")]
    /// contains the URLs to the 3D models used in the map
    pub three_dimensional_models: ThreeDimensionalModels,
    /// is the URL to where the athlete's statzs can be found
    pub athlete_status: Url,
    /// A list of all athletes that are participating in the race
    pub athletes: Vec<Athlete>,
    // start and end time of the race. This seems to be redundant to `race_dates`
    pub clock_dates: DateRange,
    /// URLs to where race-news can be found
    pub news: News,
    /// start and end time of the race. This seems to be redundant to `clock_dates`
    pub race_dates: DateRange,
    /// a list of all turnpoints
    pub turnpoints: Vec<TurnPoint>,
}

impl Overview {
    /// # Race Overview
    /// Gives a general idea of how the race is running.
    /// This data is updated every few days.
    /// ## Returns
    /// A `Overview` struct.
    /// ## Example
    /// ```rust
    /// # tokio_test::block_on(async {
    /// use xalps::Overview;
    /// let response = Overview::request().await;
    /// assert!(response.is_ok());
    /// # });
    pub async fn request() -> Result<Overview, reqwest::Error> {
        Ok(reqwest::get("https://www.redbullxalps.com/fileadmin/live-tracking/2023/race/feeds/cdn-long/overview.json")
        .await?
        .json::<Overview>()
        .await?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// URLs to 3d models used in the map.
/// I did not follow these urls or their usage in the map.
pub struct ThreeDimensionalModels {
    map_icons: Url,
    paraglider: Url,
    status_icons: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// The details of an athlete.
pub struct Athlete {
    /// Unique identifier of the athlete. A positive Integer stored as String.
    pub athlete_id: String,
    pub bio_url: String,
    pub chute_color: HexColor,
    pub chute_strings: HexColor,
    pub country_code: CountryCode,
    /// The first name of the athlete.
    pub firstname: String,
    /// A shortened version of the first name. i.e. "M." for "Max"
    pub firstname_short: String,
    pub guy_bg_color: HexColor,
    pub guy_outlines: HexColor,
    pub hide: bool,
    /// The last name of the athlete.
    pub lastname: String,
    pub lt_slug: String,
    pub marker_bg_color: HexColor,
    pub marker_border_color: HexColor,
    pub nationality: String,
    pub news: Url,
    pub nightpasses_count: usize,
    pub portrait: Url,
    pub portrait_retina: Url,
    pub ranking_portrait: Url,
    pub ranking_portrait_retina: Url,
    pub status_bg_color: HexColor,
    /// The "team" of the athlete. Since it is a one-person-team, this can be used as ID.
    /// It is built of the country code, followed by a number. e.g. "AUT1"
    pub team: String,
    pub text_color: HexColor,
    pub track_color: HexColor,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    end_time: usize,
    start_time: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub latest: Url,
    pub list: Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// # Turnpoint
/// A turnpoint is a point on the map that the athletes have to pass.
pub struct TurnPoint {
    pub id: String,
    pub altitude: usize,
    pub altitude_on_mesh: usize,
    pub caption: String,
    pub country: String,
    pub country_code: CountryCode,
    pub cylinderradius: usize,
    pub dist_to_goal: f64,
    pub header: String,
    pub hidden: Option<bool>,
    pub img: Url,
    pub img_retina: Url,
    pub lat: f64,
    pub leg_dist: f64,
    pub lng: f64,
    pub lt_slug: String,
    pub polygon: Option<String>,
    pub sponsor_img: Option<Url>,
    pub sponsor_img_retina: Option<Url>,
    pub sponsor_url: Option<Url>,
    pub sponsors: Option<Vec<Sponsor>>,
    pub tot_dist: f64,
    pub visible_on_map: bool,
    pub weather: Url,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub sponsor_img: Url,
    pub sponsor_img_retina: Url,
    pub sponsor_url: Url,
}

/// A hex color code, represented as a string. e.g. "#ff0000". It is not checked if the color is valid.
type HexColor = String;

/// An url, represented as a string. It is not checked if the url is valid.
type Url = String;

#[cfg(test)]
mod test {
    use super::Overview;
    #[tokio::test]
    async fn load_overview() {
        assert_eq!(true, Overview::request().await.is_ok());
    }
}
