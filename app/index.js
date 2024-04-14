import __wbg_init, { Universe } from "../pkg/wasm_game_of_life.js";
import { drawGrid, drawCells, createCanvas } from "./grid.js";

const wasm = await __wbg_init();
const memory = wasm.memory;

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = createCanvas(height, width);

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();

    drawGrid(ctx , height , width);
    drawCells(ctx, memory , height , width , universe);

    requestAnimationFrame(renderLoop);
};

// Start the rendering loop
renderLoop();
