class State {

    render 
    isPaused 
    animationId 

    constructor(isPaused , render) {
        this.isPaused = isPaused;
        this.render = render;
    }

    pause = () => {
        cancelAnimationFrame(this.animationId);
        this.animationId = null;
    };
}

export default State

