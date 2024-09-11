use std::sync::Arc;
use crate::target_runtime::cache::HttpCacheManager;
use crate::target_runtime::http::NativeHttp;

#[derive(Clone)]
pub struct TargetRuntime {
    /// HTTP client for making standard HTTP requests.
    pub http: Arc<NativeHttp>,
    pub cache: Arc<HttpCacheManager>,
}

mod cache {
    use http_cache_reqwest::{CacheManager, HttpResponse};
    use http_cache_semantics::CachePolicy;
    use serde::{Deserialize, Serialize};
    pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
    pub type Result<T> = std::result::Result<T, BoxError>;
    use std::sync::Arc;

    use moka::future::Cache;
    use moka::policy::EvictionPolicy;

    pub struct HttpCacheManager {
        pub cache: Arc<Cache<String, Store>>,
    }

    impl Default for HttpCacheManager {
        fn default() -> Self {
            Self::new(42)
        }
    }

    #[derive(Clone, Deserialize, Serialize)]
    pub struct Store {
        response: HttpResponse,
        policy: CachePolicy,
    }

    impl HttpCacheManager {
        pub fn new(cache_size: u64) -> Self {
            let cache = Cache::builder()
                .eviction_policy(EvictionPolicy::lru())
                .max_capacity(cache_size)
                .build();
            Self { cache: Arc::new(cache) }
        }

        pub async fn clear(&self) -> Result<()> {
            self.cache.invalidate_all();
            self.cache.run_pending_tasks().await;
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl CacheManager for HttpCacheManager {
        async fn get(&self, cache_key: &str) -> Result<Option<(HttpResponse, CachePolicy)>> {
            let store: Store = match self.cache.get(cache_key).await {
                Some(d) => d,
                None => return Ok(None),
            };
            Ok(Some((store.response, store.policy)))
        }

        async fn put(
            &self,
            cache_key: String,
            response: HttpResponse,
            policy: CachePolicy,
        ) -> Result<HttpResponse> {
            let data = Store { response: response.clone(), policy };
            self.cache.insert(cache_key, data).await;
            self.cache.run_pending_tasks().await;
            Ok(response)
        }

        async fn delete(&self, cache_key: &str) -> Result<()> {
            self.cache.invalidate(cache_key).await;
            self.cache.run_pending_tasks().await;
            Ok(())
        }
    }
}

mod http {
    use http_cache_reqwest::{Cache, CacheMode, HttpCache, HttpCacheOptions};
    use reqwest::Client;
    use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
    use crate::blueprint::Upstream;
    use crate::cache::HttpCacheManager;

    pub struct NativeHttp {
        client: ClientWithMiddleware,
    }

    impl NativeHttp {
        pub fn init(upstream: &Upstream) -> Self {
            let mut client = ClientBuilder::new(Client::new());

            client = client.with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: HttpCacheManager::new(upstream.http_cache),
                options: HttpCacheOptions::default(),
            }));

            Self {
                client: client.build(),
            }
        }
    }
}