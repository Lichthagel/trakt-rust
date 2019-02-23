use crate::{
    models::{
        comment::GetComments, CollectionMovie, CollectionShow, CommentAndItem, FullUser, ToId, User,
    },
    sync::pagination::PaginationRequest,
    Result, TraktApi,
};

pub trait UserMethods<'a>: ToId<'a, &'a str> {
    fn fetch(&'a self, client: &TraktApi, access_token: Option<&str>) -> Result<User> {
        client.user_profile(self.id(), access_token)
    }

    fn fetch_full(&'a self, client: &TraktApi, access_token: Option<&str>) -> Result<FullUser> {
        client.user_profile_full(self.id(), access_token)
    }

    fn collection_movies(
        &'a self,
        client: &TraktApi,
        access_token: Option<&str>,
    ) -> Result<Vec<CollectionMovie>> {
        client.user_collection_movies(self.id(), access_token)
    }

    fn collection_shows(
        &'a self,
        client: &TraktApi,
        access_token: Option<&str>,
    ) -> Result<Vec<CollectionShow>> {
        client.user_collection_shows(self.id(), access_token)
    }

    fn comments(
        &'a self,
        client: &'a TraktApi,
        access_token: Option<&str>,
        f: impl FnOnce(GetComments) -> GetComments,
    ) -> PaginationRequest<CommentAndItem> {
        client.user_comments(self.id(), f, access_token)
    }
}

impl<'a> UserMethods<'a> for User {}
impl<'a> UserMethods<'a> for FullUser {}
