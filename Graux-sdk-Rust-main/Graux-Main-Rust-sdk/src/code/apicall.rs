mod graux_provider;
mod graux_websocket_provider;

pub struct GrauxConfig {
    api_key: String,
    network: Network,
    max_retries: u32,
    batch_requests: bool,
    url: Option<String>,
    auth_token: Option<String>,
    request_timeout: u32,
    base_graux_provider: Option<Box<dyn GrauxProvider>>,
    base_graux_wss_provider: Option<Box<dyn GrauxWebSocketProvider>>,
}

impl GrauxConfig {
    pub fn new(config: Option<GrauxSettings>) -> Self {
        let api_key = config.map_or_else(|| DEFAULT_GRAUX_API_KEY.to_string(), |c| c.api_key.unwrap_or_else(|| DEFAULT_GRAUX_API_KEY.to_string()));
        let network = config.map_or(DEFAULT_NETWORK, |c| c.network.unwrap_or(DEFAULT_NETWORK));
        let max_retries = config.map_or(DEFAULT_MAX_RETRIES, |c| c.max_retries.unwrap_or(DEFAULT_MAX_RETRIES));
        let batch_requests = config.map_or(false, |c| c.batch_requests.unwrap_or(false));
        let url = config.and_then(|c| c.url);
        let auth_token = config.and_then(|c| c.auth_token);
        let request_timeout = config.map_or(DEFAULT_REQUEST_TIMEOUT, |c| c.request_timeout.unwrap_or(DEFAULT_REQUEST_TIMEOUT));
        let base_graux_provider = None;
        let base_graux_wss_provider = None;
        
        GrauxConfig {
            api_key,
            network,
            max_retries,
            batch_requests,
            url,
            auth_token,
            request_timeout,
            base_graux_provider,
            base_graux_wss_provider,
        }
    }

    fn get_request_url(&self, api_type: GrauxApiType) -> String {
        if let Some(url) = &self.url {
            url.clone()
        } else if api_type == GrauxApiType::NFT {
            get_graux_nft_http_url(self.network, &self.api_key)
        } else if api_type == GrauxApiType::WEBHOOK {
            get_graux_webhook_http_url()
        } else {
            get_graux_http_url(self.network, &self.api_key)
        }
    }

    pub async fn get_provider(&mut self) -> Box<dyn GrauxProvider> {
        if let Some(provider) = &self.base_graux_provider {
            provider.clone()
        } else {
            let graux_provider = graux_provider::GrauxProvider::new(self).await;
            self.base_graux_provider = Some(Box::new(graux_provider.clone()));
            Box::new(graux_provider)
        }
    }

    pub async fn get_websocket_provider(&mut self) -> Box<dyn GrauxWebSocketProvider> {
        if let Some(provider) = &self.base_graux_wss_provider {
            provider.clone()
        } else {
            let graux_wss_provider = graux_websocket_provider::GrauxWebSocketProvider::new(self).await;
            self.base_graux_wss_provider = Some(Box::new(graux_wss_provider.clone()));
            Box::new(graux_wss_provider)
        }
    }
}

Please note that the GrauxProvider and GrauxWebSocketProvider structs and their implementations 
should be defined in separate files (graux_provider.rs and graux_websocket_provider.rs respectively).
The helper functions like get_graux_nft_http_url, get_graux_webhook_http_url, and 
get_graux_http_url should be implemented separately according to requirements.
