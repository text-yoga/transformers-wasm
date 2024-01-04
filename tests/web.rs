//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::fmt::format;
use std::println;

use js_sys::Uint8Array;
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

    let tokenizer: Vec<u8> = load_binary(&tokenizer_url).await?;
    let tokenizer_len = format!("{}", &tokenizer.len());
    console::log_2(&"tokenizer size".into(), &tokenizer_len.into());

    let model: Vec<u8> = load_binary(&model_url).await?;
    let model_len = format!("{}", &model.len());
    console::log_2(&"model size".into(), &model_len.into());

    assert_eq!(1 + 1, 2);
    Ok(())
}
