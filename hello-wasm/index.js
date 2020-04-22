const wasm = import("./rust/pkg/hello_wasm.js");
wasm.then(wasm => {
  wasm.greet("WebAssembly");
});