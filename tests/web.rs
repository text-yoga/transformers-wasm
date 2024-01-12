//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::fmt::format;
use std::println;

use gloo::console::log;
use js_sys::Uint8Array;
use transformers_wasm::quantized_llama2::{run_model, Args, Which};
use transformers_wasm::utils;
use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn pass() -> Result<(), JsValue> {
    // #[cfg(web_sys_unstable_apis)]
    // log!(utils::has_gpu().await);

    let prompt: String = String::from(
        "<|system|>
        You are a helpful assistant that answers questions in a friendly manner.</s>
        <|user|>
        Can you give a simple recipe for a nice lentil soup?</s>
        <|assistant|>",
    );
    let args = Args {
        model: Some(String::from("")),
        prompt: Some(prompt),
        sample_len: 20,
        tokenizer: None,
        temperature: 0.8,
        top_p: None,
        seed: 299792458,
        tracing: true,
        verbose_prompt: true,
        repeat_penalty: 1.1,
        repeat_last_n: 64,
        which: Which::L7b,
        // gqa: None,
    };
    run_model(args).await?;
    // let tokenizer_url = "http://localhost:31300/TinyLlama_TinyLlama-1.1B-Chat-v1.0/tokenizer.json";
    // let model_url = "http://localhost:31300/TheBloke_TinyLlama-1.1B-Chat-v1.0-GGUF/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf";

    // log!("Downloading tokenizer...");
    // let tokenizer_blob: Vec<u8> = utils::load_binary(&tokenizer_url).await?;
    // let tokenizer_blob_len = format!("{}", &tokenizer_blob.len());
    // log!("...done! Tokenizer size:", &tokenizer_blob_len);

    // log!("Downloading model...");
    // let model_blob: Vec<u8> = utils::load_binary(&model_url).await?;
    // let model_blob_len = format!("{}", &model_blob.len());
    // log!("...done! Model size:", &model_blob_len);

    // log!("Initialising model...");

    // let mut model = Model::new(model_blob, tokenizer_blob)?;
    // log!("...done!");
    // let prompt: String = String::from(
    //     "<|system|>
    //     You are a helpful assistant that answers questions in a friendly manner.</s>
    //     <|user|>
    //     Can you give a simple recipe for a nice lentil soup?</s>
    //     <|assistant|>",
    // );
    // let temp: f64 = 0.8;
    // let top_p: f64 = 1.;
    // let repeat_penalty: f32 = 1.1;
    // let seed: u64 = 203948203948;
    // let max_token = 20;
    // log!("Initialising prompt...");
    // let first_result: String = model.init_with_prompt(prompt, temp, top_p, repeat_penalty, seed)?;
    // log!("...done. First token:", &first_result);

    // let mut result = first_result;
    // for index in 0..max_token {
    //     let tok = model.next_token()?;
    //     result += &tok;
    //     log!(tok);
    // }
    // log!("\nComplete result:");
    // log!(result);
    // let result = assert_eq!(1 + 1, 2);
    Ok(())
}
