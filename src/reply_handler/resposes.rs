use std::fs;

use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::SendPhotoParams;
use frankenstein::AsyncTelegramApi;
use frankenstein::AsyncApi;


pub async fn handle_input(message: &Message, api: &AsyncApi){
    if message.text.is_some() {
        match &message.text {
            Some(text) => {
                let text = text.to_lowercase().clone();
                if text == "hi" || text == "hello" {
                    send_message_text(message, &text, api).await;
                    return;
                } else if text == "/help" {
                    let contents = fs::read_to_string("resources/help.txt")
                    .expect("Should have been able to read the file(error is in file resposes.rs)");
                    send_message_text(message, &contents.as_str(), api).await;
                    return;
                } else if text == "/list" {
                    let contents = fs::read_to_string("resources/list.txt")
                    .expect("Should have been able to read the file(error is in file resposes.rs)");
                    send_message_text(message, &contents.as_str(), api).await;
                    return;
                } else if text.starts_with("/search") {
                    let s: Option<String> = crate::reply_handler::reply_finder::search_file(text[7..].trim());
                    if text[7..].trim().is_empty() {
                        send_message_text(message, "you need to enter in the name of an anime.", api).await;
                        return;
                    }
                    match s {
                        Some(s) => send_message_text(message, &s.as_str(), api).await,
                        None => send_message_text(message, "no anime with such name was found in our data base", api).await,
                    }
                    return;
                }
            },
            None => (),
        }
    };
    
}

async fn send_message_text(message: &Message, text: &str, api: &AsyncApi){
    let send_message_params = SendMessageParams::builder()
    .chat_id(message.chat.id)
    .text(text)
    .build();
    if let Err(err) = api.send_message(&send_message_params).await {
        println!("Failed to send message: {err:?}");
    }
}

async fn _send_message_photo(message: &Message, file_address: &str,  api: &AsyncApi){
    let file = std::path::PathBuf::from(file_address);

    let send_photo_params = SendPhotoParams::builder()
    .chat_id(message.chat.id)
    .photo(file)
    .build();

    if let Err(err) = api.send_photo(&send_photo_params).await{
        println!("Failed to send photo: {err:?}");
    }
}