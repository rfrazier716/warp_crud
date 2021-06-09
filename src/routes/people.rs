use warp::filters::{body, path};
use warp::{Filter};

use super::with_db;
use crate::{data, db, handler::people};

pub fn people_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let people = warp::path("api")
        .and(warp::path("people"));

    // Create Routes
    people
        .and(with_db(client.clone()))
        .and(create_route()).and_then(people::create)
        .or(
            people
                .and(with_db(client.clone()))
                .and(read_route()).and_then(people::read)
        )
}

fn read_route() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::get()
        .and(
            path::param::<String>()
                .or(warp::any().map(|| String::from("")))
                .unify(),
        )
        .and(path::end())
}

fn create_route() -> impl Filter<Extract = (data::PersonRequest,), Error = warp::Rejection> + Copy {
    body::content_length_limit(4096).and(body::json())
}

#[cfg(test)]
mod test {
    use super::*;

    mod create {
        use super::create_route;
        use crate::data::PersonRequest;
        use warp::test;

        #[tokio::test]
        async fn test_create_person() {
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
    }

    mod read {
        use super::read_route;
        use warp::test;

        #[tokio::test]
        async fn test_empty_read() {
            // an empty read request should result in an empty string
            let filter = read_route();
            let value = test::request().path("/").filter(&filter).await.unwrap();
            assert_eq!(value, "")
        }

        #[tokio::test]
        async fn test_read_with_name() {
            //a read route with only one path should return the path string
            let filter = read_route();
            let value = test::request()
                .path("/FirstName")
                .filter(&filter)
                .await
                .unwrap();
            assert_eq!(value, "FirstName");
        }

        #[tokio::test]
        async fn test_read_with_multiple_paths() {
            //a read with multiple paths should not match
            let filter = read_route();
            let value = test::request()
                .path("/FirstName/LastName")
                .filter(&filter)
                .await;
            assert!(value.is_err(), "Path Should not Match");
        }

        #[tokio::test]
        async fn test_filter_only_matches_read_request() {
            let filter = read_route();
            let value = warp::test::request()
                .method("POST")
                .path("/FirstName")
                .filter(&filter)
                .await;
            assert!(value.is_err(), "Read Filter Matched a Post Request");
        }
    }
}
