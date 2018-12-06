import { Universe, Cell } from "tierra";
import { memory } from "tierra/tierra_bg";
const CELL_SIZE = 2; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.getElementById("universe");
const universe = Universe.new();
const length = universe.length();
const width = 200;
const ctx = canvas.getContext('2d');

const getPosition = (idx) => {
  return [Math.floor(idx / width), idx % width];
};

const drawCells = () => {
  const soupPtr = universe.soup_ptr();
  const soup = new Uint8Array(memory.buffer, soupPtr, length);
  ctx.beginPath();
  soup.forEach((instruction, idx) => {
    let [row, col] = getPosition(idx);
    ctx.fillStyle = instruction === 32
      ? DEAD_COLOR
      : ALIVE_COLOR;
    ctx.fillRect(
      col * (CELL_SIZE + 1) + 1,
      row * (CELL_SIZE + 1) + 1,
      CELL_SIZE,
      CELL_SIZE
    );
  });
  ctx.stroke();
}

const renderLoop = () => {
  for (let i = 0; i < 100; i++) {
    universe.tick();
  }
  drawCells();

  requestAnimationFrame(renderLoop);
  ;
};
requestAnimationFrame(renderLoop);


