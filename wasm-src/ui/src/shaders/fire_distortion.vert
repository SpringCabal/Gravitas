varying vec2 noiseCoord;
varying vec2 radial;
varying float strength;
varying float heat;

uniform float time;

void main(void) {
    gl_Position = gl_ProjectionMatrix * gl_ModelViewMatrix * gl_Vertex;
    noiseCoord = gl_MultiTexCoord0.st * gl_MultiTexCoord2.z;
    noiseCoord += vec2(time * 0.01, -time * 0.013);
    radial = gl_MultiTexCoord1.st;
    strength = gl_MultiTexCoord2.x * 0.01;
    heat = gl_MultiTexCoord2.y;
}
