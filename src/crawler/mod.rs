use super::status::ApiTrackResponse;
use ormlite::model::Model;

#[derive(Model)]
pub struct CrawledApiTrackResponse(ApiTrackResponse);

#[cfg(test)]
mod test{
    #[tokio:test]
    fn sqlite_works(){
        let mut conn = ormlite::SqliteConnection::connect(":memory:").await.unwrap();

    }
}