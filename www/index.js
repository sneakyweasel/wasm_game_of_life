import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/quantum_wave_bg";

const CELL_SIZE = 10; // px
const ENTITY_SIZE = 3; // r, g, b values are from 0 to 255.
const GRID_COLOR = "#333";

// Construct the universe, and get its width and height.
const width = 60;
const height = 60;
const universe = Universe.new(width, height);
universe.setup();
console.log(universe);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
    FPS: ${Math.round(fps)}
    `.trim();
  }
};

let animationId = null;

const renderLoop = () => {
  fps.render();

  drawCells();

  universe.step();
  console.log(universe);
  animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
  return animationId === null;
};

const resetButton = document.getElementById("reset");
const playPauseButton = document.getElementById("play-pause");
const stepButton = document.getElementById("step");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶️";
  cancelAnimationFrame(animationId);
  animationId = null;
};

resetButton.addEventListener("click", event => {
  pause();
  universe.reset();
  drawCells();
})

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

stepButton.addEventListener("click", event => {
  universe.step();
  drawCells();
});

const getIndex = (row, column) => {
  return (row * width + column) * ENTITY_SIZE;
};

// Render cells.
const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height * ENTITY_SIZE);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      const r = cells[idx];
      const g = cells[idx + 1];
      const b = cells[idx + 2];

      ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

// Click handler.
canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  universe.toggle_cell(row, col);

  drawCells();
});

// Add keyboard event listeners.
document.addEventListener("keypress", event => {
  event.stopPropagation();
  if (event.key === " ") {
    universe.step();
    drawCells();
  } else if (event.key === "p") {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
  } else if (event.key === "r") {
    universe.reset();
    drawCells();
  }
});

play();
pause();