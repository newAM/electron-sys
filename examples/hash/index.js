import { default as init, main } from "./crates/app-renderer/pkg/hash_app_renderer.js";

async function run() {
  await init("./crates/app-renderer/pkg/hash_app_renderer_bg.wasm");
  main();
}

run();
