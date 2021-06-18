use warp::filters::{body, path};
use warp::Filter;

use super::with_db;
use crate::{data, db, handler};

pub fn people_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let people = warp::path("api")
        .and(warp::path("people"))
        .and(with_db(client));

    // Create Routes
    people
        .clone()
        .and(create_route())
        .and_then(handler::people::create)
        .or(people
            .clone()
            .and(read_all_route())
            .and_then(handler::people::read_all))
        .or(people
            .clone()
            .and(read_single_route())
            .and_then(handler::people::read_single))
}

fn create_route() -> impl Filter<Extract = (data::PersonRequest,), Error = warp::Rejection> + Copy {
    warp::post().and(person_request())
}

fn read_all_route() -> impl Filter<Extract = (), Error = warp::Rejection> + Copy {
    warp::get().and(path::end())
}

fn read_single_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::get()
        .and(warp::path::param::<String>())
        .and(path::end())
}

#[allow(dead_code)] //TODO!: Remove this after implementing handler
fn update_route(
) -> impl Filter<Extract = (String, data::PersonRequest), Error = warp::Rejection> + Copy {
    warp::put()
        .and(warp::path::param::<String>())
        .and(person_request())
        .and(path::end())
}

#[allow(dead_code)] //TODO!: Remove this after implementing handler
fn delete_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::delete()
        .and(warp::path::param::<String>())
        .and(path::end())
}

fn person_request() -> impl Filter<Extract = (data::PersonRequest,), Error = warp::Rejection> + Copy
{
    body::content_length_limit(4096).and(body::json())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data::PersonRequest;
    use warp::test;

    #[tokio::test]
    async fn test_create() {
        let new_person = PersonRequest {
            fname: "Chicken".to_string(),
            lname: "Little".to_string(),
        };
        let filter = create_route();
        let reply = test::request()
            .path("/")
            .method("POST")
            .json(&new_person)
            .filter(&filter)
            .await
            .unwrap();
        assert_eq!(reply, new_person);
    }

    #[tokio::test]
    async fn test_read_empty() {
        // an empty read request should result in an empty string
        let filter = read_all_route();
        let value = test::request().path("/").filter(&filter).await.unwrap();
    }

    #[tokio::test]
    async fn test_read_with_name() {
        //a read route with only one path should return the path string
        let filter = read_single_route();
        let value = test::request()
            .path("/FirstName")
            .filter(&filter)
            .await
            .unwrap();
        assert_eq!(value, "FirstName");
    }

    #[tokio::test]
    async fn test_update() {
        let filter = update_route();

        let new_person = PersonRequest {
            fname: "Chicken".to_string(),
            lname: "Little".to_string(),
        };

        let value = test::request()
            .method("PUT")
            .path(&format!("/{}", &new_person.fname))
            .json(&new_person)
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(value.0, new_person.fname);
        assert_eq!(value.1, new_person);
    }

    #[tokio::test]
    async fn test_delete() {
        let filter = delete_route();

        let value = test::request()
            .method("DELETE")
            .path("/Chicken")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(value, "Chicken")
    }
}
