@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(2) @binding(2)
var<uniform> params: Params;
const PI: f32 = 3.14;
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    var uv: vec2<f32> = (FragCoord.xy * 3.0 - resolution) / min(resolution.x, resolution.y);
    uv.x -= 3.0;
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let frequency: f32 = params.lambda;
    for (var j: f32 = 0.0; j < 5.2; j += 1.0) {
    for (var i: f32 = 1.0; i < 5.0; i += 1.0) {
        uv.x = uv.x + (0.2 / (i + j) * sin(i * atan(u_time.time) * 2.0 * uv.y + (u_time.time * params.theta) + i * j));
        uv.y = uv.y + (1.0 / (i + j) * cos(i * 0.6 * uv.x + (u_time.time * params.theta) + i * j));
        var angle: f32 = u_time.time * params.alpha; 
        let rotation: mat2x2<f32> = mat2x2<f32>(cos(angle), -sin(angle), sin(angle), cos(angle));
        uv = rotation * uv;
    }
    var texColor: vec3<f32> = textureSample(tex, tex_sampler, uv).xyz;
    texColor = texColor + textureSample(tex, tex_sampler, uv + vec2<f32>(-0.01, 0.01)).xyz;
    texColor = texColor + textureSample(tex, tex_sampler, uv + vec2<f32>(0.01, 0.01)).xyz;  
    texColor = texColor + textureSample(tex, tex_sampler, uv + vec2<f32>(-0.01, -0.01)).xyz;
    texColor = texColor + textureSample(tex, tex_sampler, uv + vec2<f32>(0.01, -0.01)).xyz; 
    texColor = texColor / 12.0;
    let lenSq: f32 = atan(uv.y); 
    let col1: vec3<f32> = 0.1 + 0.5 * cos(frequency * (1.0 + u_time.time) + vec3<f32>(params.sigma,params.gamma,params.blue) + PI * vec3<f32>(5.0 * lenSq));
    let col2: vec3<f32> = 0.2 + 0.5 * cos(frequency * (1.1 + u_time.time) + PI * vec3<f32>(lenSq));
    let col3: vec3<f32> = 0.2 + 0.4 * cos(frequency * (1.0 + u_time.time) + vec3<f32>(params.blue, params.gamma,params.sigma) + PI * vec3<f32>(2.0 * sin(lenSq)));
    col = col + texColor + (col1 + col2 + col3 + col3);
    }
    col = col / 9.0;
    let bg: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    col = mix(col, bg, 1.0 - smoothstep(0.0, abs(sin(u_time.time * 0.05) * 3.0), length(uv) - 0.1)); 
    col = applyGamma(col,0.1);
    return vec4<f32>(col, 1.0);
}