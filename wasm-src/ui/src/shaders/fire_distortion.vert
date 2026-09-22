varying vec2 noiseCoord;
varying vec2 radial;
varying float strength;
varying float heat;

uniform float time;

void main(void) {
    gl_Position = gl_ProjectionMatrix * gl_ModelViewMatrix * gl_Vertex;
    // JitterParticles2 animates each particle with its own integer direction.
    // The previous WASM shader used one global diagonal scroll, which made all
    // heat particles repeat in sync and made the distortion look crisp.
    noiseCoord = gl_MultiTexCoord0.st + gl_MultiTexCoord0.pq * time * 0.01;
    noiseCoord *= gl_MultiTexCoord2.z;
    radial = gl_MultiTexCoord1.st;
    strength = gl_MultiTexCoord2.x * 0.01;
    heat = gl_MultiTexCoord2.y;
}
