# AWS Lambda

## Source
Semicolon YouTuber: https://www.youtube.com/watch?v=qL2mh1k3cTc&list=PLDi2liHqCnVomqnjLp9Jq1jk-XFWque6C


## Prerequisites and Installation

- Rust (https://www.rust-lang.org/tools/install)
- Cargo BInstall (https://github.com/cargo-bins/cargo-binstall)
```
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
```
- Cargo Lambda (https://www.cargo-lambda.info/guide/installation.html) for Linux: `cargo binstall cargo-lambda`
- Zig (https://github.com/ziglang/zig/wiki/Install-Zig-from-a-Package-Manager):
`snap install zig --classic --beta`

## Setup

1. Clone the repository:

```bash
git clone git@github.com:AuroraLantean/rust-aws-lambda.git
cd rust-aws-lambda

```

2. Build binaries
   
```bash
cargo lambda build --release --arm64
zip dummy.zip target/lambda/aws-lambda/bootstrap

```

3. Deployment to AWS
- Loging to AWS, search for "Lambda", click on "Create A Function", then "Author from scratch".
- Configuration: Function Name: dummy, Runtime = Amazon Linux 2023(for Go/Rust/C++), Architecture = arm64(according to what architecture you have built your lambda function), then click on "Create Function"
- Under Code tab: Code Source, click on "Upload from", then select your built lambda function
- Under Configuration, and Function URL: Create function URL, Auth = None(so the API is open to the public)
- Copy the function URL

## API Endpoints
https://abc123.lambda-url.ap-northeast-2.on.aws/

https://abc123.lambda-url.ap-northeast-2.on.aws/?name=Jack

### Use Insomnia to send Requests

### Use Slumber to send Requests
```bash
slumber request -p production root
slumber request -p production get_tasks | jq
slumber request -p production add_task1
slumber request -p production update_task1
slumber request -p production delete_task1
```


## License
This project is licensed under the GNU GENERAL PUBLIC LICENSE.
