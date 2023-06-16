use super::race_status::AthleteStatus;
use crate::{Overview, RaceStatus};
use chrono::{TimeZone, Utc};
use cli_table::{format::Justify, Table};
use std::cmp::Ordering;

#[derive(Debug, Table, PartialEq, Clone)]
pub struct AthleteSummary {
    #[table(title = "Name", justify = "Justify::Left")]
    full_name: String,
    #[table(title = "Code", justify = "Justify::Left")]
    pub team: String,
    #[table(title = "Altitude (m.a.s.l)", justify = "Justify::Right")]
    altitude: usize,
    #[table(title = "Speed (km/h)", justify = "Justify::Right")]
    speed: usize,
    #[table(title = "Distance from Target (km)", justify = "Justify::Right")]
    pub distance: f64,
    #[table(title = "Rank", justify = "Justify::Left")]
    rank: usize,
    #[table(title = "Status", justify = "Justify::Left")]
    status: AthleteStatus,
    #[table(title = "Last Update", justify = "Justify::Left")]
    last_update: String,
}

impl AthleteSummary {
    pub async fn request_minimized(
        overview: Overview,
    ) -> Result<Vec<AthleteSummary>, Box<dyn std::error::Error>> {
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

            let timestamp = Utc
                .timestamp_opt((current_status.timestamp / 1000) as i64, 0)
                .single()
                .unwrap();
            let duration = chrono::Utc::now().signed_duration_since(timestamp);
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
                last_update: format!(
                    "{}min and {}s ago",
                    duration.num_minutes(),
                    duration.num_seconds() % 60
                ),
            })
        }

        stati.sort_by_key(|athlete_summary| athlete_summary.rank);
        Ok(stati)
    }
    pub async fn request() -> Result<Vec<AthleteSummary>, Box<dyn std::error::Error>> {
        let overview = Overview::request().await?;
        let stati = AthleteSummary::request_minimized(overview).await?;
        Ok(stati)
    }
    pub fn mark(&mut self, kind: Change) {
        match kind {
            Change::RankUp => self.full_name = format!("{} ▲ ", self.full_name),
            Change::RankDown => self.full_name = format!("{} ▼ ", self.full_name),
            Change::TendencyUp => self.full_name = format!("{} △", self.full_name),
            Change::TendencyDown => self.full_name = format!("{} ▽", self.full_name),
            Change::NoChange => {}
        }
    }
    pub fn mark_if_changed(&mut self, other: &AthleteSummary, tendency: Option<Ordering>) {
        match self.rank.cmp(&other.rank) {
            Ordering::Greater => self.mark(Change::RankUp),
            Ordering::Less => self.mark(Change::RankDown),
            _ => match tendency {
                Some(Ordering::Greater) => self.mark(Change::TendencyUp),
                Some(Ordering::Less) => self.mark(Change::TendencyDown),
                _ => self.mark(Change::NoChange),
            },
        }
    }
}

// impl PartialEq for Vec<AthleteSummary> {
//     fn eq(&self, other: &Self) -> bool {
//         self.iter().zip(other.iter()).all(|(a, b)| a == b)
//     }
// }

pub enum Change {
    RankUp,
    RankDown,
    TendencyUp,
    TendencyDown,
    NoChange,
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
