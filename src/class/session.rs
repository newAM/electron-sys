use crate::{
    class::WebRequest,
    interface::{
        ClearStorageDataOptions,
        CreateInterruptedDownloadOptions,
        EnableNetworkEmulationOptions,
        NetLog,
        PreconnectOptions,
        Protocol,
        RemoveClientCertificate,
        RemovePassword,
        SetProxyOptions,
    },
};
use js_sys::{Array, Function, JsString, Promise};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/session
    pub type Session;

    // Static Methods

    // Instance Methods

    #[wasm_bindgen(method, js_name = "allowNTLMCredentialsForDomains")]
    pub fn allow_ntlm_credentials_for_domains(this: &Session, domains: &JsString);

    #[wasm_bindgen(method, js_name = "clear_auth_cache")]
    pub fn clear_auth_cache_and_remove_password(this: &Session, options: RemovePassword) -> Promise;

    #[wasm_bindgen(method, js_name = "clear_auth_cache")]
    pub fn clear_auth_cache_and_remove_client_certificate(this: &Session, options: RemoveClientCertificate) -> Promise;

    #[wasm_bindgen(method, js_name = "clearCache")]
    pub fn clear_cache(this: &Session) -> Promise;

    #[wasm_bindgen(method, js_name = "clearHostResolverCache")]
    pub fn clear_host_resolver_cache(this: &Session) -> Promise;

    #[wasm_bindgen(method, js_name = "clearStorageData")]
    pub fn clear_storage_data(this: &Session, options: Option<ClearStorageDataOptions>) -> Promise;

    #[wasm_bindgen(method, js_name = "createInterruptedDownload")]
    pub fn create_interrupted_download(this: &Session, options: CreateInterruptedDownloadOptions);

    #[wasm_bindgen(method, js_name = "disableNetworkEmulation")]
    pub fn disable_network_emulation(this: &Session);

    #[wasm_bindgen(method, js_name = "downloadURL")]
    pub fn download_url(this: &Session, url: &JsString);

    #[wasm_bindgen(method, js_name = "enableNetworkEmulation")]
    pub fn enable_network_emulation(this: &Session, options: EnableNetworkEmulationOptions);

    #[wasm_bindgen(method, js_name = "flushStorageData")]
    pub fn flush_storage_data(this: &Session);

    #[wasm_bindgen(method, js_name = "getBlobData")]
    pub fn get_blob_data(this: &Session, identifier: &JsString) -> Promise;

    #[wasm_bindgen(method, js_name = "getCacheSize")]
    pub fn get_cache_size(this: &Session) -> Promise;

    #[wasm_bindgen(method, js_name = "getPreloads")]
    pub fn get_preloads(this: &Session) -> Array;

    #[wasm_bindgen(method, js_name = "getSpellCheckerLanguages")]
    pub fn get_spell_checker_languages(this: &Session) -> Array;

    #[wasm_bindgen(method, js_name = "getUserAgent")]
    pub fn get_user_agent(this: &Session) -> Array;

    #[wasm_bindgen(method)]
    pub fn preconnect(this: &Session, options: PreconnectOptions) -> Array;

    #[wasm_bindgen(method, js_name = "resolveProxy")]
    pub fn resolve_proxy(this: &Session, url: &JsString) -> Promise;

    #[wasm_bindgen(method, js_name = "setCertificateVerifyProc")]
    pub fn set_certificate_verify_proc(this: &Session, proc: Option<&Function>);

    #[wasm_bindgen(method, js_name = "setDownloadPath")]
    pub fn set_download_path(this: &Session, path: &JsString);

    #[wasm_bindgen(method, js_name = "setPermissionCheckHandler")]
    pub fn set_permission_check_handler(this: &Session, handler: Option<&Function>);

    #[wasm_bindgen(method, js_name = "setPermissionRequestHandler")]
    pub fn set_permission_request_handler(this: &Session, handler: Option<&Function>);

    #[wasm_bindgen(method, js_name = "setPreloads")]
    pub fn set_preloads(this: &Session, preloads: &Array);

    #[wasm_bindgen(method, js_name = "setProxy")]
    pub fn set_proxy(this: &Session, options: SetProxyOptions) -> Promise;

    #[wasm_bindgen(method, js_name = "setSpellCheckerDictionaryDownloadURL")]
    pub fn set_spell_checker_dictionary_download_url(this: &Session, url: &JsString);

    #[wasm_bindgen(method, js_name = "setSpellCheckerLanguages")]
    pub fn set_spell_checker_languages(this: &Session, languages: &Array);

    #[wasm_bindgen(method, js_name = "setUserAgent")]
    pub fn set_user_agent(this: &Session, user_agent: &JsString, accept_languages: Option<JsString>);

    // Instance Properties

    #[wasm_bindgen(method, getter, js_name = "availableSpellCheckerLanguages")]
    pub fn available_spell_checker_languages(this: &Session) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn cookies(this: &Session) -> Array;

    #[wasm_bindgen(method, getter, js_name = "netLog")]
    pub fn net_log(this: &Session) -> NetLog;

    #[wasm_bindgen(method, getter)]
    pub fn protocol(this: &Session) -> Protocol;

    #[wasm_bindgen(method, getter, js_name = "webRequest")]
    pub fn web_request(this: &Session) -> WebRequest;
}
