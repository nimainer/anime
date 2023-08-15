use frankenstein::Message;
use frankenstein::AsyncApi;
pub mod resposes;
pub mod reply_finder;

pub async fn process_message(message: Message, api: AsyncApi) {
    resposes::handle_input(&message, &api).await;
}

