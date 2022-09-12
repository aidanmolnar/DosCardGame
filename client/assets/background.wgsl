// Translated and adapted from: https://www.shadertoy.com/view/lldyDs

#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.uv = vertex.uv;
    return out;
}

struct Time {
    time_since_startup: f32,
};
@group(2) @binding(0)
var<uniform> time: Time;

let tscale = 0.1;   
let toffset = 10000.;

fn distLine(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let aDotB = clamp(dot(ap,ab) / dot(ab, ab), 0.0, 1.0);
    return length(ap - ab * aDotB);
}

fn drawLine(uv: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let theline = smoothstep(0.014, 0.01, distLine(uv, a, b));
    let dist  = length(b-a);
    return theline * (smoothstep(1.3, 0.8, dist) * 0.5 + smoothstep(0.04, 0.03, abs(dist - 0.75)));
}

fn n21(ii: vec2<f32>) -> f32 {
    var i = ii;
    i += fract(i * vec2<f32>(223.64, 823.12));
    i += vec2<f32>(dot(i, i + 23.14));
    return fract(i.x * i.y);
}

fn n22(i: vec2<f32>) -> vec2<f32>{
    let x = n21(i);
    return vec2<f32>(x, n21(i+x));
}

fn getPoint(id: vec2<f32>, offset: vec2<f32>) -> vec2<f32>{
	return offset + sin(n22(id + offset) * (toffset + time.time_since_startup) *tscale* 1.0) * 0.4;
}

fn layer (uv: vec2<f32>) -> f32 {
    var m = 0.0;
    let t = (toffset + time.time_since_startup) *tscale* 2.0;
   
    let gv = fract(uv) - 0.5;
    let id = floor(uv) - 0.5;
    
    var p: array<vec2<f32>,9>;

    var i = 0;
    for (var y = -1.; y <= 1.; y+= 1.) {
        for (var x = -1.; x <= 1.; x += 1.) {
        	p[i] = getPoint(id, vec2<f32>(x, y));
            i += 1;
        }
    }
    
    for (var i = 0; i < 9; i++) {
    	m += drawLine(gv, p[4], p[i]);
        let sparkle = 1.0 / pow(length(gv - p[i]), 1.5) * 0.005;
        m += sparkle * (sin(5.*t + fract(p[i].x) * 12.23) * 0.4 + 0.6);
    }
    
    m += drawLine(gv, p[1], p[3]);
    m += drawLine(gv, p[1], p[5]);
    m += drawLine(gv, p[7], p[3]);
    m += drawLine(gv, p[7], p[5]);
     
    return m;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = (in.uv - 0.5 * vec2<f32>(1., 1.)) / 1. ;

    var c = sin((toffset + time.time_since_startup) *tscale* 2.0 * vec3<f32>(.934, .324,.768)) * 0.3 + 0.7;
    var col = vec3<f32>(0.,0.,0.);
    c.x += (uv.x + 0.5);
    col += pow(-0.5*uv.y + 0.5, 5.0) * 1. * c;
    
    var m = 0.0;
    // Rotates the background
    //let x = sin(time.time_since_startup * tscale * 0.1);
    //let y = cos(time.time_since_startup * tscale * 0.2);
    //let rotMat = mat2x2<f32>(x, y, -y, x);
    //uv *= rotMat;
    uv.x *= 1.77; // unstretch background horizontally
    uv *= 0.5*(1. + 3.*length(uv)); // stretch radially
    
    for (var i = 0.0; i <= 1.0; i+= 1.0/4.0) {
        //let z = fract(i - time.time_since_startup  * tscale * 0.05); // Zooms the background
        let z = fract(i);
        let size = mix(15.0, .1, z) * 1.50;
        let fade = smoothstep(0.0, 1.0,  z) * smoothstep(1.0, 0.9, z);
        m += layer((size * uv) + i * 10.0 ) * fade;
    }
    
    col += m * c ;

    //col.x -= 0.1;
    col.y -= 0.1;

    col.x *= 0.2;
    col.y *= 0.2;
    col.z *= 0.9;


    //let out = oklab_to_linear_srgb(col+0.01);
    let out = col;

    return vec4<f32>(out.x,out.y,out.z, 1.0);
}