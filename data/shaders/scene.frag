#version 300 es
precision highp float;
// Passes (the core says which): 0 solid, lit in Espiochum's style: a low
// sun with continuous shading, a sky fill and a ground bounce, weathered
// materials, haze; 1 ink (zero width in this style); 2 tint, see-through shapes
// (sight cones, the landing zone's glow); 4 a portal's window cleared to
// the sky (3 and 5 write no colour).
// Styles (crates/core/src/world.rs): 0 plain, 1 concrete panels,
// 2 corrugated metal, 3 camouflage, 4 glow, 5 planks, 6 ground,
// 7 active camo, 8 portal swirl.
in vec3 vWorld;
in vec3 vNormal;
in vec4 vColor;
in vec3 vLocal;
in vec3 vLocalNormal;
in vec3 vSize;
uniform vec3 uEye;
// the view through a portal cuts away what lies behind the far portal's face
uniform vec4 uClip;
uniform vec3 uLightDir;
uniform vec3 uSky;
uniform vec3 uFog;
uniform vec2 uFogRange;
uniform vec3 uLight;
uniform vec3 uInk;
uniform float uPass;
out vec4 outColor;

float hash(vec2 p) {
  return fract(sin(dot(p, vec2(12.9898, 78.233))) * 43758.5453);
}

float blotch(vec3 p) {
  vec2 q = p.xz * 2.3 + p.y * 1.7;
  vec2 i = floor(q);
  vec2 f = fract(q);
  f = f * f * (3.0 - 2.0 * f);
  float a = mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x);
  float b = mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x);
  return mix(a, b, f.y);
}

float vnoise(vec3 p) {
  vec3 i = floor(p);
  vec3 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  float a = mix(hash(i.xy + i.z * 17.0), hash(i.xy + vec2(1.0, 0.0) + i.z * 17.0), f.x);
  float b = mix(hash(i.xy + vec2(0.0, 1.0) + i.z * 17.0), hash(i.xy + vec2(1.0, 1.0) + i.z * 17.0), f.x);
  float c = mix(hash(i.xy + (i.z + 1.0) * 17.0), hash(i.xy + vec2(1.0, 0.0) + (i.z + 1.0) * 17.0), f.x);
  float d = mix(hash(i.xy + vec2(0.0, 1.0) + (i.z + 1.0) * 17.0), hash(i.xy + vec2(1.0, 1.0) + (i.z + 1.0) * 17.0), f.x);
  return mix(mix(a, b, f.y), mix(c, d, f.y), f.z);
}

float fbm(vec3 p) {
  float s = 0.0;
  float w = 0.5;
  for (int k = 0; k < 4; k++) {
    s += w * vnoise(p);
    p *= 2.03;
    w *= 0.5;
  }
  return s;
}

void main() {
  if (dot(uClip.xyz, vWorld) < uClip.w) discard;
  float d = length(vWorld - uEye);
  float f = clamp((d - uFogRange.x) / (uFogRange.y - uFogRange.x), 0.0, 1.0);
  f = f * f * (3.0 - 2.0 * f);
  if (uPass > 3.5 && uPass < 4.5) {
    // a portal's window, cleared to the sky before the view through it
    outColor = vec4(uSky, 1.0);
    return;
  }
  if (uPass > 1.5) {
    outColor = vec4(mix(vColor.rgb, uFog, f), vColor.a);
    return;
  }
  float style = floor(vColor.a + 0.001);
  if (style > 6.5 && style < 7.5) {
    // active camo: most of her is not drawn at all
    vec2 s = floor(gl_FragCoord.xy);
    if (mod(s.x + s.y * 3.0, 5.0) > 0.5) discard;
  }
  if (uPass > 0.5) {
    outColor = vec4(mix(uInk, uFog, f * 0.85), 1.0);
    return;
  }
  vec3 n = normalize(vNormal);
  vec3 base = pow(vColor.rgb, vec3(2.2));
  vec3 ln = vLocalNormal;
  bool side = abs(ln.y) < 0.5;
  float u = abs(ln.x) > abs(ln.z) ? (vLocal.z + 0.5) * vSize.z : (vLocal.x + 0.5) * vSize.x;
  float v = (vLocal.y + 0.5) * vSize.y;
  float h = vWorld.y;
  // weathering shared by every built surface: broad stains, fine grain, and
  // dirt that climbs the bottom of walls
  float broad = fbm(vWorld * 0.35);
  float fine = fbm(vWorld * 6.0);
  float grime = side ? (1.0 - smoothstep(0.0, 1.4, v)) * 0.35 : 0.0;
  if (style > 0.5 && style < 1.5) {
    // concrete panels: seams every 3 m, water stains running down
    if (side && (fract(u / 3.0) < 0.012 || abs(v - 1.2) < 0.02)) base *= 0.7;
    float streak = side ? smoothstep(0.55, 0.8, fbm(vec3(u * 2.2, v * 0.25, 0.0))) : 0.0;
    base *= (0.82 + 0.3 * broad) * (0.92 + 0.12 * fine) * (1.0 - 0.25 * streak);
  } else if (style > 1.5 && style < 2.5) {
    // corrugated sheet: ribs that catch the light, rust running from the top
    float rib = sin(u * 6.2831 * 4.0);
    n = normalize(n + (side ? 0.25 * rib : 0.0) * normalize(cross(n, vec3(0.0, 1.0, 0.0)) + 1e-4));
    float rust = side ? smoothstep(0.6, 0.85, fbm(vec3(u * 3.0, v * 0.18 + 7.0, 1.0))) * smoothstep(0.0, 2.5, v) : 0.0;
    base = mix(base * (0.85 + 0.25 * broad), vec3(0.24, 0.12, 0.06), rust * 0.6);
  } else if (style > 2.5 && style < 3.5) {
    float b = blotch(vWorld * 1.0);
    if (b > 0.55) base *= 0.5;
    else if (b < 0.25) base *= 1.15;
  } else if (style > 4.5 && style < 5.5) {
    // timber: boards and grain
    if (side && fract(v / 0.3) < 0.06) base *= 0.6;
    base *= 0.8 + 0.3 * fbm(vec3(u * 0.5, v * 12.0, 3.0));
  } else if (style > 5.5 && style < 6.5) {
    // ground: patches at several scales
    base *= 0.72 + 0.32 * broad + 0.16 * fine + 0.1 * fbm(vWorld * 1.7);
  } else {
    base *= 0.9 + 0.15 * broad;
  }
  base *= 1.0 - grime;
  if (style > 7.5) {
    // the portal: rings that brighten toward the rim
    float r = length(vLocal.xz) * 2.0;
    float ring = step(0.5, fract(r * 4.0 - atan(vLocal.z, vLocal.x) * 0.3));
    outColor = vec4(mix(base * 0.35, base * 1.25, ring * r + r * 0.5), 1.0);
    return;
  }
  if (style > 3.5 && style < 4.5) {
    outColor = vec4(mix(base * 1.15, uFog, f * 0.6), 1.0);
    return;
  }
  float sun = max(dot(n, uLightDir), 0.0);
  float up = 0.5 + 0.5 * n.y;
  vec3 sky = pow(uSky, vec3(2.2));
  vec3 sunCol = pow(uLight, vec3(2.2));
  vec3 lit = base * (sunCol * sun * 1.05 + sky * 0.5 * up + sunCol * 0.14 * (1.0 - up));
  vec3 fog = pow(uFog, vec3(2.2));
  vec3 col = mix(lit, fog, f);
  outColor = vec4(pow(col, vec3(1.0 / 2.2)), 1.0);
}
