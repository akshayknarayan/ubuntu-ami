# ubuntu-ami

Get your fresh, farm-to-table, single-origin ec2 Ubuntu AMIs.

## Example
```rust
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
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
