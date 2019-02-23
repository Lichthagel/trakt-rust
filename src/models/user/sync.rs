use crate::models::comment::GetComments;
use crate::models::CollectionMovie;
use crate::models::CollectionShow;
use crate::models::Comment;
use crate::models::CommentAndItem;
use crate::models::FullUser;
use crate::models::List;
use crate::models::ToId;
use crate::models::User;
use crate::sync::pagination::PaginationRequest;
use crate::Result;
use crate::TraktApi;

pub trait UserMethods<'a>: ToId<&'a str> {
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
        client: &TraktApi,
        access_token: Option<&str>,
        f: impl FnOnce(GetComments) -> GetComments,
    ) -> PaginationRequest<CommentAndItem> {
        client.user_comments(self.id(), f, access_token)
    }
}
