//! Get your fresh, farm-to-table, single-origin ec2 Ubuntu AMIs.
//!
//! # Example
//! ```rust
//! use ubuntu_ami::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), StdError> {
//!     let res = get_latest(
//!         "us-east-1",
//!         Some("bionic"),
//!         None,
//!         Some("hvm:ebs-ssd"),
//!         Some("amd64"),
//!     )
//!     .await?;
//!     println!("us-east-1 ubuntu:bionic: {}", res);
//!     Ok(())
//! }
//! ```

static URL: &str = "https://cloud-images.ubuntu.com/locator/ec2/releasesTable?_=1588199609256";

pub type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Clone)]
struct Entry {
    region: String,
    release_name: String,
    release_number: String,
    architecture: String,
    instance_type: String,
    date: String,
    ami: String,
    hvm: String,
}

/// Get the most recent Ubuntu AMI that matches the given criteria.
pub async fn get_latest(
    region: &str,
    release_name: Option<&str>,
    release_number: Option<&str>,
    instance_type: Option<&str>,
    architecture: Option<&str>,
) -> Result<String, StdError> {
    let mut r = reqwest::get(URL).await?.text().await?;

    // get rid of the trailing comma
    let len = r.len();
    let (first, last) = r.split_at_mut(len - 10);
    let mut r = first.to_string();
    r.extend(last.replace(',', " ").chars());

    // parse to json
    let j: serde_json::Value = serde_json::from_str(&r)?;
    let amis = j
        .as_object()
        .ok_or_else(|| String::from("Value not a JSON object"))?
        .values()
        .next()
        .unwrap();

    let mut amis: Vec<Entry> = amis
        .as_array()
        .ok_or_else(|| String::from("Value not a JSON array"))?
        .into_iter()
        .map(|v| {
            let fs: Vec<&str> = v
                .as_array()
                .unwrap()
                .into_iter()
                .map(|s| s.as_str().unwrap())
                .collect();
            Entry {
                region: fs[0].to_string(),
                release_name: fs[1].to_string(),
                release_number: fs[2].to_string(),
                architecture: fs[3].to_string(),
                instance_type: fs[4].to_string(),
                date: fs[5].to_string(),
                ami: fs[6].to_string(),
                hvm: fs[7].to_string(),
            }
        })
        .filter(|e| e.region == region)
        .filter(|e| {
            if let Some(release_name) = release_name {
                e.release_name == release_name
            } else {
                true
            }
        })
        .filter(|e| {
            if let Some(release_number) = release_number {
                e.release_number == release_number
            } else {
                true
            }
        })
        .filter(|e| {
            if let Some(instance_type) = instance_type {
                e.instance_type == instance_type
            } else {
                true
            }
        })
        .filter(|e| {
            if let Some(architecture) = architecture {
                e.architecture == architecture
            } else {
                true
            }
        })
        .collect();
    amis.sort_by_key(|e| e.date.clone());
    let entry = amis
        .pop()
        .ok_or_else(|| anyhow::anyhow!("Could not find ami for criteria"))?;

    Ok(parse_ami(&entry.ami)
        .ok_or_else(|| anyhow::anyhow!("Failure parsing ami"))?
        .to_string())
}

fn parse_ami(a_tag: &str) -> Option<&str> {
    // example:
    // "<a href=\"https://console.aws.amazon.com/ec2/home?region=us-east-1#launchAmi=ami-085925f297f89fce1\">ami-085925f297f89fce1</a>"
    let start_idx = a_tag.find('>')?;
    let end_idx = a_tag.rfind('<')?;
    Some(&a_tag[start_idx + 1..end_idx])
}
