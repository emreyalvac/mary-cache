#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};
use cache::cache_server::{Cache, CacheServer};
use cache::{CacheGetResponse, CacheGetRequest};
use lazy_static::lazy_static;
use prost_types::Any;
use store::Store;
use futures::executor::ThreadPool;
use prost_types::field::Kind;
use crate::cache::{CacheDeleteRequest, CacheDeleteResponse, CacheSetRequest, CacheSetResponse, GetAllCacheSetResponse};

mod store;

mod cache {
    include!("cache.rs");
}

// Create Store
lazy_static! {
    static ref STORE: Arc<Mutex<Store>> = Arc::new(Mutex::new(Store::new()));
}

pub struct CacheImpl {
    _pool: ThreadPool,
}

#[tonic::async_trait]
impl Cache for CacheImpl {
    async fn get_all_cache_set(&self, request: Request<()>) -> Result<Response<GetAllCacheSetResponse>, Status> {
        let store = STORE.lock().unwrap();
        Ok(Response::new(GetAllCacheSetResponse { length: store.cache_length() }))
    }

    async fn get(&self, request: Request<CacheGetRequest>) -> Result<Response<CacheGetResponse>, Status> {
        let req = request.get_ref().clone();
        let store_op = STORE.lock().unwrap().get(req.key);
        match store_op {
            Ok(value) => Ok(Response::new(CacheGetResponse { value: Some(value) })),
            // TODO: gRPC - Cache Error
            Err(_) => Err(Status::not_found("Not Found"))
        }
    }

    async fn set(&self, request: Request<CacheSetRequest>) -> Result<Response<CacheSetResponse>, Status> {
        println!("{:?}", request);
        let req = request.get_ref().clone();
        let store_op = STORE.lock().unwrap().set(req.key, req.value.unwrap());
        match store_op {
            Ok(_) => Ok(Response::new(CacheSetResponse { success: true })),
            Err(_) => Ok(Response::new(CacheSetResponse { success: false }))
        }
    }

    async fn delete(&self, request: Request<CacheDeleteRequest>) -> Result<Response<CacheDeleteResponse>, Status> {
        let req = request.get_ref().clone();
        let store_op = STORE.lock().unwrap().delete(req.key);
        match store_op {
            Ok(value) => Ok(Response::new(CacheDeleteResponse { value: Some(value), success: true })),
            Err(_) => Err(Status::internal("Cannot delete")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let thread_pool = ThreadPool::new()?;

    let addr = "0.0.0.0:3000".parse().unwrap();
    let cache = CacheImpl { _pool: thread_pool };

    println!("Cache server listening on {}", addr);

    Server::builder()
        .add_service(CacheServer::new(cache))
        .serve(addr)
        .await?;

    Ok(())
}