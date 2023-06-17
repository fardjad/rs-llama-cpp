/* automatically generated by rust-bindgen 0.66.0 */

pub type token_callback =
    ::std::option::Option<unsafe extern "C" fn(token: *const ::std::os::raw::c_char) -> bool>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gpt_params_c {
    pub seed: i32,
    pub n_threads: i32,
    pub n_predict: i32,
    pub n_ctx: i32,
    pub n_batch: i32,
    pub n_keep: i32,
    pub n_gpu_layers: i32,
    pub main_gpu: i32,
    pub tensor_split: [f32; 1usize],
    pub top_k: i32,
    pub top_p: f32,
    pub tfs_z: f32,
    pub typical_p: f32,
    pub temp: f32,
    pub repeat_penalty: f32,
    pub repeat_last_n: i32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub mirostat: ::std::os::raw::c_int,
    pub mirostat_tau: f32,
    pub mirostat_eta: f32,
    pub model: *mut ::std::os::raw::c_char,
    pub model_alias: *mut ::std::os::raw::c_char,
    pub prompt: *mut ::std::os::raw::c_char,
    pub path_prompt_cache: *mut ::std::os::raw::c_char,
    pub input_prefix: *mut ::std::os::raw::c_char,
    pub input_suffix: *mut ::std::os::raw::c_char,
    pub lora_adapter: *mut ::std::os::raw::c_char,
    pub lora_base: *mut ::std::os::raw::c_char,
    pub memory_f16: bool,
    pub random_prompt: bool,
    pub use_color: bool,
    pub interactive: bool,
    pub prompt_cache_all: bool,
    pub prompt_cache_ro: bool,
    pub embedding: bool,
    pub interactive_first: bool,
    pub multiline_input: bool,
    pub instruct: bool,
    pub penalize_nl: bool,
    pub perplexity: bool,
    pub use_mmap: bool,
    pub use_mlock: bool,
    pub mem_test: bool,
    pub export_cgraph: bool,
    pub verbose_prompt: bool,
}
#[test]
fn bindgen_test_layout_gpt_params_c() {
    const UNINIT: ::std::mem::MaybeUninit<gpt_params_c> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<gpt_params_c>(),
        176usize,
        concat!("Size of: ", stringify!(gpt_params_c))
    );
    assert_eq!(
        ::std::mem::align_of::<gpt_params_c>(),
        8usize,
        concat!("Alignment of ", stringify!(gpt_params_c))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).seed) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(seed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_threads) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(n_threads)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_predict) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(n_predict)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_ctx) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(n_ctx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_batch) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(n_batch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_keep) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(n_keep)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n_gpu_layers) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(n_gpu_layers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).main_gpu) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(main_gpu)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tensor_split) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(tensor_split)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).top_k) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(top_k)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).top_p) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(top_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tfs_z) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(tfs_z)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typical_p) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(typical_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).temp) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(temp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).repeat_penalty) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(repeat_penalty)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).repeat_last_n) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(repeat_last_n)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).frequency_penalty) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(frequency_penalty)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).presence_penalty) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(presence_penalty)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mirostat) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(mirostat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mirostat_tau) as usize - ptr as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(mirostat_tau)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mirostat_eta) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(mirostat_eta)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).model) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(model)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).model_alias) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(model_alias)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prompt) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(prompt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).path_prompt_cache) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(path_prompt_cache)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input_prefix) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(input_prefix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input_suffix) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(input_suffix)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lora_adapter) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(lora_adapter)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lora_base) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(lora_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memory_f16) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(memory_f16)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).random_prompt) as usize - ptr as usize },
        153usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(random_prompt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).use_color) as usize - ptr as usize },
        154usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(use_color)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interactive) as usize - ptr as usize },
        155usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(interactive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prompt_cache_all) as usize - ptr as usize },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(prompt_cache_all)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prompt_cache_ro) as usize - ptr as usize },
        157usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(prompt_cache_ro)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).embedding) as usize - ptr as usize },
        158usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(embedding)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interactive_first) as usize - ptr as usize },
        159usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(interactive_first)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).multiline_input) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(multiline_input)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).instruct) as usize - ptr as usize },
        161usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(instruct)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).penalize_nl) as usize - ptr as usize },
        162usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(penalize_nl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).perplexity) as usize - ptr as usize },
        163usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(perplexity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).use_mmap) as usize - ptr as usize },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(use_mmap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).use_mlock) as usize - ptr as usize },
        165usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(use_mlock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mem_test) as usize - ptr as usize },
        166usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(mem_test)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).export_cgraph) as usize - ptr as usize },
        167usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(export_cgraph)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).verbose_prompt) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(gpt_params_c),
            "::",
            stringify!(verbose_prompt)
        )
    );
}
extern "C" {
    pub fn rs_llama_cpp_run_inference(params: gpt_params_c, callback: token_callback);
}
