use gloo::console::log;
use js_sys::{ArrayBuffer, Uint8Array, JSON};

use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Navigator, Request, RequestInit, RequestMode, Response, Window};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

fn to_error(value: JsValue) -> JsError {
    JsError::new(
        JSON::stringify(&value)
            .map(|js_string| {
                js_string
                    .as_string()
                    .unwrap_or(String::from("An unknown error occurred."))
            })
            .unwrap_or(String::from("An unknown error occurred."))
            .as_str(),
    )
}

async fn fetch(url: &str) -> Result<Response, JsError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).map_err(to_error)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(to_error)?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    Ok(resp)
}

pub async fn load_json(url: &str) -> Result<JsValue, JsError> {
    let response = fetch(url).await?;
    let json = JsFuture::from(
        response
            .json()
            .map_err(|err| JsError::new("Failed to parse json"))?,
    )
    .await
    .map_err(to_error)?;
    Ok(json)
}

pub async fn load_binary(url: &str) -> Result<Vec<u8>, JsError> {
    let response = fetch(url).await?;
    let ab = JsFuture::from(response.array_buffer().map_err(to_error)?)
        .await
        .map_err(to_error)?;

    let vec = Uint8Array::new(&ab).to_vec();
    let bla = (&vec.iter().take(10).map(|x| x.clone()).collect::<Vec<u8>>()).clone();
    let x = js_sys::Uint8Array::from(bla.as_slice());
    log!(url, x);
    Ok(vec)
}

#[cfg(web_sys_unstable_apis)]
pub async fn has_gpu() -> bool {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator = window.navigator();

    let gpu: web_sys::Gpu = navigator.gpu();
    let has_gpu_check = JsFuture::from(gpu.request_adapter()).await;

    let mut has_gpu = false;
    match has_gpu_check {
        Ok(_) => has_gpu = true,
        Err(err) => {}
    }
    log!("wgsl_language_features");
    has_gpu
}
