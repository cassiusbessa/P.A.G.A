import { execSync } from "child_process";
import { log } from "console";

export const compileCommand = `cargo build --release --target wasm32-unknown-unknown`;

export const optimizeCommand = `docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0`;

export const compileContracts = () => {
  log("compiling contracts...");
  execSync(compileCommand, { stdio: "ignore" });
  log("Compilation completed successfully!");
};

export const optimizeContracts = () => {
  log("optimizing contracts...");
  execSync(optimizeCommand, { stdio: "ignore" });
  log("Optimization completed successfully!");
};

export const compileProject = () => {
  compileContracts();
  optimizeContracts();
};
