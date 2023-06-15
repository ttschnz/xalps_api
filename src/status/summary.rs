use cli_table::{format::Justify, Table};

use crate::{ApiTrackResponse, Overview, RaceStatus};

use super::race_status::AthleteStatus;

#[derive(Debug, Table)]
pub struct AthleteSummary {
    #[table(title = "Name", justify = "Justify::Left")]
    full_name: String,
    #[table(title = "Code", justify = "Justify::Left")]
    team: String,
    #[table(title = "Altitude (m.a.s.l)", justify = "Justify::Right")]
    altitude: SomeOrNaN<f32>,
    #[table(title = "Speed (km/h)", justify = "Justify::Right")]
    speed: SomeOrNaN<f32>,
    #[table(title = "Distance from Target (km)", justify = "Justify::Right")]
    distance: f64,
    #[table(title = "Rank", justify = "Justify::Left")]
    rank: usize,
    #[table(title = "Status", justify = "Justify::Left")]
    status: AthleteStatus,
}

impl AthleteSummary {
    pub async fn request() -> Result<Vec<AthleteSummary>, Box<dyn std::error::Error>> {
        let overview = Overview::request().await?;
        let mut stati = vec![];
        let all_status = RaceStatus::request().await?;

        let mut ranking = all_status
            .iter()
            .map(|status| (status.athlete_id.clone(), status.distance_to_goal))
            .collect::<Vec<(String, f64)>>();

        ranking.sort_by(|athlete_a, athlete_b| {
            athlete_a
                .1
                .partial_cmp(&athlete_b.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for athlete in overview.athletes {
            let current_status = all_status
                .iter()
                .find(|status| status.athlete_id == athlete.athlete_id)
                .ok_or(format!(
                    "could not find status of athlete with id {}",
                    athlete.athlete_id
                ))?;

            let track_response = ApiTrackResponse::request(&athlete.athlete_id.clone()).await?;
            let (altitude, speed) =
                match track_response
                    .get_track_points()
                    .iter()
                    .max_by(|point_a, point_b| {
                        point_a
                            .get_timestamp()
                            .partial_cmp(&point_b.get_timestamp())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }) {
                    Some(latest_trackpoint) => (
                        Some(latest_trackpoint.get_altitude()),
                        Some(latest_trackpoint.get_speed()),
                    ),
                    None => (None, None),
                };
            stati.push(AthleteSummary {
                full_name: athlete.firstname.clone() + " " + &athlete.lastname,
                team: athlete.team.clone(),
                altitude: altitude.into(),
                speed: speed.into(),
                distance: current_status.distance_to_goal,
                rank: ranking
                    .iter()
                    .position(|status| status.0 == athlete.athlete_id)
                    .unwrap(),
                status: current_status.status,
            })
        }

        stati.sort_by_key(|athlete_summary| athlete_summary.rank);

        Ok(stati)
    }
}

#[derive(Debug)]
pub struct SomeOrNaN<T>(Option<T>);

impl std::fmt::Display for SomeOrNaN<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{:.2}", value),
            None => write!(f, "NaN"),
        }
    }
}

impl<T> From<Option<T>> for SomeOrNaN<T> {
    fn from(option: Option<T>) -> Self {
        SomeOrNaN(option)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cli_table::{print_stdout, WithTitle};
    #[tokio::test]
    async fn test_summary() {
        let summary = AthleteSummary::request().await.unwrap();
        print_stdout(summary.with_title()).unwrap();
    }
}
