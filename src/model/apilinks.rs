use serde::Deserialize;

use super::Links;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ApiLinks {
    #[serde(rename = "Manifest")]
    manifest: Manifest,
}

impl ApiLinks {
    pub fn verify(&self) -> bool {
        let (res, msg) = self
            .manifest
            .links
            .verify();

        println!(
            "|{}| {msg}",
            match res {
                true => '✅',
                false => '❌',
            }
        );

        res
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct Manifest {
    #[serde(flatten)]
    links: Links,
}