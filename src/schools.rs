use crate::{
    error::Error, jsonrpc, params::FindSchoolParams, resources::School, SchoolSearchResult,
};

fn get_client() -> jsonrpc::Client {
    jsonrpc::Client::new("https://mobile.webuntis.com/ms/schoolquery2")
}

/// Returns all schools matching the query or an empty vec if there are too many results.
pub async fn search(query: &str) -> Result<Vec<School>, Error> {
    let result = get_client()
        .request(
            "searchSchool",
            vec![FindSchoolParams::Search { search: query }],
        )
        .await;
    catch_too_many(result)
}

/// Retrieves a school by its id.
pub async fn get_by_id(id: &usize) -> Result<School, Error> {
    let result = get_client()
        .request(
            "searchSchool",
            vec![FindSchoolParams::ById { schoolid: id }],
        )
        .await;

    get_first(catch_too_many(result)?)
}

/// Retrieves a school by it's [`login_name`](School#structfield.login_name).
pub async fn get_by_name(name: &str) -> Result<School, Error> {
    let result = get_client()
        .request(
            "searchSchool",
            vec![FindSchoolParams::ByName { schoolname: name }],
        )
        .await;

    get_first(catch_too_many(result)?)
}

fn get_first(mut list: Vec<School>) -> Result<School, Error> {
    if list.is_empty() {
        Err(Error::NotFound)
    } else {
        Ok(list.swap_remove(0))
    }
}

fn catch_too_many(result: Result<SchoolSearchResult, Error>) -> Result<Vec<School>, Error> {
    match result {
        Ok(v) => Ok(v.schools),
        Err(Error::Rpc(err)) => {
            if err.code == jsonrpc::ErrorCode::TooManyResults.as_isize() {
                Ok(vec![])
            } else {
                Err(Error::Rpc(err))
            }
        }
        Err(err) => Err(err),
    }
}
