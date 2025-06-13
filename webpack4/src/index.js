async function loadWasm() {
  try {
    // 导入生成的 JS 绑定
    const rustWasm = await import('../rust-wasm-webpack4/pkg/rust_wasm_webpack4.js');
    console.log(rustWasm, 'rustWasm')
  } catch (err) {
    console.error('加载 WASM 失败:', err);
  }
}

loadWasm();