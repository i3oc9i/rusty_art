//my first shadertoy code: https://www.shadertoy.com/view/DdyXDt citations:
//some noise functions and Brownian motion: frankenburgh, (2016) https://www.shadertoy.com/view/lty3Rt 
// Dave_Hoskins, (2014) https://www.shadertoy.com/view/MdXSzS ; FabriceNeyret2, (2014) https://www.shadertoy.com/view/XdsSRS
const PI: f32 = 3.1415926535897932384626433832795;
struct TimeUniform {
    time: f32,
};
struct Params {
    lambda: f32,
    theta: f32,
    sigma: f32,
    gamma: f32,
    alpha:f32,
    delta: f32,
    eta: f32,
    rho: f32,
    phi: f32,
    psi: f32,
    omega: f32,
    blue: f32,
    noise:f32,
    noise2:f32,
    color:f32,
    fw:f32,
    fh:f32,
    fcx:f32,
    fcy:f32,

};
@group(0) @binding(1)
var<uniform> params: Params;
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn fbmslow(p: vec2<f32>) -> f32 {
    var f: f32 = 0.0;
    var p_mod: vec2<f32> = p; 
    f += 1.001 * length(p_mod);
    p_mod = p_mod * 2.02; 
    f += 2.001 * length(p_mod);
    p_mod = p_mod * 2.03;
    f += 0.1250 * length(p_mod);
    p_mod = p_mod * 0.001;
    f += 0.0625 * length(p_mod);
    return f / 2.9375;
}
fn noise(p: vec2<f32>, iTime: f32) -> f32 {
    return fbmslow(p + iTime * params.gamma);
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let PI: f32 = 3.1415926535897932384626433832795;
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    let lightDirection: vec3<f32> = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let zoomLevel: f32 = params.blue; 
    var uv: vec2<f32> = (FragCoord.xy / resolution) - vec2<f32>(0.5, 0.5);
    let distanceFromCenter: f32 = length(uv) * params.alpha;
    let flow: vec2<f32> = vec2<f32>(noise(uv, u_time.time), noise(uv + vec2<f32>(0.1, 0.1), u_time.time));
    let timeFactor: f32 =params.theta* u_time.time;
    let adjustedTime: f32 = timeFactor + (params.delta + sin(timeFactor)) * 0.1 / (distanceFromCenter + 0.07);
    let sineTime: f32 = sin(adjustedTime);
    let cosineTime: f32 = cos(adjustedTime);
    uv = uv * mat2x2(cosineTime, sineTime, -sineTime, cosineTime);
    uv = uv + flow * params.lambda;
    var baseColor: f32 = 0.0;
    var color1: f32 = params.noise;
    var color2: f32 = params.noise2;
    var color3: f32 = 0.0;
    var point: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var i: i32 = 0; i < 150; i = i + 1) {
        point = params.eta * f32(i) * vec3<f32>(uv, params.fh);
        let stars =  oscillate(0.1,params.color*PI, 20.5, u_time.time);
        point = point + vec3<f32>(0.1, 0.01, -params.fw*PI - sin(timeFactor * 0.1) * stars);
        for (var j: i32 = 0; j < 11; j = j + 1) {
            point = abs(point) / dot(point, point) - params.psi;
        }
        let pointIntensity: f32 = dot(point, point) *params.omega;
        color1 = color1 + pointIntensity * (params.rho+ sin(distanceFromCenter * 13.0 + 3.5 - timeFactor * 2.0));
        color2 = color2 + pointIntensity * (params.phi + sin(distanceFromCenter * 13.5 + 2.2 - timeFactor * 3.0));
        color3 = color3 + pointIntensity * (2.4 + sin(distanceFromCenter * 14.5 + 1.5 - timeFactor * 2.5));
    }
    baseColor = (3.1 / (params.fcx + zoomLevel)) * length(vec2<f32>(point.x, point.y)) * params.sigma;
    color1 = color1 * 0.5;
    color2 = color2 * 0.5; 
    color3 = smoothstep(params.fcy, 0.0, distanceFromCenter);
    color3 = color3 * 0.3;
    var direction: vec3<f32> = normalize(vec3<f32>(uv, 0.0)); 
    let sundot: f32 = dot(lightDirection, direction);
    var finalColor: vec3<f32> = vec3<f32>(baseColor, (color1 + baseColor) * 0.25, color2);
    finalColor = finalColor + color3 * 2.9;
    finalColor.g = finalColor.g + color3 * 0.45;
    finalColor = applyGamma(finalColor, 0.5);
    return vec4<f32>(finalColor, 1.0);
}