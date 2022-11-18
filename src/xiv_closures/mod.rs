use std::collections::HashMap;

#[derive(Default)]
struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(Default)]
struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}
type BoxedCallback = Box<dyn Fn(&Request) -> Response>;
struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    /// Create an empty router.
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router.
    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        fn not_found_response() -> Response {
            Response {
                code: 400,
                ..Default::default()
            }
        }
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}
#[cfg(test)]
mod test {
    use super::*;

    fn get_form_response() -> Response {
        Response {
            code: 302,
            body: "redirect to login".into(),
            ..Default::default()
        }
    }
    #[test]
    fn test_router() {
        let mut router = BasicRouter::new();
        router.add_route("/", |_| get_form_response());
        router.add_route("/gdc", |_| Response {
            code: 200,
            body: "gdc response".into(),
            ..Default::default()
        });

        assert_eq!(
            router
                .handle_request(&Request {
                    url: "/".to_string(),
                    ..Default::default()
                })
                .code,
            302
        );
        assert_eq!(
            router
                .handle_request(&Request {
                    url: "/gdc".to_string(),
                    ..Default::default()
                })
                .code,
            200
        )
    }
    #[test]
    fn test_sorting_cities() {
        let mut cities = vec![
            City {
                name: "LIS".to_string(),
                population: 504_718i64,
                country: "Portugal".to_string(),
            },
            City {
                name: "BCN".to_string(),
                population: 1_680_000i64,
                country: "Spain".to_string(),
            },
            City {
                name: "NY".to_string(),
                population: 5_468_000i64,
                country: "US".to_string(),
            },
        ];

        sort_cities(&mut cities);

        println!("{:#?}", cities);

        assert_eq!(cities[0].name, "NY".to_string());
    }
}
