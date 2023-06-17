#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn str_to_mut_i8(s: &str) -> *mut i8 {
    let cstring = std::ffi::CString::new(s).expect("CString::new failed");
    cstring.into_raw() as *mut i8
}

impl Default for gpt_params_c {
    fn default() -> Self {
        gpt_params_c {
            seed: -1,
            n_threads: std::thread::available_parallelism().unwrap().get() as i32,
            n_predict: -1,
            n_ctx: 512,
            n_batch: 512,
            n_keep: 0,
            n_gpu_layers: 0,
            main_gpu: 0,
            tensor_split: [0.00],

            top_k: 40,
            top_p: 0.95,
            tfs_z: 1.0,
            typical_p: 1.0,
            temp: 0.80,
            repeat_penalty: 1.10,
            repeat_last_n: 64,
            frequency_penalty: 0.00,
            presence_penalty: 0.00,
            mirostat: 0,
            mirostat_tau: 5.00,
            mirostat_eta: 0.10,

            model: str_to_mut_i8("./models/7B/ggml-model.bin"),
            model_alias: str_to_mut_i8("unknown"),
            prompt: str_to_mut_i8(""),
            path_prompt_cache: str_to_mut_i8(""),
            input_prefix: str_to_mut_i8(""),
            input_suffix: str_to_mut_i8(""),

            lora_adapter: str_to_mut_i8(""),
            lora_base: str_to_mut_i8(""),

            memory_f16: true,
            random_prompt: false,
            use_color: false,
            interactive: false,
            prompt_cache_all: false,
            prompt_cache_ro: false,

            embedding: false,
            interactive_first: false,
            multiline_input: false,

            instruct: false,
            penalize_nl: true,
            perplexity: false,
            use_mmap: true,
            use_mlock: false,
            mem_test: false,
            export_cgraph: false,
            verbose_prompt: false,
        }
    }
}

static mut RS_TOKEN_CALLBACK: Option<fn(&str) -> bool> = None;
unsafe extern "C" fn c_token_callback(token: *const ::std::os::raw::c_char) -> bool {
    let message = unsafe { std::ffi::CStr::from_ptr(token).to_string_lossy() };
    RS_TOKEN_CALLBACK.unwrap()(&message)
}

static MUTEX: Mutex<()> = Mutex::new(());
pub fn run_inference(params: gpt_params_c, token_callback: fn(&str) -> bool) {
    let _lock = MUTEX.lock().unwrap();

    unsafe {
        RS_TOKEN_CALLBACK = Some(token_callback);
        rs_llama_cpp_run_inference(params, Some(c_token_callback));
        RS_TOKEN_CALLBACK = None;
    };
}
