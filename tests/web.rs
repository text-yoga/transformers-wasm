//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::fmt::format;
use std::println;

use gloo::console::log;
use js_sys::Uint8Array;
use transformers_wasm::quantized_mistral::Model;
use transformers_wasm::utils;
use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn pass() -> Result<(), JsValue> {
    #[cfg(web_sys_unstable_apis)]
    log!(utils::has_gpu().await);

    let tokenizer_url = "http://localhost:31300/TinyLlama_TinyLlama-1.1B-Chat-v1.0/tokenizer.json";
    let model_url = "http://localhost:31300/TheBloke_TinyLlama-1.1B-Chat-v1.0-GGUF/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf";

    let tokenizer_blob: Vec<u8> = utils::load_binary(&tokenizer_url).await?;
    let tokenizer_blob_len = format!("{}", &tokenizer_blob.len());
    log!("tokenizer blob size", &tokenizer_blob_len);

    let model_blob: Vec<u8> = utils::load_binary(&model_url).await?;
    let model_blob_len = format!("{}", &model_blob.len());
    log!("model blob size", &model_blob_len);

    log!("loading model...");

    let mut model = Model::new(model_blob, tokenizer_blob)?;
    log!("model loaded.");
    let prompt: String = String::from(
        "<|system|>
        You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature. If a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information. </s>
    <|user|>
    What is borrow checking in rust?</s>
    <|assistant|>",
    );
    let temp: f64 = 0.8;
    let top_p: f64 = 1.;
    let repeat_penalty: f32 = 1.1;
    let seed: u64 = 203948203948;
    let max_token = 20;
    let first_result: String = model.init_with_prompt(prompt, temp, top_p, repeat_penalty, seed)?;

    log!("first prompt result", &first_result);

    let mut result = first_result;
    for index in 0..max_token {
        let tok = model.next_token()?;
        result += &tok;
        log!(tok);
    }
    log!("\nComplete result:");
    log!(result);
    let result = assert_eq!(1 + 1, 2);
    Ok(())
}
