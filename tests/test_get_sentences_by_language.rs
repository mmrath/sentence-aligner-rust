extern crate postgres;
extern crate reqwest;
extern crate uuid;
extern crate interface_tests_helpers;

#[macro_use] extern crate serde_derive;

use reqwest::StatusCode;

use postgres::Connection;

use interface_tests_helpers::ResponseHandler;

mod db;
mod handlers;

use db::DatabaseHandler;
use handlers::SentenceHandler;

#[path = "../utils/tests_commons.rs"]
mod tests_commons;

#[test]
fn test_get_sentence_by_language_returns_200() {

    let connection: Connection = DatabaseHandler::connect_and_clean();

    let first_iso639_3 = "eng";
    connection.insert_language(&first_iso639_3);

    let second_iso639_3 = "fra";
    connection.insert_language(&second_iso639_3);

    let first_text = "This is one sentence";
    connection.insert_sentence(&first_text, &first_iso639_3);

    let second_text = "This is a second sentence";
    connection.insert_sentence(&second_text, &first_iso639_3);

    let third_text = "Ceci est une phrase.";
    connection.insert_sentence(&third_text, &second_iso639_3);

    let client = reqwest::Client::new();
    let mut response = client.get_sentences_by_language(&first_iso639_3);

    response.assert_200();

    let sentences = response.json::<tests_commons::Sentences>().unwrap();

    assert_eq!(sentences.len(), 2);
}
