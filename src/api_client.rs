use hyper::header::Headers;
use reqwest;
use serde::de::DeserializeOwned;

use index;

header! { (XAlgoliaApplicationId, "X-Algolia-Application-Id") => [String] }
header! { (XAlgoliaAPIKey, "X-Algolia-API-Key") => [String] }

pub struct ApiClient {
    app_id: String,
    api_key: String,
}

impl ApiClient {
    fn headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(XAlgoliaApplicationId(self.app_id.clone()));
        headers.set(XAlgoliaAPIKey(self.api_key.clone()));

        headers
    }

    fn list_indices(&self) -> reqwest::Result<index::Indices> {
        self.request(String::from("/1/indexes"))
    }

    fn request<T: DeserializeOwned>(&self, url: String) -> reqwest::Result<T> {
        let client = reqwest::Client::new().map(|mut c| { c.gzip(true); c });
        let final_url = format!("https://{}.algolia.net{}", self.app_id.clone(), url);

        client
            .and_then(|c| c.get(&final_url).headers(self.headers()).send())
            .and_then(|mut i| i.json())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let client = super::ApiClient {
            app_id: String::from("XT7HFL89G3"),
            api_key: String::from("b58dd27a53f71be07d72cf013f21bc36"),
        };

        client.list_indices();
    }
}
