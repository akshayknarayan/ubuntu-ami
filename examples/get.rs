use ubuntu_ami::*;

#[tokio::main]
async fn main() -> Result<(), StdError> {
    let res = get_latest(
        "us-east-1",
        Some("bionic"),
        None,
        Some("hvm:ebs-ssd"),
        Some("amd64"),
    )
    .await?;
    println!("us-east-1 ubuntu:bionic: {}", res);
    Ok(())
}
