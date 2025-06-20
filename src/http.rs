use std::{
    fmt::{self, Formatter},
    io::Error,
    net::IpAddr,
    str::Utf8Error,
};

use rand::random;
use smoltcp::{phy::TunTapInterface, wire::EthernetAddress};
use url::Url;

#[derive(Debug)]
enum HttpState {
    Connect,
    Request,
    Response,
}

#[derive(Debug)]
pub enum UpstreamError {
    Network(Error),
    InvalidUrl,
    Content(Utf8Error),
}
impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<Utf8Error> for UpstreamError {
    fn from(value: Utf8Error) -> Self {
        UpstreamError::Content(value)
    }
}

fn random_port() -> u16 {
    49152 + random::<u16>() % 16384
}

pub fn get(
    tap: TunTapInterface,
    mac: EthernetAddress,
    addr: IpAddr,
    url: Url,
) -> Result<(), UpstreamError> {
    let domain_name = url.host().unwrap();
    Ok(())
}
