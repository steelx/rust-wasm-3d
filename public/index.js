const rust = import('../pkg/index.js');

const canvas = document.getElementById('rust_canvas');
const gl = canvas.getContext('webgl', {antialias: true});

rust.then(r => {
    if (!gl) {
        alert("Failed to initialize WebGL");
        return;
    }

    if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0, 0, canvas.width, canvas.height);
    }

    const game_client = new r.GameClient();

    const initial_time = Date.now();
    const FPS = 1000.0 / 30.0;
    let delta_time = -1;// ms
    function game_loop() {
        window.requestAnimationFrame(game_loop);
        const now = Date.now();
        if (now >= delta_time + FPS) {
            delta_time = now;

            let elapsed_time = now - initial_time;
            //Rust render
            game_client.render();
            //Rust update
            game_client.update(elapsed_time, canvas.height, canvas.width);
        }
    }

    //START
    game_loop();
})
.catch(console.error);