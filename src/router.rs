use std::path::Path;

use salvo::{fs::NamedFile, handler, http::StatusCode, Depot, Request, Response, Router};
use tracing::info;

use crate::{
    error::{ServiceError, ServiceResult},
    opt::{Metadata, MetadataResp},
};

pub fn router() -> Router {
    Router::new()
        .push(Router::with_path("download/{name}").get(get_download))
        .push(Router::with_path("metadata").get(get_metadata))
}

#[handler]
async fn get_metadata(depot: &Depot) -> ServiceResult<MetadataResp> {
    info!("get metadata");
    let metadata = depot.obtain::<Vec<Metadata>>().unwrap().clone();
    Ok(MetadataResp(metadata))
}

#[handler]
async fn get_download(req: &mut Request, res: &mut Response) -> ServiceResult<()> {
    let name = get_req_path(req, "name")?;

    info!("download file: {}", name);
    let file_path = format!("assets/{}", name);
    if Path::new(&file_path).exists() {
        NamedFile::builder(&file_path)
            .attached_name(name)
            .send(req.headers(), res)
            .await;
        res.status_code(StatusCode::OK);
    } else {
        return Err(ServiceError::NotFound(format!("file {name} not found")));
    }
    Ok(())
}

pub fn get_req_path(req: &mut Request, key: &str) -> ServiceResult<String> {
    req.params()
        .get(key)
        .map(|v| v.to_string())
        .ok_or(ServiceError::InternalServerError(format!(
            "param {key} not found"
        )))
}
