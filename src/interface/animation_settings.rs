use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type AnimationSettings;

    #[wasm_bindgen(method, getter, js_name = "shouldRenderRichAnimation")]
    pub fn should_render_rich_animation(this: &AnimationSettings) -> bool;

    #[wasm_bindgen(method, getter, js_name = "scrollAnimationsEnabledBySystem")]
    pub fn scroll_animations_enabled_by_system(this: &AnimationSettings) -> bool;

    #[wasm_bindgen(method, getter, js_name = "prefersReducedMotion")]
    pub fn prefers_reduced_motion(this: &AnimationSettings) -> bool;
}
