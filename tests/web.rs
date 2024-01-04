//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::fmt::format;
use std::println;

use js_sys::Uint8Array;
use transformers_wasm::quantized_mistral::Model;
use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};

wasm_bindgen_test_configure!(run_in_browser);

async fn fetch(url: &str) -> Result<Response, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    // let url = "http://localhost:45678/tokenizer.json";

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    Ok(resp)
}

async fn load_json(url: &str) -> Result<JsValue, JsValue> {
    let response = fetch(url).await?;
    let json = JsFuture::from(response.json()?).await?;

    Ok(json)
}

async fn load_binary(url: &str) -> Result<Vec<u8>, JsValue> {
    let response = fetch(url).await?;
    let ab = JsFuture::from(response.array_buffer()?).await?;
    let vec = Uint8Array::new(&ab).to_vec();
    Ok(vec)
}

#[wasm_bindgen_test]
async fn pass() -> Result<(), JsValue> {
    let tokenizer_url = "http://localhost:45678/tokenizer.json";
    let model_url = "http://localhost:45678/tinymistral-248m.q4_k_m.gguf";

    let tokenizer_blob: Vec<u8> = load_binary(&tokenizer_url).await?;
    let tokenizer_blob_len = format!("{}", &tokenizer_blob.len());
    console::log_2(&"tokenizer blob size".into(), &tokenizer_blob_len.into());

    let model_blob: Vec<u8> = load_binary(&model_url).await?;
    let model_blob_len = format!("{}", &model_blob.len());
    console::log_2(&"model blob size".into(), &model_blob_len.into());

    let mut model = Model::new(tokenizer_blob, model_blob)?;

    let prompt: String = String::from("What is a good recipe for onion soup");
    let temp: f64 = 0.8;
    let top_p: f64 = 1.;
    let repeat_penalty: f32 = 1.1;
    let seed: u64 = 203948203948;
    let first_result: String = model.init_with_prompt(prompt, temp, top_p, repeat_penalty, seed)?;

    console::log_2(&"first prompt result".into(), &first_result.into());
    assert_eq!(1 + 1, 2);
    Ok(())
}
