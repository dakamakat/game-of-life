import init, { Universe } from "./pkg/wasm_game_of_life.js";

init().then(() => {
    const pre = document.getElementById("game-of-life-canvas");
    const universe = Universe.new();

    const renderLoop = () => {
        pre.textContent = universe.render();
        universe.tick();

        requestAnimationFrame(renderLoop);
    };

    // Start the rendering loop
    renderLoop();
});
