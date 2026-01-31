use hyper::{client::HttpConnector, Client, Request, Body, Response};
use crate::constants::Constants;

pub struct HttpProxyClient {
    client: Client<HttpConnector>,
}

impl Default for HttpProxyClient {
    fn default() -> Self {
        let mut http = HttpConnector::new();
        http.set_nodelay(true);
        http.set_keepalive(Some(std::time::Duration::from_secs(Constants::DEFAULT_POOL_IDLE_TIMEOUT)));

        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(Constants::DEFAULT_POOL_IDLE_TIMEOUT))
            .pool_max_idle_per_host(Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST)
            .build(http);

        Self { client }
    }
}

// todo: implement constructor that can be configured with config

impl HttpProxyClient {
    pub async fn request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        self.client.request(req).await
    }
}

