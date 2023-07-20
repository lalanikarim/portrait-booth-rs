use std::collections::HashMap;

use leptos::ServerFnError;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use crate::models::order_item::Mode;
use crate::to_server_fn_error;

pub fn get_prefix(order_id: u64, mode: Mode) -> String {
    let prefix = format!("/{:0>6}/{:?}", order_id, mode).to_lowercase();
    prefix
}

pub async fn get_bucket() -> Result<Bucket, ServerFnError> {
    let bucket_name = dotenvy::var("S3_BUCKET_NAME").expect("should be present");
    let endpoint = dotenvy::var("S3_ENDPOINT").expect("should be present");
    let region = dotenvy::var("S3_REGION").expect("should be present");
    let region = Region::Custom { region, endpoint };
    let access_key = dotenvy::var("S3_ACCESS_KEY").expect("should be present");
    let secret_key = dotenvy::var("S3_SECRET_KEY").expect("should be present");
    let credentials = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)
        .expect("should work");
    let bucket = Bucket::new(&bucket_name, region, credentials);
    bucket.map_err(|e| ServerFnError::ServerError(e.to_string()))
}

pub async fn get_files(prefix: String) -> Result<Vec<String>, ServerFnError> {
    let bucket = get_bucket().await?;
    bucket
        .list(prefix, Some("/".into()))
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
        .map(|r| {
            r.iter()
                .flat_map(|i| {
                    let mut common_prefixes: Vec<String> = i
                        .common_prefixes
                        .clone()
                        .map(|p| p.iter().map(|p| p.prefix.clone()).collect())
                        .unwrap_or(Vec::new());
                    let mut content: Vec<String> =
                        i.contents.iter().map(|c| c.key.clone()).collect();
                    common_prefixes.append(&mut content);
                    common_prefixes
                })
                .collect()
        })
}

pub async fn delete_file(path: String) -> Result<bool, ServerFnError> {
    leptos::log!("Deleting {path:?}");
    let bucket = get_bucket().await?;
    bucket
        .delete_object(path)
        .await
        .map_err(to_server_fn_error)
        .map(|response| {
            leptos::log!("{:#?}", response);
            true
        })
}

pub async fn create_presigned_url(
    prefix: String,
    file_name: String,
    mime_type: String,
) -> Result<String, ServerFnError> {
    let path = format!("{prefix}/{file_name}");
    let mut get_queries = HashMap::new();
    get_queries.insert(
        "response-content-disposition".into(),
        format!("attachment; filename=\"{file_name}\""),
    );
    get_queries.insert("content-type".into(), mime_type.clone());
    let bucket = get_bucket().await?;
    bucket
        .presign_get(path, 604800, Some(get_queries))
        .map_err(to_server_fn_error)
}
pub async fn create_presigned_put_url(path: String) -> Result<String, ServerFnError> {
    let bucket = get_bucket().await?;
    bucket
        .presign_put(path, 300, None)
        .map_err(to_server_fn_error)
}

pub async fn create_presigned_url_pair(
    prefix: String,
    file_name: String,
    mime_type: String,
) -> Result<(String, String), ServerFnError> {
    let path = format!("{prefix}/{file_name}");

    let mut get_queries = HashMap::new();
    get_queries.insert(
        "response-content-disposition".into(),
        format!("attachment; filename=\"{file_name}\""),
    );
    get_queries.insert("content-type".into(), mime_type.clone());
    let bucket = get_bucket().await?;
    bucket
        .presign_put(path.clone(), 3600, None)
        .and_then(|put_url| {
            bucket
                .presign_get(path, 604800, Some(get_queries))
                .map(|get_url| (get_url, put_url))
        })
        .map_err(to_server_fn_error)
}
