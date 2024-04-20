import ButtonComponent from './buttonComponent.js';

export function initUI(state) {
    initPauseButton((function() {
        if (state.isPaused) {
            state.render();
            this.textContent = "⏸";
        }
        else {
            state.pause();
            this.textContent = "▶";
        }

        state.isPaused = !state.isPaused;
    }));
}

function initPauseButton(handleClick) {
    const button = new ButtonComponent("play-pause", '⏸', handleClick);
    button.init();
}

