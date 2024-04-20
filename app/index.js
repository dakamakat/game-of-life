import __wbg_init, { Universe } from "../pkg/wasm_game_of_life.js";
import State from "./state.js";
import { drawGrid, drawCells, createCanvas } from "./grid.js";
import { initUI } from "./ui.js";

const wasm = await __wbg_init();
const memory = wasm.memory;

let state = new State(false);

// Construct the universe, and get its width and height.
const universe = Universe.new(1, 100, 80);

const width = universe.width();
const height = universe.height();

const canvas = createCanvas(height, width);

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();

    drawGrid(ctx, height, width);
    drawCells(ctx, memory, height, width, universe);

    state.animationId = requestAnimationFrame(renderLoop);
};

// Start the rendering loop
state.render = renderLoop;
initUI(state);
state.render();

