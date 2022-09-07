import init, { World } from "snake-rust";

init().then(() => {
  const CELL_SIZE = 10;
  const world = World.new();
  const width = world.width();
  const canvas = document.getElementById("game");
  const ctx = canvas.getContext("2d");
  canvas.width = width * CELL_SIZE;
  canvas.height = width * CELL_SIZE;

  function drawWorld() {
    ctx.beginPath();

    for (let i = 0; i < width + 1; i++) {
      ctx.moveTo(i * CELL_SIZE, 0);
      ctx.lineTo(i * CELL_SIZE, width * CELL_SIZE);
    }

    for (let i = 0; i < width + 1; i++) {
      ctx.moveTo(0, i * CELL_SIZE);
      ctx.lineTo(width * CELL_SIZE, i * CELL_SIZE);
    }
    ctx.stroke();
  }

  drawWorld();
});
