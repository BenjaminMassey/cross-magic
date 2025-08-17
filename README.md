# Cross Magic

## About

A game of 5x5 perfectly square crosswords featuring auto-generated puzzles.

Using macroquad for rendering and ollama for generation.

## Example

![Example Image](docs/example.png)

## Source

Word squares were generated from this project: https://github.com/BartMassey/ws5

## LLM Information

This program uses [ollama](https://www.ollama.com) to run [the Qwen3 model](https://huggingface.co/Qwen/Qwen3-8B) via [the `ollama_embed` crate](https://www.github.com/BenjaminMassey/ollama_embed).

While you should follow `ollama_embed`'s details for up-to-date info, it currently should be noted that the first `cargo build` of this crate will involve downloading `ollama`, and the first `cargo run` of this crate will involve downloading the `qwen3` model via `ollama`. These will both take a while, but will not have to happen after the initial times: future plans in this regard TBD.

## Contact

Feel free to contact me at benjamin.w.massey@gmail.com with any questions / inquiries.