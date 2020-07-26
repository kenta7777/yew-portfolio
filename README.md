# Build
`wasm-pack build --target web --out-name wasm --out-dir ./static`

# Run 
`cargo +nightly install miniserve`
`miniserve ./static --index index.html`