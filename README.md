# Conway's game of life using SvelteKit + WASM in Rust

This repository is an example of how to bind a SvelteKit web app to a Rust library using WASM. It displays a simple Game of life.

https://sveltekit-wasm-game-of-life.vercel.app/

## Dependencies

### JavaScript Dependencies

| Name                  | Version | Goal                                       |
| --------------------- | ------- | ------------------------------------------ |
| @sveltejs/kit         | 1.0.0   | The core framework, SvelteKit              |
| vite-plugin-wasm-pack | 0.1.12  | A Vite plugin for handling wasm-pack crate |

### Rust Dependencies

| Name         | Version | Goal                                                            |
| ------------ | ------- | --------------------------------------------------------------- |
| conlife      | 0.1.4   | A library that contains the Game of life logic                  |
| wasm-bindgen | 0.2.74  | A library to handle bindings between JavaScript and WASM module |
| web_sys      | 0.3.4   | A library to access Web APIs provided by current web browsers   |

## How to deploy?

If you deploy this project on Vercel, you can follow the article here: https://betterprogramming.pub/deploying-a-wasm-powered-react-app-on-vercel-cf3cae2a75d6

You only have to do this:

1. Create (or copy) `build.sh` script that contains all the necessary commands (install Rust, wasm-pack, build the Rust and then the SvelteKit app)
2. Change the Vercel deployment configuration `Build Command` to `bash build.sh`
