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
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn pass() -> Result<(), JsValue> {
    let tokenizer_url = "http://localhost:31300/tokenizer.json";
    let model_url = "http://localhost:31300/tinymistral-248m.q4_k_m.gguf";

    let tokenizer_blob: Vec<u8> = utils::load_binary(&tokenizer_url).await?;
    let tokenizer_blob_len = format!("{}", &tokenizer_blob.len());
    log!("tokenizer blob size", &tokenizer_blob_len);

    let model_blob: Vec<u8> = utils::load_binary(&model_url).await?;
    let model_blob_len = format!("{}", &model_blob.len());
    log!("model blob size", &model_blob_len);

    log!("loading model...");
    let mut model = Model::new(model_blob, tokenizer_blob)?;
    log!("model loaded.");
    let prompt: String = String::from("What is a good recipe for onion soup");
    let temp: f64 = 0.8;
    let top_p: f64 = 1.;
    let repeat_penalty: f32 = 1.1;
    let seed: u64 = 203948203948;
    let first_result: String = model.init_with_prompt(prompt, temp, top_p, repeat_penalty, seed)?;

    log!("first prompt result", &first_result);
    assert_eq!(1 + 1, 2);
    Ok(())
}
