use crate::models::sync::RatingFactory;
use crate::models::ListItem;
use crate::pagination::PaginationFactory;
use crate::{
    error::Result,
    models::{
        AllItemType, CollectionMovie, CollectionShow, HistoryItem, ItemType, LastActivities,
        MediaType, Playback, Rating, SyncAddResponse, SyncFactory, SyncRemoveResponse, SyncType,
        WatchableType, WatchedEntry,
    },
    TraktApi,
};
use chrono::{DateTime, Utc};

impl TraktApi {
    pub fn sync_last_activities(&self, access_token: &str) -> Result<LastActivities> {
        self.auth_get(api_url!(("sync", "last_activities")), access_token)
    }

    pub fn sync_playback(
        &self,
        item_type: WatchableType,
        access_token: &str,
    ) -> Result<Vec<Playback>> {
        self.auth_get(api_url!(("sync", "playback", item_type)), access_token)
    }

    pub fn sync_playback_delete(&self, playback_id: u64, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("sync", "playback", playback_id)), access_token)
    }

    pub fn sync_collection_movie(&self, access_token: &str) -> Result<Vec<CollectionMovie>> {
        self.auth_get(api_url!(("sync", "collection", "movies")), access_token)
    }

    pub fn sync_collection_show(&self, access_token: &str) -> Result<Vec<CollectionShow>> {
        self.auth_get(api_url!(("sync", "collection", "shows")), access_token)
    }

    pub fn sync_collection_add(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncAddResponse> {
        let body = f(SyncFactory::new(SyncType::Collect)).build();

        self.auth_post(
            api_url!(("sync", "collection")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_collection_remove(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncRemoveResponse> {
        let body = f(SyncFactory::new(SyncType::Collect)).build();

        self.auth_post(
            api_url!(("sync", "collection", "remove")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_watched(
        &self,
        item_type: MediaType,
        access_token: &str,
    ) -> Result<Vec<WatchedEntry>> {
        self.auth_get(api_url!(("sync", "watched", item_type)), access_token)
    }

    pub fn sync_history(
        &self,
        item_type: ItemType,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        access_token: &str,
    ) -> Result<Vec<HistoryItem>> {
        let pf = f(PaginationFactory::default());
        self.auth_get(
            api_url!(
                ("sync", "history", item_type),
                ("start_at", start_at),
                ("end_at", end_at),
                ("page", pf.page),
                ("limit", pf.limit)
            ),
            access_token,
        )
    }

    pub fn sync_history_add(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncAddResponse> {
        let body = f(SyncFactory::new(SyncType::Watch)).build();

        self.auth_post(
            api_url!(("sync", "history")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_history_remove(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncRemoveResponse> {
        let body = f(SyncFactory::new(SyncType::Watch)).build();

        self.auth_post(
            api_url!(("sync", "history", "remove")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_ratings(&self, item_type: AllItemType, access_token: &str) -> Result<Vec<Rating>> {
        self.auth_get(api_url!(("sync", "ratings", item_type)), access_token)
    }

    pub fn sync_ratings_add(
        &self,
        f: impl FnOnce(RatingFactory) -> RatingFactory,
        access_token: &str,
    ) -> Result<SyncAddResponse> {
        let body = f(RatingFactory::new()).build();

        self.auth_post(
            api_url!(("sync", "ratings")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_ratings_remove(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncRemoveResponse> {
        let body = f(SyncFactory::new(SyncType::Rating)).build();

        self.auth_post(
            api_url!(("sync", "ratings", "remove")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_watchlist(
        &self,
        item_type: Option<ItemType>,
        access_token: &str,
    ) -> Result<Vec<ListItem>> {
        self.auth_get(
            match item_type {
                Some(t) => api_url!(("sync", "watchlist", t)),
                None => api_url!(("sync", "watchlist")),
            },
            access_token,
        )
    }

    pub fn sync_watchlist_add(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncAddResponse> {
        let body = f(SyncFactory::new(SyncType::Watchlist)).build();

        self.auth_post(
            api_url!(("sync", "watchlist")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }

    pub fn sync_watchlist_remove(
        &self,
        f: impl FnOnce(SyncFactory) -> SyncFactory,
        access_token: &str,
    ) -> Result<SyncRemoveResponse> {
        let body = f(SyncFactory::new(SyncType::Watchlist)).build();

        self.auth_post(
            api_url!(("sync", "watchlist", "remove")),
            serde_json::to_string(&body)?,
            access_token,
        )
    }
}
