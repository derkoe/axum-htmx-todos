use std::str::FromStr;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{HeaderMap, StatusCode},
};

pub struct HxRequest(pub bool);

const HX_REQUEST: &str = "hx-request";

fn is_hx_request(headers: &HeaderMap) -> bool {
    headers
        .get(HX_REQUEST)
        .and_then(|hv| {
            let result: bool = FromStr::from_str(hv.to_str().unwrap())
                .or_else(|_| Ok::<bool, bool>(false))
                .unwrap();
            Some(result)
        })
        .or_else(|| Some(false))
        .unwrap()
}

#[async_trait]
impl<B> FromRequest<B> for HxRequest
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        Ok(HxRequest(is_hx_request(headers)))
    }
}
