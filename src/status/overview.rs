use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    #[serde(rename = "3dModels")]
    three_dimensional_models: ThreeDimensionalModels,
    athlete_status: Url,
    athletes: Vec<Athlete>,
    clock_dates: DateRange,
    news: News,
    race_dates: DateRange,
    turnpoints: Vec<TurnPoint>,
}

impl Overview {
    pub async fn request() -> Result<Overview, reqwest::Error> {
        Ok(reqwest::get("https://www.redbullxalps.com/fileadmin/live-tracking/2023/race/feeds/cdn-long/overview.json")
        .await?
        .json::<Overview>()
        .await?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ThreeDimensionalModels {
    map_icons: Url,
    paraglider: Url,
    status_icons: Url,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Athlete {
    athlete_id: String,
    bio_url: String,
    chute_color: HexColor,
    chute_strings: HexColor,
    country_code: CountryCode,
    firstname: String,
    firstname_short: String,
    guy_bg_color: HexColor,
    guy_outlines: HexColor,
    hide: bool,
    lastname: String,
    lt_slug: String,
    marker_bg_color: HexColor,
    marker_border_color: HexColor,
    nationality: String,
    news: Url,
    nightpasses_count: usize,
    portrait: Url,
    portrait_retina: Url,
    ranking_portrait: Url,
    ranking_portrait_retina: Url,
    status_bg_color: HexColor,
    team: String,
    text_color: HexColor,
    track_color: HexColor,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DateRange {
    end_time: usize,
    start_time: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct News {
    latest: Url,
    list: Url,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TurnPoint {
    id: String,
    altitude: usize,
    altitude_on_mesh: usize,
    caption: String,
    country: String,
    country_code: CountryCode,
    cylinderradius: usize,
    dist_to_goal: f64,
    header: String,
    hidden: Option<bool>,
    img: Url,
    img_retina: Url,
    lat: f64,
    leg_dist: f64,
    lng: f64,
    lt_slug: String,

    polygon: Option<String>,
    sponsor_img: Option<Url>,
    sponsor_img_retina: Option<Url>,
    sponsor_url: Option<Url>,
    sponsors: Option<Vec<Sponsor>>,
    tot_dist: f64,
    visible_on_map: bool,
    weather: Url,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Sponsor {
    sponsor_img: Url,
    sponsor_img_retina: Url,
    sponsor_url: Url,
}

type HexColor = String;
type Url = String;

#[cfg(test)]
mod test {
    use super::Overview;
    #[tokio::test]
    async fn load_overview() {
        assert_eq!(true, Overview::request().await.is_ok());
    }
}
