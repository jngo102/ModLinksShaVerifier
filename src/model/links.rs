use serde::Deserialize;

use super::FileDef;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Links {
    #[serde(rename = "Link")]
    Universal(FileDef),
    #[serde(rename = "Links")]
    PlatformDependent {
        #[serde(rename = "Windows")]
        windows: FileDef,
        #[serde(rename = "Mac")]
        mac: FileDef,
        #[serde(rename = "Linux")]
        linux: FileDef,
    },
}

impl Links {
    pub fn verify(&self) -> (bool, String) {
        match self {
            Self::Universal(def) => def.verify(),
            Self::PlatformDependent {
                windows,
                mac,
                linux,
            } => {
                let mut res_windows = None;
                let mut res_mac = None;
                let mut res_linux = None;
                rayon::scope(|s| {
                    s.spawn(|_| res_windows = Some(windows.verify()));
                    s.spawn(|_| res_mac = Some(mac.verify()));
                    s.spawn(|_| res_linux = Some(linux.verify()));
                });

                let (r_windows, r_mac, r_linux) =
                    (res_windows.unwrap(), res_mac.unwrap(), res_linux.unwrap());

                (
                    r_windows.0 && r_mac.0 && r_linux.0,
                    format!(
                        "\n  Windows:  {}\n  Mac:      {}\n  Linux:    {}",
                        r_windows.1, r_mac.1, r_linux.1,
                    ),
                )
            }
        }
    }
}
