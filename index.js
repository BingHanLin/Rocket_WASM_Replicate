// Fetch and instantiate our wasm module
import init, {
    update,
    toggle_shoot,
    toggle_boost,
    toggle_turn_left,
    toggle_turn_right,
    resize,
    draw,
} from "./pkg/wasm_rocket_game.js";

let canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");

let res;

// let res = {
//     player: document.createElement("canvas"),
//     enemy: document.createElement("canvas"),
//     bullet: document.createElement("canvas"),
//     particle: document.createElement("canvas"),
// };

export function clear_screen() {
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

export function draw_player(x, y, angle) {
    ctx.translate(x, y);
    ctx.rotate(angle);
    ctx.translate(0, -8);
    ctx.drawImage(res.player, 0, 0);
    ctx.setTransform(1, 0, 0, 1, 0, 0);

    ctx.fillStyle = "black";
    // ctx.fillRect(x - 17, y - 12, 4, 4);
}

export function draw_enemy(x, y) {
    ctx.drawImage(res.enemy, x - 10, y - 10);
}

export function draw_bullet(x, y) {
    ctx.drawImage(res.bullet, x - 3, y - 3);
}

export function draw_particle(x, y, radius) {
    ctx.drawImage(res.particle, x - radius, y - radius, 2 * radius, 2 * radius);
}

export function draw_score(x) {
    ctx.fillStyle = "orange";
    ctx.textBaseline = "top";
    ctx.font = "20px sans-serif";
    ctx.fillText("Score: " + x, 10, 10);
}

function init_resources() {
    let init_res = {
        player: document.createElement("canvas"),
        enemy: document.createElement("canvas"),
        bullet: document.createElement("canvas"),
        particle: document.createElement("canvas"),
    };

    // Particle
    init_res.particle.width = 20;
    init_res.particle.height = 20;
    let pCtx = init_res.particle.getContext("2d");
    pCtx.fillStyle = "darkviolet";
    pCtx.beginPath();
    pCtx.arc(10, 10, 10, 0, 2 * Math.PI);
    pCtx.fill();

    // Bullet
    init_res.bullet.width = 6;
    init_res.bullet.height = 6;
    let bCtx = init_res.bullet.getContext("2d");
    bCtx.fillStyle = "blue";
    bCtx.beginPath();
    bCtx.arc(3, 3, 3, 0, 2 * Math.PI);
    bCtx.fill();

    // Enemy
    init_res.enemy.width = 20;
    init_res.enemy.height = 20;
    let eCtx = init_res.enemy.getContext("2d");
    eCtx.fillStyle = "yellow";
    eCtx.beginPath();
    eCtx.arc(10, 10, 10, 0, 2 * Math.PI);
    eCtx.fill();

    // Player
    init_res.player.width = 20;
    init_res.player.height = 16;
    let plCtx = init_res.player.getContext("2d");
    plCtx.fillStyle = "red";
    plCtx.beginPath();
    plCtx.lineTo(20, 8);
    plCtx.lineTo(0, 16);
    plCtx.lineTo(0, 0);
    plCtx.fill();

    res = init_res;
}

async function run() {
    await init();

    console.log("Wasm module loaded..!");

    // Input processing
    function processKey(key, b) {
        switch (key) {
            case "ArrowLeft":
                toggle_turn_left(b);
                console.log("left");
                break;
            case "ArrowRight":
                toggle_turn_right(b);
                console.log("right");
                break;
            case "ArrowUp":
                toggle_boost(b);
                console.log("up");
                break;
            case " ":
                toggle_shoot(b);
                console.log("space");
                break;
        }
    }

    init_resources();

    document.addEventListener("keydown", (e) => processKey(e.key, true));
    document.addEventListener("keyup", (e) => processKey(e.key, false));

    // Resizing
    function resizeCanvas() {
        // We make the canvas somewhat smaller to get some zooming
        canvas.width = window.innerWidth * 0.8;
        canvas.height = window.innerHeight * 0.8;
        resize(canvas.width, canvas.height);
    }

    window.addEventListener("resize", () => {
        resizeCanvas();
    });

    // Game loop
    let start = null;
    let prevTimestamp = null;
    let drawAndUpdate = (timestamp) => {
        // Initialization
        if (!prevTimestamp) {
            start = timestamp;
            prevTimestamp = timestamp;
            requestAnimationFrame(drawAndUpdate);
            return;
        }

        // Update and draw
        let progress = (timestamp - prevTimestamp) / 1000;
        update(progress);
        draw();

        // Some bookkeeping
        prevTimestamp = timestamp;
        requestAnimationFrame(drawAndUpdate);
    };

    resizeCanvas();
    drawAndUpdate();
}

run();
