use crate::models::CollectionRemoveResponse;
use crate::models::HistoryItem;
use crate::models::ItemType;
use crate::models::MediaType;
use crate::models::WatchedEntry;
use crate::{
    error::Result,
    models::{
        CollectionAddResponse, CollectionMovie, CollectionRequest, CollectionShow, LastActivities,
        Playback, WatchableType,
    },
    TraktApi,
};
use chrono::DateTime;
use chrono::Utc;

impl TraktApi {
    pub fn sync_last_activities(&self, access_token: String) -> Result<LastActivities> {
        self.auth_get(api_url!(("sync", "last_activities")), access_token)
    }

    pub fn sync_playback(
        &self,
        item_type: WatchableType,
        access_token: String,
    ) -> Result<Vec<Playback>> {
        self.auth_get(api_url!(("sync", "playback", item_type)), access_token)
    }

    pub fn sync_playback_delete(&self, playback_id: u64, access_token: String) -> Result<()> {
        self.auth_delete(api_url!(("sync", "playback", playback_id)), access_token)
    }

    pub fn sync_collection_movie(&self, access_token: String) -> Result<Vec<CollectionMovie>> {
        self.auth_get(api_url!(("sync", "collection", "movies")), access_token)
    }

    pub fn sync_collection_show(&self, access_token: String) -> Result<Vec<CollectionShow>> {
        self.auth_get(api_url!(("sync", "collection", "shows")), access_token)
    }

    pub fn sync_collection_add(
        &self,
        f: impl Fn(&mut CollectionRequest),
        access_token: String,
    ) -> Result<CollectionAddResponse> {
        let mut req = CollectionRequest::new();

        f(&mut req);

        let req = req.build();

        self.auth_post(
            api_url!(("sync", "collection")),
            serde_json::to_string(&req)?,
            access_token,
        )
    }

    pub fn sync_collection_remove(
        &self,
        f: impl Fn(&mut CollectionRequest),
        access_token: String,
    ) -> Result<CollectionRemoveResponse> {
        let mut req = CollectionRequest::new();

        f(&mut req);

        let req = req.build();

        self.auth_post(
            api_url!(("sync", "collection", "remove")),
            serde_json::to_string(&req)?,
            access_token,
        )
    }

    pub fn sync_watched(
        &self,
        item_type: MediaType,
        access_token: String,
    ) -> Result<Vec<WatchedEntry>> {
        self.auth_get(api_url!(("sync", "watched", item_type)), access_token)
    }

    pub fn sync_history(
        &self,
        item_type: ItemType,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        access_token: String,
    ) -> Result<Vec<HistoryItem>> {
        self.auth_get(
            api_url!(
                ("sync", "history", item_type),
                ("start_at", start_at),
                ("end_at", end_at)
            ),
            access_token,
        )
    }
}
