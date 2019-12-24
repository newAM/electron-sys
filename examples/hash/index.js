import init from "./crates/app-renderer/pkg/hash_app_renderer.js";

async function main() {
  await init("./crates/app-renderer/pkg/hash_app_renderer_bg.wasm");
}

main();
