class ButtonComponent {
    constructor(id, text, onClick) {
        this.id = id;
        this.text = text;
        this.onClick = onClick;
    }

    init() {
        const button = document.getElementById(this.id);
        button.textContent = this.text;
        button.addEventListener('click', this.onClick);
        return button;
    }
}

export default ButtonComponent;

