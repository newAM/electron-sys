use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SpellCheckProvider {
    spell_check: Function,
}

#[wasm_bindgen]
impl SpellCheckProvider {
    #[wasm_bindgen(constructor)]
    pub fn new(spell_check: Function) -> SpellCheckProvider {
        SpellCheckProvider { spell_check }
    }

    #[wasm_bindgen(getter, js_name = "spellCheck")]
    pub fn spell_check(&self) -> Function {
        self.spell_check.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_spell_check(&mut self, value: Function) {
        self.spell_check = value;
    }
}
