use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::{AsyncApi, UpdateContent};
mod reply_handler;


static TOKEN: &str = "YOU'RE TOKEN";

#[tokio::main]
async fn main() {
    let api = AsyncApi::new(TOKEN);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        let result = api.get_updates(&update_params).await;

        // println!("result: {result:?}");
        println!("updating... .. .");

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        let api_clone: AsyncApi = api.clone();

                        tokio::spawn(async move {
                            reply_handler::process_message(message, api_clone).await;
                        });
                    }
                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }
}


