uniform sampler2D noiseMap;

varying vec2 noiseCoord;
varying vec2 radial;
varying float strength;
varying float heat;

void main(void) {
    vec2 noiseVec = (texture2D(noiseMap, noiseCoord).xy - 0.50) * strength;
    float mask = smoothstep(1.0, 0.0, dot(radial, radial));
    noiseVec *= mask;
    gl_FragColor = vec4(noiseVec * 0.5 + 0.5, length(noiseVec) * heat, mask);
}
