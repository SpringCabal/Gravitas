uniform sampler2D noiseMap;

varying vec2 noiseCoord;
varying vec2 radial;
varying float strength;
varying float heat;

void main(void) {
    vec2 noiseVec = (texture2D(noiseMap, noiseCoord).xy - 0.50) * strength;
    float mask = smoothstep(1.0, 0.0, dot(radial, radial));
    noiseVec *= mask;
    // Keep the signed displacement in the floating-point target.  LUPS uses
    // GL_ONE/GL_ONE while drawing the distortion layers, so overlapping heat
    // particles add naturally instead of replacing one another.
    gl_FragColor = vec4(noiseVec, length(noiseVec) * heat, mask);
}
