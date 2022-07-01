use serde::Deserialize;

use super::{Links, Verifiable};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ApiLinks {
    #[serde(rename = "Manifest")]
    manifest: Api,
}

impl Verifiable for ApiLinks {
    fn verify(&self) -> bool {
        self.manifest.verify()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct Api {
    #[serde(flatten)]
    links: Links,
}

impl Api {
    pub fn verify(&self) -> bool {
        let (res, msg) = self.links.verify();

        println!(
            "API |{}| {msg}",
            match res {
                true => '✅',
                false => '❌',
            }
        );

        res
    }
}
