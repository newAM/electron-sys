use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FindInPageOptions {
    find_next: Option<bool>,
    forward: Option<bool>,
    match_case: Option<bool>,
    medial_capital_as_word_start: Option<bool>,
    word_start: Option<bool>,
}

#[wasm_bindgen]
impl FindInPageOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        find_next: Option<bool>,
        forward: Option<bool>,
        match_case: Option<bool>,
        medial_capital_as_word_start: Option<bool>,
        word_start: Option<bool>,
    ) -> FindInPageOptions {
        FindInPageOptions {
            find_next,
            forward,
            match_case,
            medial_capital_as_word_start,
            word_start,
        }
    }

    #[wasm_bindgen(getter, js_name = "findNext")]
    pub fn find_next(self) -> Option<bool> {
        self.find_next
    }

    #[wasm_bindgen(setter)]
    pub fn set_find_next(mut self, value: Option<bool>) {
        self.find_next = value;
    }

    #[wasm_bindgen(getter)]
    pub fn forward(self) -> Option<bool> {
        self.forward
    }

    #[wasm_bindgen(setter)]
    pub fn set_forward(mut self, value: Option<bool>) {
        self.forward = value;
    }

    #[wasm_bindgen(getter, js_name = "matchCase")]
    pub fn match_case(self) -> Option<bool> {
        self.match_case
    }

    #[wasm_bindgen(setter)]
    pub fn set_match_case(mut self, value: Option<bool>) {
        self.match_case = value;
    }

    #[wasm_bindgen(getter, js_name = "medialCapitalAsWordStart")]
    pub fn medial_capital_as_word_start(self) -> Option<bool> {
        self.medial_capital_as_word_start
    }

    #[wasm_bindgen(setter)]
    pub fn set_medial_capital_as_word_start(mut self, value: Option<bool>) {
        self.medial_capital_as_word_start = value;
    }

    #[wasm_bindgen(getter, js_name = "wordStart")]
    pub fn word_start(self) -> Option<bool> {
        self.word_start
    }

    #[wasm_bindgen(setter)]
    pub fn set_word_start(mut self, value: Option<bool>) {
        self.word_start = value;
    }
}
