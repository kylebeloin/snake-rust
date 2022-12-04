export default class Snake {
  constructor(world) {
    this.world = world;
  }

  draw() {
    const snakeHeadIdx = this.world.snake_head_idx();

    console.log(snakeHeadIdx);
  }
  // ...
}
