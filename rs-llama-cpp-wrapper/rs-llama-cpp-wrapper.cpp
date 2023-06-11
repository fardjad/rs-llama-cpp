#include "rs-llama-cpp-wrapper.h"

gpt_params convert_params_to_cpp(const gpt_params_c params) {
  gpt_params params_cpp;

  params_cpp.seed = params.seed;
  params_cpp.n_threads = params.n_threads;
  params_cpp.n_predict = params.n_predict;
  params_cpp.n_ctx = params.n_ctx;
  params_cpp.n_batch = params.n_batch;
  params_cpp.n_keep = params.n_keep;
  params_cpp.n_gpu_layers = params.n_gpu_layers;

  params_cpp.top_k = params.top_k;
  params_cpp.top_p = params.top_p;
  params_cpp.tfs_z = params.tfs_z;
  params_cpp.typical_p = params.typical_p;
  params_cpp.temp = params.temp;
  params_cpp.repeat_penalty = params.repeat_penalty;
  params_cpp.repeat_last_n = params.repeat_last_n;
  params_cpp.frequency_penalty = params.frequency_penalty;
  params_cpp.presence_penalty = params.presence_penalty;
  params_cpp.mirostat = params.mirostat;
  params_cpp.mirostat_tau = params.mirostat_tau;
  params_cpp.mirostat_eta = params.mirostat_eta;

  params_cpp.model = std::string(params.model);
  params_cpp.model_alias = std::string(params.model_alias);
  params_cpp.prompt = std::string(params.prompt);
  params_cpp.path_prompt_cache = std::string(params.path_prompt_cache);
  params_cpp.input_prefix = std::string(params.input_prefix);
  params_cpp.input_suffix = std::string(params.input_suffix);

  params_cpp.lora_adapter = std::string(params.lora_adapter);
  params_cpp.lora_base = std::string(params.lora_base);

  params_cpp.memory_f16 = params.memory_f16;
  params_cpp.random_prompt = params.random_prompt;
  params_cpp.use_color = params.use_color;
  params_cpp.interactive = params.interactive;
  params_cpp.prompt_cache_all = params.prompt_cache_all;

  params_cpp.embedding = params.embedding;
  params_cpp.interactive_first = params.interactive_first;
  params_cpp.multiline_input = params.multiline_input;

  params_cpp.instruct = params.instruct;
  params_cpp.penalize_nl = params.penalize_nl;
  params_cpp.perplexity = params.perplexity;
  params_cpp.use_mmap = params.use_mmap;
  params_cpp.use_mlock = params.use_mlock;
  params_cpp.mem_test = params.mem_test;
  params_cpp.verbose_prompt = params.verbose_prompt;

  return params_cpp;
}

void rsllama_run_inference(gpt_params_c params, token_callback on_token) {
  run_inference(convert_params_to_cpp(params), on_token);
}