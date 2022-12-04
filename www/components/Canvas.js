export default class Canvas {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.width = canvas.width;
    this.height = canvas.height;
    this.cellSize = 10;

    return this;
  }

  draw() {
    this.ctx.beginPath();

    for (let i = 0; i < this.width + 1; i++) {
      this.ctx.moveTo(i * this.cellSize, 0);
      this.ctx.lineTo(i * this.cellSize, this.width * this.cellSize);
    }

    for (let i = 0; i < this.width + 1; i++) {
      this.ctx.moveTo(0, i * this.cellSize);
      this.ctx.lineTo(this.width * this.cellSize, i * this.cellSize);
    }
    this.ctx.stroke();
  }
}
