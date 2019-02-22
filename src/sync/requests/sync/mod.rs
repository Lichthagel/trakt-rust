pub mod sync_request;

use crate::{
    models::{
        AllItemType, CollectionMovie, CollectionShow, HistoryItem, ItemType, LastActivities,
        ListItem, MediaType, Playback, Rating, SyncAddResponse, SyncRemoveResponse, WatchableType,
        WatchedEntry,
    },
    sync::{pagination::PaginationRequest, requests::sync::sync_request::SyncRequest},
    Result, TraktApi,
};
use chrono::{DateTime, Utc};
use reqwest::Method;

impl<'a> TraktApi<'a> {
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

    pub fn sync_collection_add(&self) -> SyncRequest<SyncAddResponse> {
        SyncRequest::new(api_url!(("sync", "collection")), &self)
    }

    pub fn sync_collection_remove(&self) -> SyncRequest<SyncRemoveResponse> {
        SyncRequest::new(api_url!(("sync", "collection", "remove")), &self)
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
        access_token: &str,
    ) -> PaginationRequest<HistoryItem> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("sync", "history", item_type)))
                .bearer_auth(access_token)
                .query(&[("start_at", start_at), ("end_at", end_at)]),
        )
    }

    pub fn sync_history_add(&self) -> SyncRequest<SyncAddResponse> {
        SyncRequest::new(api_url!(("sync", "history")), &self)
    }

    pub fn sync_history_remove(&self) -> SyncRequest<SyncRemoveResponse> {
        SyncRequest::new(api_url!(("sync", "history", "remove")), &self)
    }

    pub fn sync_ratings(&self, item_type: AllItemType, access_token: &str) -> Result<Vec<Rating>> {
        self.auth_get(api_url!(("sync", "ratings", item_type)), access_token)
    }

    pub fn sync_ratings_add(&self) -> SyncRequest<SyncAddResponse> {
        SyncRequest::new(api_url!(("sync", "ratings")), &self)
    }

    pub fn sync_ratings_remove(&self) -> SyncRequest<SyncRemoveResponse> {
        SyncRequest::new(api_url!(("sync", "ratings", "remove")), &self)
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

    pub fn sync_watchlist_add(&self) -> SyncRequest<SyncAddResponse> {
        SyncRequest::new(api_url!(("sync", "watchlist")), &self)
    }

    pub fn sync_watchlist_remove(&self) -> SyncRequest<SyncRemoveResponse> {
        SyncRequest::new(api_url!(("sync", "watchlist", "remove")), &self)
    }
}
