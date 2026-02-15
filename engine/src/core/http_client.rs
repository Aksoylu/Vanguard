use crate::constants::Constants;
use hyper::{client::HttpConnector, Body, Client, Request, Response};

pub struct HttpClient {
    client: Client<HttpConnector>,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::init(
            Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST,
        )
    }
}

impl HttpClient {
    pub fn init(pool_idle_timeout: u64, max_idle_conns_per_host: usize) -> Self {
        let mut http = HttpConnector::new();
        http.set_nodelay(true);
        http.set_keepalive(Some(std::time::Duration::from_secs(pool_idle_timeout)));

        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(pool_idle_timeout))
            .pool_max_idle_per_host(max_idle_conns_per_host)
            .build(http);

        Self { client }
    }

    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        self.client.request(req).await
    }
}
