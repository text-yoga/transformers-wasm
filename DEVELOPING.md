# Developing


## Testing

Download chromedriver from https://googlechromelabs.github.io/chrome-for-testing/#stable

```bash
wasm-pack test --chrome --chromedriver "$(which chromedriver)" --headless
```
