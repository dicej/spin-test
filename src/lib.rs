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
fn hello_world(req: Request) -> Result<Response> {
    hello_world_0(req).map_err(|e| {
        eprintln!("{e:?}");
        e
    })
}

fn hello_world_0(_req: Request) -> Result<Response> {
    let mut file = File::open("/static/message.txt")?;
    let mut message = String::new();
    file.read_to_string(&mut message)?;

    let mut sum = 0_u32;

    for n in 1..=200 {
        let mut file = File::open(&format!("/static/asset-{n:03}.txt"))?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        sum = content
            .into_iter()
            .fold(sum, |sum, byte| sum.wrapping_add(byte as u32));
    }

    Ok(http::Response::builder().status(200).body(Some(
        format!("{message}\nsum of asset contents: {sum}\n").into(),
    ))?)
}
