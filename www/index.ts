import init, { World, Direction, GameStatus } from 'snake_game'
import { rnd } from './utils/rnd';

// 重新打包后，不用重启服务器，也不用重新npm i
init().then(wasm => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 8;
    let snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH)

    const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
    const worldWidth = world.width()

    const points = document.getElementById("points")
    const gameStatus = document.getElementById("game-status")
    const gameControlBtn = document.getElementById("game-control-btn")
    const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas')
    const ctx = canvas.getContext("2d")

    canvas.height = worldWidth * CELL_SIZE
    canvas.width = worldWidth * CELL_SIZE

    gameControlBtn.addEventListener("click", () => {
        const status = world.game_status()

        // if (gameStatus === undefined) {
        if (!status) {
            gameControlBtn.textContent = "Playing..."
            world.start_game()
            play()
        } else {
            location.reload()
        }
    })

    document.addEventListener("keydown", (e) => {
        switch (e.code) {
            // ->
            case "ArrowLeft":
                world.change_snake_dir(Direction.Left)
                break
            case "ArrowRight":
                world.change_snake_dir(Direction.Right)
                break
            case "ArrowUp":
                world.change_snake_dir(Direction.Up)
                break
            case "ArrowDown":
                world.change_snake_dir(Direction.Down)
                break
        }
    })

    function drawWorld() {
        ctx.beginPath()

        // 画x条竖线
        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0)
            ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
        }
        // 画y条横线
        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE * y)
            ctx.lineTo(CELL_SIZE * worldWidth, y * CELL_SIZE)
        }

        ctx.stroke()
    }

    function drawReward() {
        const idx = world.reward_cell()
        const col = idx % worldWidth
        const row = Math.floor(idx / worldWidth)

        ctx.beginPath()
        ctx.fillStyle = "#FF0000"

        ctx.fillRect(
            // x
            col * CELL_SIZE,
            // y
            row * CELL_SIZE,
            // draw x
            CELL_SIZE,
            // draw y
            CELL_SIZE
        )

        ctx.stroke()
        if (idx === 1000) {
            alert("You Won!")
        }
    }

    function drawSnake() {
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length()
        )

        snakeCells
            // 撞到的那个去掉，就显示头部
            // 这样碰撞lose后显示的蛇就有头，而不是全是body
            // .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
            .slice()
            .reverse()
            .forEach((cellIdx, i) => {
                const col = cellIdx % worldWidth
                const row = Math.floor(cellIdx / worldWidth)

                ctx.fillStyle = i === snakeCells.length - 1 ? "#7878db" : "#000000"

                ctx.beginPath()
                ctx.fillRect(
                    // x
                    col * CELL_SIZE,
                    // y
                    row * CELL_SIZE,
                    // draw x
                    CELL_SIZE,
                    // draw y
                    CELL_SIZE
                )
            })

        ctx.stroke()
    }

    function drawGameStatus() {
        gameStatus.textContent = world.game_status_text()
        points.textContent = world.points().toString()
    }

    function paint() {
        drawWorld()
        drawReward()
        drawSnake()
        drawGameStatus()
    }

    function play() {
        const status = world.game_status()

        if (status == GameStatus.Won || status == GameStatus.Lost) {
            gameControlBtn.textContent = "Re-Play"
            return
        }

        const fps = 4
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height)
            world.step()
            paint()
            // 你希望执行一个动画，并且要求浏览器在下次重绘之前调用指定的回调函数更新动画。
            // 该方法需要传入一个回调函数作为参数，该回调函数会在浏览器下一次重绘之前执行。
            requestAnimationFrame(play)
        }, 1000 / fps)
    }

    paint()
})