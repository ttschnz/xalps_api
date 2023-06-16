use cli_table::{format::Justify, Table};

use crate::{Overview, RaceStatus};

use super::race_status::AthleteStatus;

#[derive(Debug, Table)]
pub struct AthleteSummary {
    #[table(title = "Name", justify = "Justify::Left")]
    full_name: String,
    #[table(title = "Code", justify = "Justify::Left")]
    team: String,
    #[table(title = "Altitude (m.a.s.l)", justify = "Justify::Right")]
    altitude: usize,
    #[table(title = "Speed (km/h)", justify = "Justify::Right")]
    speed: usize,
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

            stati.push(AthleteSummary {
                full_name: athlete.firstname.clone() + " " + &athlete.lastname,
                team: athlete.team.clone(),
                altitude: current_status.altitude,
                speed: current_status.speed,
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
