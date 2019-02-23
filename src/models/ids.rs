#[cfg(feature = "async")]
use crate::asyn::{Result as AsyncResult, TraktApi as AsyncTraktApi};
use crate::error::Error;
use crate::models::{SearchItemType, SearchResult};
#[cfg(feature = "sync")]
use crate::{Result, TraktApi};
use std::fmt;
use std::fmt::Display;
use std::ops::AddAssign;

pub trait ToId<T> {
    fn id(&self) -> T;
}

/// [Ids] of almost every item
///
/// [Ids]: https://trakt.docs.apiary.io/#introduction/standard-media-objects
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Ids {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trakt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvrage: Option<u64>,
}

#[cfg(feature = "sync")]
impl Ids {
    pub fn lookup_trakt(
        &self,
        client: &TraktApi,
        item_type: Option<SearchItemType>,
    ) -> Result<Vec<SearchResult>> {
        match self.trakt {
            Some(id) => client.id_lookup(IdType::Trakt, id, item_type),
            None => Err(Error::NoneError),
        }
    }

    pub fn lookup_imdb(
        &self,
        client: &TraktApi,
        item_type: Option<SearchItemType>,
    ) -> Result<Vec<SearchResult>> {
        match &self.imdb {
            Some(id) => client.id_lookup(IdType::IMDb, id, item_type),
            None => Err(Error::NoneError),
        }
    }

    pub fn lookup_tmdb(
        &self,
        client: &TraktApi,
        item_type: Option<SearchItemType>,
    ) -> Result<Vec<SearchResult>> {
        match self.tmdb {
            Some(id) => client.id_lookup(IdType::TMDb, id, item_type),
            None => Err(Error::NoneError),
        }
    }

    pub fn lookup_tvdb(
        &self,
        client: &TraktApi,
        item_type: Option<SearchItemType>,
    ) -> Result<Vec<SearchResult>> {
        match self.tvdb {
            Some(id) => client.id_lookup(IdType::TVDB, id, item_type),
            None => Err(Error::NoneError),
        }
    }
}

#[cfg(feature = "async")]
impl Ids {
    pub fn lookup_trakt_async(
        &self,
        client: &AsyncTraktApi,
        item_type: Option<SearchItemType>,
    ) -> AsyncResult<Vec<SearchResult>> {
        match self.trakt {
            Some(id) => client.id_lookup(IdType::Trakt, id, item_type),
            None => Box::new(futures::future::err(Error::NoneError)),
        }
    }

    pub fn lookup_imdb_async(
        &self,
        client: &AsyncTraktApi,
        item_type: Option<SearchItemType>,
    ) -> AsyncResult<Vec<SearchResult>> {
        match &self.imdb {
            Some(id) => client.id_lookup(IdType::IMDb, id, item_type),
            None => Box::new(futures::future::err(Error::NoneError)),
        }
    }

    pub fn lookup_tmdb_async(
        &self,
        client: &AsyncTraktApi,
        item_type: Option<SearchItemType>,
    ) -> AsyncResult<Vec<SearchResult>> {
        match self.tmdb {
            Some(id) => client.id_lookup(IdType::TMDb, id, item_type),
            None => Box::new(futures::future::err(Error::NoneError)),
        }
    }

    pub fn lookup_tvdb_async(
        &self,
        client: &AsyncTraktApi,
        item_type: Option<SearchItemType>,
    ) -> AsyncResult<Vec<SearchResult>> {
        match self.tvdb {
            Some(id) => client.id_lookup(IdType::TVDB, id, item_type),
            None => Box::new(futures::future::err(Error::NoneError)),
        }
    }
}

impl Default for Ids {
    fn default() -> Self {
        Self {
            trakt: None,
            slug: None,
            tvdb: None,
            imdb: None,
            tmdb: None,
            tvrage: None,
        }
    }
}

impl AddAssign<Ids> for &mut Ids {
    fn add_assign(&mut self, rhs: Ids) {
        self.trakt = self.trakt.or(rhs.trakt);
        self.slug = self.slug.clone().or(rhs.slug);
        self.tvdb = self.tvdb.or(rhs.tvdb);
        self.imdb = self.imdb.clone().or(rhs.imdb);
        self.tmdb = self.tmdb.or(rhs.tmdb);
        self.tvrage = self.tvrage.or(rhs.tvrage);
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdType {
    Trakt,
    IMDb,
    TMDb,
    TVDB,
}

impl Display for IdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            IdType::Trakt => "trakt",
            IdType::IMDb => "imdb",
            IdType::TMDb => "tmdb",
            IdType::TVDB => "tvdb",
        })
    }
}
