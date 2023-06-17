use rs_llama_cpp::{gpt_params_c, run_inference, str_to_mut_i8};
use std::io::{self, Write};

fn main() {
    let params: gpt_params_c = {
        gpt_params_c {
            n_threads: 8,
            temp: 0.0,
            use_mlock: true,
            model: str_to_mut_i8("./models/13B/ggml-model.bin"),
            prompt: str_to_mut_i8("Here is a short greeting message in English: \""),
            ..Default::default()
        }
    };

    run_inference(params, |x| {
        if x.ends_with("\"") {
            print!("{}", x.replace("\"", ""));
            io::stdout().flush().unwrap();

            return true; // stop inference
        }

        print!("{}", x);
        io::stdout().flush().unwrap();

        return true; // continue inference
    });
}
