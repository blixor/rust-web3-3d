import init from './pkg/web_3d.js';
import {Universe} from './pkg/web_3d.js';

let canvas = document.getElementById("canvas");
let gl = canvas.getContext('webgl');


//     _               _               
// ___| |__   __ _  __| | ___ _ __ ___ 
/// __| '_ \ / _` |/ _` |/ _ \ '__/ __|
//\__ \ | | | (_| | (_| |  __/ |  \__ \
//|___/_| |_|\__,_|\__,_|\___|_|  |___/

 let fragmentShaderCode = `
precision mediump float;
varying vec4 v_color;
void main(void) {
        gl_FragColor = v_color;
}
`                                    
let vertexShaderCode = `
attribute vec4 coordinates;
uniform mat4 projection;

attribute vec3 color;
varying vec4 v_color;
void main() {
        gl_Position = projection * coordinates;
        v_color = vec4(color, 0.5); 
}
`



// setup color
gl.clearColor(0.,0.,0., 0);
gl.clear(gl.COLOR_BUFFER_BIT);
gl.enable(gl.CULL_FACE);
gl.enable(gl.DEPTH_TEST); 


// SHADERS
var vertShader = gl.createShader(gl.VERTEX_SHADER);
gl.shaderSource(vertShader, vertexShaderCode);
gl.compileShader(vertShader);

var fragShader = gl.createShader(gl.FRAGMENT_SHADER);
gl.shaderSource(fragShader, fragmentShaderCode);
gl.compileShader(fragShader);
var shaderProgram = gl.createProgram();
gl.attachShader(shaderProgram, vertShader); 
gl.attachShader(shaderProgram, fragShader);
gl.linkProgram(shaderProgram);
gl.useProgram(shaderProgram);



// setup WEBGL buffers

var index_buffer = gl.createBuffer();
gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer);

//Get the attribute location	
let coord_loc = gl.getAttribLocation(shaderProgram, "coordinates");
let color_loc = gl.getAttribLocation(shaderProgram, "color");

let point_buffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, point_buffer);


gl.vertexAttribPointer(coord_loc, 3, gl.FLOAT, false, 
        6 * Float32Array.BYTES_PER_ELEMENT, 
        0
);
gl.enableVertexAttribArray(coord_loc);


gl.vertexAttribPointer(color_loc, 3, gl.FLOAT, false,
        6 * Float32Array.BYTES_PER_ELEMENT,
        3 * Float32Array.BYTES_PER_ELEMENT
);
gl.enableVertexAttribArray(color_loc);

let trans_loc = gl.getUniformLocation(shaderProgram, "projection");






// rust interface for our 3d game
let demo;

// key manager
var pressedKeys = {};
window.onkeyup = function(e) { pressedKeys[e.keyCode] = false; }
window.onkeydown = function(e) { pressedKeys[e.keyCode] = true;}

// time manager
const initialTime = Date.now();
const FPS_THROTTLE = 1000.0 / 20.0;
let lastDrawTime = -1;


//unction game_loop() {
 //       if (pressedKeys[37]) {demo.rotate(0.05)};
 //       if (pressedKeys[39]) {demo.rotate(-0.05)};
 //       if (pressedKeys[40]) {demo.forward(-0.06)};
 //       if (pressedKeys[38]) {demo.forward(0.06)};

 //       // only change position of player, not environment
//}

function universe_loop() {
        // change environment
        //                        left arrow       right arrow      down arrow       up arrow
        demo.update(lastDrawTime, pressedKeys[37], pressedKeys[39], pressedKeys[40], pressedKeys[38]); 
}

function render() {
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
                lastDrawTime = currTime;

                let width = 0.9*window.innerWidth - 25;
                let height = 0.8*window.innerHeight - 30;

                if (width != canvas.width || height != canvas.height){
                        canvas.width = width; canvas.height = height;
                        gl.viewport(0, 0, width, height);
                }

                let t = currTime - initialTime;

                // render without changing environment
                demo.render();
        }


        requestAnimationFrame(render);
}

init().then(() => {demo=new Universe(gl, trans_loc, Date.now())}).then(() => {
        setInterval(universe_loop, FPS_THROTTLE/3);
}).then(render);
