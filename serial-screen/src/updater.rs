use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::Deserialize;

#[cfg(any(target_arch = "x86_64"))]
pub const PLATFORM: &str = "x86_64";

#[cfg(any(target_arch = "arm"))]
pub const PLATFORM: &str = "arm-gnueabihf";

#[cfg(any(target_arch = "aarch64"))]
pub const PLATFORM: &str = "aarch64";

#[derive(Debug, Clone, Deserialize)]
struct GithubReleasesRoot {
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Debug, Clone, Deserialize)]
struct GithubReleaseAsset {
    name: String,
    size: u64,
    browser_download_url: String,
}

const GITHUB_RELEASES_URL: &str =
    "https://api.github.com/repos/filipton/dgus-moonraker-screen/releases/latest";

pub async fn check_for_updates() {
    let file_exists = tokio::fs::try_exists("/opt/serial-screen/serial-screen").await;
    if file_exists.is_err() || file_exists.unwrap() == false {
        println!("\x1b[93mNot checking for updates, not installed through setup script!\x1b[0m");
        return;
    }

    let client = reqwest::Client::new();
    let current_file_size = tokio::fs::metadata("/opt/serial-screen/serial-screen")
        .await
        .unwrap()
        .len();

    let headers = reqwest::header::HeaderMap::from_iter(vec![(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("dgus-moonraker-screen"),
    )]);

    tokio::task::spawn(async move {
        let resp = client
            .get(GITHUB_RELEASES_URL)
            .headers(headers.clone())
            .send()
            .await;

        if let Ok(resp) = resp {
            let json = resp.json::<GithubReleasesRoot>().await;
            if let Ok(releases) = json {
                let current_arch_release = releases
                    .assets
                    .iter()
                    .find(|asset| asset.name.contains(PLATFORM));

                if let Some(current_arch_release) = current_arch_release {
                    if current_arch_release.size != current_file_size {
                        let res = update(&client, headers, &current_arch_release.browser_download_url)
                            .await;

                        println!("Failed to update: {:?}", res);
                    }
                }
            }
        } else {
            println!("Failed to check for updates: {:?}", resp);
        }

        tokio::time::sleep(std::time::Duration::from_secs(60 * 5)).await;
    });
}

async fn update(client: &reqwest::Client, headers: HeaderMap, url: &str) -> Result<()> {
    let bytes = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;

    println!("Updating to latest version...");

    tokio::fs::write("/opt/serial-screen/serial-screen-update", bytes).await?;
    std::process::Command::new("bash")
        .arg("-c")
        .arg("'(sleep 1 ; mv /opt/serial-screen/serial-screen-update /opt/serial-screen/serial-screen ; chmod +x /opt/serial-screen/serial-screen) &'")
        .spawn()?;

    std::process::exit(0);
}
