import init, { World } from "snake-rust";
import Canvas from "./components/Canvas";
import Snake from "./components/Snake";
import Game from "./components/Game";

init().then(() => {
  console.log("Rust wasm module loaded");
});
