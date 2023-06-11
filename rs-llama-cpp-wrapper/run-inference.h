#pragma once

#include <build-info.h>
#include <common.h>
#include <llama.h>

extern "C" {
typedef void (*token_callback)(const char *token);
}

int run_inference(gpt_params params, token_callback on_token);