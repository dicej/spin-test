use {
    anyhow::Result,
    spin_sdk::{
        http::{Request, Response},
        http_component,
    },
    std::{fs::File, io::Read},
};

/// A simple Spin HTTP component.
#[http_component]
fn hello_world(_req: Request) -> Result<Response> {
    let mut file = File::open("/static/message.txt")?;
    let mut message = String::new();
    file.read_to_string(&mut message)?;

    Ok(http::Response::builder()
        .status(200)
        .body(Some(message.into()))?)
}
