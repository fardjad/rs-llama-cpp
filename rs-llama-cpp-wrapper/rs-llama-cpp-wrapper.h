#pragma once

#include "run-inference.h"
#include <stdint.h>

extern "C" {
// Generated by ChatGPT with this prompt: Create a C-compatible struct for the
// given C++ struct and write a converter function to convert the C-compatible
// struct to the C++ struct. Use char* for strings.
struct gpt_params_c {
  int32_t seed;
  int32_t n_threads;
  int32_t n_predict;
  int32_t n_ctx;
  int32_t n_batch;
  int32_t n_keep;
  int32_t n_gpu_layers;

  // sampling parameters
  // std::unordered_map<llama_token, float> logit_bias;
  int32_t top_k;
  float top_p;
  float tfs_z;
  float typical_p;
  float temp;
  float repeat_penalty;
  int32_t repeat_last_n;
  float frequency_penalty;
  float presence_penalty;
  int mirostat;
  float mirostat_tau;
  float mirostat_eta;

  char *model;
  char *model_alias;
  char *prompt;
  char *path_prompt_cache;
  char *input_prefix;
  char *input_suffix;
  // char **antiprompt;

  char *lora_adapter;
  char *lora_base;

  int32_t memory_f16;
  int32_t random_prompt;
  int32_t use_color;
  int32_t interactive;
  int32_t prompt_cache_all;

  int32_t embedding;
  int32_t interactive_first;
  int32_t multiline_input;

  int32_t instruct;
  int32_t penalize_nl;
  int32_t perplexity;
  int32_t use_mmap;
  int32_t use_mlock;
  int32_t mem_test;
  int32_t verbose_prompt;
};

void rs_llama_cpp_run_inference(gpt_params_c params, token_callback callback);
}