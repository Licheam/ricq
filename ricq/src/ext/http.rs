use alhc::ClientExt;
use bytes::Bytes;

use ricq_core::{RQError, RQResult};

pub async fn http_get<S0, S1>(url: S0, cookie: S1) -> RQResult<Bytes>
where
    S0: AsRef<str>,
    S1: AsRef<str>,
{
    let client = alhc::ClientBuilder::default().build();
    let req = client.get(url.as_ref());
    if req.is_err() {
        return Err(RQError::Other("url".into()));
    }
    let mut req = req
        .unwrap()
        .header("User-Agent", "QQ/8.2.0.1296 CFNetwork/1126")
        .header("Net-Type", "Wifi");
    if !cookie.as_ref().is_empty() {
        req = req.header("Cookie", cookie.as_ref());
    }
    let resp = req.await;
    match resp {
        Ok(resp) => {
            // todo gzip
            match resp.recv().await {
                Ok(body) => Ok(Bytes::from(body.data().to_owned())),
                Err(err) => Err(RQError::IO(err)),
            }
        }
        Err(err) => Err(RQError::IO(err)),
    }
}
