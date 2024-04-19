class State {

    animationId 

    constructor(isPaused) {
        this.isPaused = isPaused;
    }

    pause = () => {
        cancelAnimationFrame(this.animationId);
        this.animationId = null;
    };

    play = (render) => {
        render();
    };
}

export default State

