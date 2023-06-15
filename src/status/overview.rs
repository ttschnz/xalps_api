use isocountry::CountryCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Overview {
    #[serde(rename = "3dModels")]
    pub three_dimensional_models: ThreeDimensionalModels,
    pub athlete_status: Url,
    pub athletes: Vec<Athlete>,
    pub clock_dates: DateRange,
    pub news: News,
    pub race_dates: DateRange,
    pub turnpoints: Vec<TurnPoint>,
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
pub struct ThreeDimensionalModels {
    map_icons: Url,
    paraglider: Url,
    status_icons: Url,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Athlete {
    pub athlete_id: String,
    pub bio_url: String,
    pub chute_color: HexColor,
    pub chute_strings: HexColor,
    pub country_code: CountryCode,
    pub firstname: String,
    pub firstname_short: String,
    pub guy_bg_color: HexColor,
    pub guy_outlines: HexColor,
    pub hide: bool,
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
    pub team: String,
    pub text_color: HexColor,
    pub track_color: HexColor,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    end_time: usize,
    start_time: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub latest: Url,
    pub list: Url,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub sponsor_img: Url,
    pub sponsor_img_retina: Url,
    pub sponsor_url: Url,
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
