use gloo::console::log;
use js_sys::{ArrayBuffer, Uint8Array};

use wasm_bindgen::{prelude::*, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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

async fn fetch(url: &str) -> Result<Response, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    Ok(resp)
}

pub async fn load_json(url: &str) -> Result<JsValue, JsValue> {
    let response = fetch(url).await?;
    let json = JsFuture::from(response.json()?).await?;
    Ok(json)
}

pub async fn load_binary(url: &str) -> Result<Vec<u8>, JsValue> {
    let response = fetch(url).await?;
    let ab = JsFuture::from(response.array_buffer()?).await?;

    let vec = Uint8Array::new(&ab).to_vec();
    let bla = (&vec.iter().take(10).map(|x| x.clone()).collect::<Vec<u8>>()).clone();
    let x = js_sys::Uint8Array::from(bla.as_slice());
    log!(url, x);
    Ok(vec)
}
