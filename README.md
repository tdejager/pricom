# Pricom

Small identicons for personal experimentation.


![2023-01-25 15 12 59](https://user-images.githubusercontent.com/417374/214806905-62a73d70-14c7-404e-b7e4-dcc2077276ae.gif)

## Running on wasm

This should run both natively and on the web, but the first use case is integrating it into the website prefix.de

To build you should have [wasm-pack](https://github.com/rustwasm/wasm-pack) installed.

Then run:

```bash
wasm-pack build -t web
```

This will create a `pkg` file in the repository. You can then proceed to use either someting like `python3 -m http.server` or [miniserve](https://github.com/svenstaro/miniserve)

You can navigate to the `index.html` page and try it out.

## Testing

Uses wasm-pack for testing
```bash
was-pack test --firefox --headless
```
