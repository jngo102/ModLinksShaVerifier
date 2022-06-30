use rayon::prelude::*;
use serde::Deserialize;

use super::Links;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ModLinks {
    #[serde(rename = "Manifest")]
    mods: Vec<Mod>,
}

impl ModLinks {
    pub fn verify(&self) -> bool {
        let (res, failed_mods) = self
            .mods
            .par_iter()
            .fold(
                || (true, vec![]),
                |mut a, m| {
                    if !m.verify() {
                        a.0 = false;
                        a.1.push(m.name.to_owned());
                    }

                    a
                },
            )
            .reduce(
                || (true, vec![]),
                |a, b| (a.0 && b.0, a.1.into_iter().chain(b.1.into_iter()).collect()),
            );

        if !res {
            eprintln!(
                "\n::error title=Error Summary::These mod(s) failed the verification: {}",
                failed_mods.join(", ")
            );
        }

        res
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Mod {
    #[serde(rename = "Name")]
    name: String,
    #[serde(flatten)]
    links: Links,
}

impl Mod {
    pub fn verify(&self) -> bool {
        let (res, msg) = self.links.verify();

        println!(
            "{:32} |{}| {msg}",
            self.name,
            match res {
                true => '✅',
                false => '❌',
            }
        );

        res
    }
}
