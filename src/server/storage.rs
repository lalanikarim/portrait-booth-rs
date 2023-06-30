use std::collections::HashMap;

use leptos::ServerFnError;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

use crate::to_server_fn_error;

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
    match get_bucket().await {
        Err(e) => Err(e),
        Ok(bucket) => bucket
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
            }),
    }
}

pub async fn create_presigned_url(path: String) -> Result<String, ServerFnError> {
    match get_bucket().await {
        Err(e) => Err(e),
        Ok(bucket) => bucket
            .presign_get(path, 300, None)
            .map_err(to_server_fn_error),
    }
}
pub async fn create_presigned_put_url(path: String) -> Result<String, ServerFnError> {
    match get_bucket().await {
        Err(e) => Err(e),
        Ok(bucket) => bucket
            .presign_put(path, 300, None)
            .map_err(to_server_fn_error),
    }
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
    match get_bucket().await {
        Err(e) => Err(e),
        Ok(bucket) => match bucket.presign_put(path.clone(), 3600, None) {
            Err(e) => Err(to_server_fn_error(e)),
            Ok(put_url) => match bucket.presign_get(path, 604800, Some(get_queries)) {
                Err(e) => Err(to_server_fn_error(e)),
                Ok(get_url) => Ok((get_url, put_url)),
            },
        },
    }
}
