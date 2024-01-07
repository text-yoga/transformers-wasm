// Adapted from https://github.com/huggingface/candle/blob/main/candle-wasm-examples/llama2-c/src/bin/m.rs
use std::io::Cursor;

use candle::{quantized::gguf_file, Device, Tensor};
use candle_transformers::{generation::LogitsProcessor, models::quantized_llama::ModelWeights};

use js_sys::Uint32Array;

use tokenizers::Tokenizer;
// use text_yoga_ai::worker::{Model as M, ModelData};
use wasm_bindgen::prelude::*;
use web_time as time;

use gloo::console::log;
use web_sys::console;

#[wasm_bindgen]
pub struct Model {
    inner: ModelWeights,
    logits_processor: LogitsProcessor,
    tokens: Vec<u32>,
    repeat_penalty: f32,
    tokenizer: Tokenizer,
}

impl Model {
    fn process(&mut self, tokens: &[u32]) -> candle::Result<String> {
        let u32_array = Uint32Array::new_with_length(tokens.len() as u32);
        u32_array.copy_from(tokens);
        log!("Processing tokens", u32_array);

        const REPEAT_LAST_N: usize = 64;
        let dev = Device::Cpu;
        let input = Tensor::new(tokens, &dev)?.unsqueeze(0)?;
        let logits = self.inner.forward(&input, tokens.len())?;
        let logits = logits.squeeze(0)?;
        let logits = if self.repeat_penalty == 1. || tokens.is_empty() {
            logits
        } else {
            let start_at = self.tokens.len().saturating_sub(REPEAT_LAST_N);
            candle_transformers::utils::apply_repeat_penalty(
                &logits,
                self.repeat_penalty,
                &self.tokens[start_at..],
            )?
        };

        let next_token_id = self.logits_processor.sample(&logits)?;

        let js: JsValue = next_token_id.into();
        console::log_2(&"Generated token id: ".into(), &js);

        self.tokens.push(next_token_id);

        let next_token = self.tokenizer.id_to_token(next_token_id);

        let js: JsValue = next_token.clone().into();
        console::log_2(&"Generated token: ".into(), &js);

        let text = match next_token {
            Some(text) => text.replace('▁', " ").replace("<0x0A>", "\n"),
            None => "".to_string(),
        };
        Ok(text)
    }
}

#[wasm_bindgen]
impl Model {
    #[wasm_bindgen(constructor)]
    pub fn new(weights: Vec<u8>, tokenizer: Vec<u8>) -> Result<Model, JsError> {
        log!("Initialising model...");
        let seed = 299792458;
        let temperature: Option<f64> = Some(0.8);
        let top_p: Option<f64> = None;
        let repeat_penalty: f32 = 1.;
        let start = time::Instant::now();
        let mut cursor = Cursor::new(&weights);
        let mut cursor2 = Cursor::new(&weights);
        let model: ModelWeights = {
            log!("Loading gguf file...");
            let model = gguf_file::Content::read(&mut cursor)?;
            log!("gguf file loaded.");
            let mut total_size_in_bytes = 0;
            for (_, tensor) in model.tensor_infos.iter() {
                let elem_count = tensor.shape.elem_count();
                log!("elem_count", elem_count);
                log!("type_size", tensor.ggml_dtype.type_size());
                log!("blck_size", tensor.ggml_dtype.blck_size());
                total_size_in_bytes +=
                    // Very important to keep the parenthesis here, otherwise might overflow (in test).
                    elem_count * (tensor.ggml_dtype.type_size() / tensor.ggml_dtype.blck_size());
            }
            log!(format!(
                "loaded {:?} tensors ({}) in {:.2}s",
                model.tensor_infos.len(),
                format_size(total_size_in_bytes),
                start.elapsed().as_secs_f32()
            ));
            ModelWeights::from_gguf(model, &mut cursor2)?
        };
        println!("model built");

        let tokenizer = Tokenizer::from_bytes(&tokenizer)
            .map_err(|_msg| JsError::new("Failed to load tokenizer."))?;
        let logits_processor = LogitsProcessor::new(seed, temperature, top_p);
        Ok(Self {
            inner: model,
            logits_processor,
            repeat_penalty,
            tokens: vec![],
            tokenizer,
        })
    }

    #[wasm_bindgen]
    pub fn get_seq_len(&mut self) -> usize {
        // self.inner.config.seq_len
        100
    }

    #[wasm_bindgen]
    pub fn init_with_prompt(
        &mut self,
        prompt: String,
        temp: f64,
        top_p: f64,
        repeat_penalty: f32,
        seed: u64,
    ) -> Result<String, JsError> {
        console::log_1(&"Initialising prompt with temperature=".into());
        console::log_2(&"temperature=".into(), &temp.into());
        console::log_2(&"top_p=".into(), &top_p.into());
        console::log_2(&"repeat_penalty=".into(), &repeat_penalty.into());
        console::log_2(&"seed=".into(), &seed.into());
        // First reset the cache.
        // {
        //     let mut cache = self.inner.cache.kvs.lock().unwrap();
        //     for elem in cache.iter_mut() {
        //         *elem = None
        //     }
        // }
        let temp = if temp <= 0. { None } else { Some(temp) };
        let top_p = if top_p <= 0. || top_p >= 1. {
            None
        } else {
            Some(top_p)
        };
        self.logits_processor = LogitsProcessor::new(seed, temp, top_p);
        self.repeat_penalty = repeat_penalty;
        self.tokens.clear();
        let tokens = self
            .tokenizer
            .encode(prompt, true)
            .map_err(|m| JsError::new(&m.to_string()))?
            .get_ids()
            .to_vec();
        let text = self
            .process(&tokens)
            .map_err(|m| JsError::new(&m.to_string()))?;
        Ok(text)
    }

    #[wasm_bindgen]
    pub fn next_token(&mut self) -> Result<String, JsError> {
        let last_token = *self.tokens.last().unwrap();
        let text = self
            .process(&[last_token])
            .map_err(|m| JsError::new(&m.to_string()))?;
        Ok(text)
    }
}

fn format_size(size_in_bytes: usize) -> String {
    if size_in_bytes < 1_000 {
        format!("{}B", size_in_bytes)
    } else if size_in_bytes < 1_000_000 {
        format!("{:.2}KB", size_in_bytes as f64 / 1e3)
    } else if size_in_bytes < 1_000_000_000 {
        format!("{:.2}MB", size_in_bytes as f64 / 1e6)
    } else {
        format!("{:.2}GB", size_in_bytes as f64 / 1e9)
    }
}
