import init,  {Universe} from 'wasm-web-3d';


let canvas = <HTMLCanvasElement>document.getElementById("canvas");
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
uniform float time;

attribute vec3 color;
attribute vec3 ondulation_vec;
attribute float frequency;
attribute float phase;

varying vec4 v_color;

void main() {
        vec4 shift = vec4(cos(time*frequency+phase) * ondulation_vec, 1.0);
        gl_Position = projection * (coordinates + shift);
        v_color = vec4(color, 0.5); 
}
`

console.log(vertexShaderCode);


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



// _            __  __               
//| |__  _   _ / _|/ _| ___ _ __ ___ 
//| '_ \| | | | |_| |_ / _ \ '__/ __|
//| |_) | |_| |  _|  _|  __/ |  \__ \
//|_.__/ \__,_|_| |_|  \___|_|  |___/
                                   
// index buffer
var index_buffer = gl.createBuffer();
gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer);



// point buffer
let point_buffer = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, point_buffer);


let SIZE_VERTEX = 12;
/* The POINT buffer has 12 elements:
 * x y z      |      r g b      |      dx dy dz     | frequency phase other
 * position          color       ondulation vector    other params of point
 */

//Get the attribute location in the programm
let coord_loc = gl.getAttribLocation(shaderProgram, "coordinates");
let color_loc = gl.getAttribLocation(shaderProgram, "color");
let ondulation_loc = gl.getAttribLocation(shaderProgram, "ondulation_vec");
let phase_loc = gl.getAttribLocation(shaderProgram, "phase");
let frequency_loc = gl.getAttribLocation(shaderProgram, "frequency");


function define_float_point_buffer(loc: number, size:number, offset:number) {
        gl.vertexAttribPointer(loc, size, gl.FLOAT, false,
        SIZE_VERTEX * Float32Array.BYTES_PER_ELEMENT, 
        offset      * Float32Array.BYTES_PER_ELEMENT);

        gl.enableVertexAttribArray(loc);
}

define_float_point_buffer(coord_loc, 3, 0);
define_float_point_buffer(color_loc, 3, 3);
define_float_point_buffer(ondulation_loc, 3, 6);
define_float_point_buffer(frequency_loc, 1, 9);
define_float_point_buffer(phase_loc, 1, 10);




// uniforms (values passed to vertex shader)

let trans_loc = gl.getUniformLocation(shaderProgram, "projection");
let time_loc = gl.getUniformLocation(shaderProgram, "time");






// rust interface for our 3d game
let demo: Universe;

// key manager
enum Keys {
    Left = 37,
    Right = 39,
    Down = 40,
    Up = 38,
    Space = 32,
    Shift = 9,
}

var pressedKeys: Record<number, boolean> = {};
window.onkeyup = (e: KeyboardEvent) => { pressedKeys[e.keyCode] = false; }
window.onkeydown = (e: KeyboardEvent) => { pressedKeys[e.keyCode] = true; }

// time manager
const initialTime = Date.now();
const FPS_THROTTLE = 1000.0 / 20.0;
let lastDrawTime = -1;


// controlls : left arrow, right arrow, down arrow, up arrow, space, shift
function get_controlls() : [boolean, boolean, boolean, boolean, boolean, boolean]
{
    return [
        pressedKeys[Keys.Left]  || false,
        pressedKeys[Keys.Right] || false,
        pressedKeys[Keys.Down]  || false,
        pressedKeys[Keys.Up]    || false,
        pressedKeys[Keys.Space] || false,
        pressedKeys[Keys.Shift] || false,
    ];
}

function universe_loop() {
        demo.update(lastDrawTime, ...get_controlls()); 
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
                demo.render(t);
        }


        requestAnimationFrame(render);
}

init().then(() => {demo=new Universe(gl, trans_loc, time_loc, Date.now())}).then(() => {
        setInterval(universe_loop, FPS_THROTTLE/3);
}).then(render);
