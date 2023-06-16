use clearscreen;
use cli_table::{print_stdout, WithTitle};
use std::thread::sleep;
use std::time::{Duration, Instant};
use xalps::{summary::Change, AthleteSummary, Overview};

#[tokio::main]
async fn main() {
    let mut last_summary: Option<Vec<AthleteSummary>> = None;
    let overview = Overview::request().await.unwrap();

    loop {
        let start_time = Instant::now();
        let mut summary = AthleteSummary::request_minimized(overview.clone())
            .await
            .unwrap();
        // truncate to top 5
        summary.truncate(5);
        // skip if nothing changed
        if last_summary.is_some() && last_summary.as_ref().unwrap() == &summary {
            continue;
        }
        // mark changes
        let new_last_summary = summary.clone();
        if let Some(last_summary) = last_summary {
            let first = &summary[0];
            let last_top_distance = last_summary[0].distance - first.distance;
            let current_top_distance = last_summary[0].distance - first.distance;
            for row in summary.iter_mut() {
                if let Some(last_row) = last_summary
                    .iter()
                    .find(|last_row| last_row.team == row.team)
                {
                    row.mark_if_changed(
                        last_row,
                        (row.distance - current_top_distance)
                            .partial_cmp(&(last_row.distance - last_top_distance)),
                    );
                } else {
                    row.mark(Change::RankUp);
                }
            }
        }
        // update last summary
        last_summary = Some(new_last_summary);

        clearscreen::clear().unwrap_or(());
        print_stdout(summary.with_title()).unwrap();
        sleep(Duration::from_secs(5) - start_time.elapsed());
    }
}
