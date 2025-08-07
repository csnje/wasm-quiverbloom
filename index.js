const wasm_file = await fetch('./target/wasm32-unknown-unknown/release/wasm_quiverbloom.wasm');
const { instance } = await WebAssembly.instantiateStreaming(wasm_file);

const exports = instance.exports;
const wasmMemory = exports.memory;

const canvas = document.getElementById('image');
const ctx = canvas.getContext('2d');

const numAlgorithms = exports.num_algorithms();

let algorithm = 1;
let width = exports.width(algorithm);
let height = exports.width(algorithm);
let numPoints = exports.num_points(algorithm);
let xptsPtr = exports.create_array(numPoints);
let yptsPtr = exports.create_array(numPoints);
let xpts = new Float64Array(wasmMemory.buffer, xptsPtr, numPoints);
let ypts = new Float64Array(wasmMemory.buffer, yptsPtr, numPoints);

canvas.width = width * 2;
canvas.height = height * 2;
ctx.scale(2, 2);

const select = document.getElementById('algorithm')
for (var i = 1; i <= numAlgorithms; i++) {
    const option = document.createElement('option');
    option.value = `${i}`;
    option.text = `${i}`;
    select.appendChild(option);
}
select.addEventListener('change', (event) => {
    exports.free_array(xptsPtr, numPoints);
    exports.free_array(yptsPtr, numPoints);

    algorithm = parseInt(event.target.value)
    width = exports.width(algorithm);
    height = exports.width(algorithm);
    numPoints = exports.num_points(algorithm);
    xptsPtr = exports.create_array(numPoints);
    yptsPtr = exports.create_array(numPoints);
    xpts = new Float64Array(wasmMemory.buffer, xptsPtr, numPoints);
    ypts = new Float64Array(wasmMemory.buffer, yptsPtr, numPoints);

    canvas.width = width * 2;
    canvas.height = height * 2;
    ctx.scale(2, 2);

    console.log(`selected algorithm ${algorithm}`, t)
})

let t = 0;
// const sleep = ms => new Promise(handler => setTimeout(handler, ms));
async function step() {
    // get points
    exports.frame_points(algorithm, t, xptsPtr, yptsPtr, numPoints)

    // clear canvas
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, canvas.width, canvas.height)

    // draw points
    ctx.fillStyle = 'rgb(255 255 255 / 20%)';
    for (let i = 0; i < numPoints; i++) {
        ctx.beginPath();
        ctx.arc(xpts[i], ypts[i], 1.0, 0, Math.PI);
        ctx.fill();
    }

    t += 0.001;
    if (t >= 1.0) t = 0;
    // await sleep(200);

    requestAnimationFrame(step);
}
requestAnimationFrame(step);
