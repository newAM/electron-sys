/* NOTE: Even though the browser accepts module imports for asynchronous
 * loading, webpack complains unless dynamic import() is used.
 */

// import init from "../crates/app-renderer/pkg/activity_monitor_app_renderer.js";

// async function main() {
//   await init("../crates/app-renderer/pkg/activity_monitor_app_renderer_bg.wasm");
// }

// main();

import("../crates/app-renderer/pkg/activity_monitor_app_renderer.js")
    // .then(module => {
    //     module.init("../crates/app-renderer/pkg/activity_monitor_app_renderer_bg.wasm");
    // })
    .catch(console.error);
