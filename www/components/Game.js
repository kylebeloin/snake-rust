import { World } from "snake-rust";
import Canvas from "./Canvas";
import Snake from "./Snake";

export default class Game {
  constructor(canvas) {
    this.canvas = new Canvas(canvas);
    this.world = World.new();
    this.snake = new Snake(this.world);

    return this;
  }

  draw() {
    this.canvas.draw();
    this.snake.draw();
  }
}
