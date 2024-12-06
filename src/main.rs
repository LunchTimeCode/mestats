


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
            let raw = message.1;
            let out = message.0;
            let message = if raw { out } else { format!("{}", out) };
            println!("{message}")
        }

        Err(error_message) => {
            let message = format!("{}", error_message.to_string());
            println!("{message}");
            std::process::exit(1)
        }
    }
}
