mod command;
mod gh;

#[tokio::main]
async fn main() {
    let res = tokio::task::spawn_blocking(command::figure)
        .await
        .expect("async comp not working")
        .await;

    match res {
        Ok(message) => {
            println!("{message}")
        }

        Err(error_message) => {
            let message = error_message.to_string();
            println!("{message}");
            std::process::exit(1)
        }
    }
}
