use crate::class::{Accelerator, Menu, NativeImage};
use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MenuItemOptions {
    accelerator_works_when_hidden: Option<bool>,
    accelerator: Option<Accelerator>,
    after_group_containing: Option<Array>,
    after: Option<Array>,
    before_group_containing: Option<Array>,
    before: Option<Array>,
    checked: Option<bool>,
    click: Option<Function>,
    enabled: Option<bool>,
    icon: Option<NativeImage>,
    id: Option<JsString>,
    kind: Option<JsString>,
    label: Option<JsString>,
    register_accelerator: Option<bool>,
    role: Option<JsString>,
    sub_label: Option<JsString>,
    submenu: Option<Menu>,
    tool_tip: Option<JsString>,
    visible: Option<bool>,
}

#[wasm_bindgen]
impl MenuItemOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        accelerator_works_when_hidden: Option<bool>,
        accelerator: Option<Accelerator>,
        after_group_containing: Option<Array>,
        after: Option<Array>,
        before_group_containing: Option<Array>,
        before: Option<Array>,
        checked: Option<bool>,
        click: Option<Function>,
        enabled: Option<bool>,
        icon: Option<NativeImage>,
        id: Option<JsString>,
        kind: Option<JsString>,
        label: Option<JsString>,
        register_accelerator: Option<bool>,
        role: Option<JsString>,
        sub_label: Option<JsString>,
        submenu: Option<Menu>,
        tool_tip: Option<JsString>,
        visible: Option<bool>,
    ) -> MenuItemOptions {
        MenuItemOptions {
            accelerator_works_when_hidden,
            accelerator,
            after_group_containing,
            after,
            before_group_containing,
            before,
            checked,
            click,
            enabled,
            icon,
            id,
            kind,
            label,
            register_accelerator,
            role,
            sub_label,
            submenu,
            tool_tip,
            visible,
        }
    }

    pub fn new() -> MenuItemOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn accelerator_works_when_hidden(&self) -> Option<bool> {
        self.accelerator_works_when_hidden
    }

    #[wasm_bindgen(setter)]
    pub fn set_accelerator_works_when_hidden(&mut self, value: Option<bool>) {
        self.accelerator_works_when_hidden = value;
    }

    #[wasm_bindgen(getter)]
    pub fn accelerator(&self) -> Option<Accelerator> {
        self.accelerator.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_accelerator(&mut self, value: Option<Accelerator>) {
        self.accelerator = value;
    }

    #[wasm_bindgen(getter)]
    pub fn after_group_containing(&self) -> Option<Array> {
        self.after_group_containing.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_after_group_containing(&mut self, value: Option<Array>) {
        self.after_group_containing = value;
    }

    #[wasm_bindgen(getter)]
    pub fn after(&self) -> Option<Array> {
        self.after.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_after(&mut self, value: Option<Array>) {
        self.after = value;
    }

    #[wasm_bindgen(getter)]
    pub fn before_group_containing(&self) -> Option<Array> {
        self.before_group_containing.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_before_group_containing(&mut self, value: Option<Array>) {
        self.before_group_containing = value;
    }

    #[wasm_bindgen(getter)]
    pub fn before(&self) -> Option<Array> {
        self.before.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_before(&mut self, value: Option<Array>) {
        self.before = value;
    }

    #[wasm_bindgen(getter)]
    pub fn checked(&self) -> Option<bool> {
        self.checked
    }

    #[wasm_bindgen(setter)]
    pub fn set_checked(&mut self, value: Option<bool>) {
        self.checked = value;
    }

    #[wasm_bindgen(getter)]
    pub fn click(&self) -> Option<Function> {
        self.click.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_click(&mut self, value: Option<Function>) {
        self.click = value;
    }

    #[wasm_bindgen(getter)]
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    #[wasm_bindgen(setter)]
    pub fn set_enabled(&mut self, value: Option<bool>) {
        self.enabled = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<NativeImage> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<NativeImage>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Option<JsString> {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, value: Option<JsString>) {
        self.id = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter, js_name = "type")]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<JsString> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, value: Option<JsString>) {
        self.label = value;
    }

    #[wasm_bindgen(getter)]
    pub fn register_accelerator(&self) -> Option<bool> {
        self.register_accelerator
    }

    #[wasm_bindgen(setter)]
    pub fn set_register_accelerator(&mut self, value: Option<bool>) {
        self.register_accelerator = value;
    }

    #[wasm_bindgen(getter)]
    pub fn role(&self) -> Option<JsString> {
        self.role.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_role(&mut self, value: Option<JsString>) {
        self.role = value;
    }

    #[wasm_bindgen(getter)]
    pub fn sub_label(&self) -> Option<JsString> {
        self.sub_label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_sub_label(&mut self, value: Option<JsString>) {
        self.sub_label = value;
    }

    #[wasm_bindgen(getter)]
    pub fn submenu(&self) -> Option<Menu> {
        self.submenu.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_submenu(&mut self, value: Option<Menu>) {
        self.submenu = value;
    }

    #[wasm_bindgen(getter)]
    pub fn tool_tip(&self) -> Option<JsString> {
        self.tool_tip.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tool_tip(&mut self, value: Option<JsString>) {
        self.tool_tip = value;
    }

    #[wasm_bindgen(getter)]
    pub fn visible(&self) -> Option<bool> {
        self.visible
    }

    #[wasm_bindgen(setter)]
    pub fn set_visible(&mut self, value: Option<bool>) {
        self.visible = value;
    }
}
