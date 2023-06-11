# rs-llama-cpp

> Automated Rust bindings generation for LLaMA.cpp

## Description

LLaMA.cpp is under heavy development with contributions pouring in from numerous individuals every day. Currently, its C API is very low-level and given how fast the project is evolving, keeping up with the changes and porting the examples into a higher-level API prove to be difficult. As a trade-off, this project prioritizes automation over flexibility by automatically generating Rust bindings for the main example of LLaMA.cpp.

### Limitations

The main design goal of this project is to minimize the effort of updating LLaMA.cpp by automating as many steps as possible. However, this approach does have some limitations:

1. The API is very high-level, resembling a call to the main function of LLaMA.cpp and receiving tokens through a callback function.
2. Currently, the project does not expose parameters with types that are more challenging to convert to Rust, such as `std::unordered_map` and `std::vector`.
3. Some of the parameters exposed via the Rust API are only relevant for a CLI.
4. The generated C++ library outputs a significant amount of debug information to `stderr` and `stdout`. This is not configurable at the moment

## Usage

TODO
