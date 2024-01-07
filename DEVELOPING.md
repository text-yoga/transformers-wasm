# Developing


## Testing

Download chromedriver from https://googlechromelabs.github.io/chrome-for-testing/#stable

```bash
RUST_LOG=wasm_bindgen_test_runner wasm-pack test --chrome --chromedriver "$(which chromedriver)" --headless
```
