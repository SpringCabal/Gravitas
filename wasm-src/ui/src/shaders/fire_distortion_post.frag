uniform sampler2D infoTex;
uniform sampler2D screenTex;

void main(void) {
    vec2 texcoord = vec2(gl_TexCoord[0]);
    vec4 info = texture2D(infoTex, texcoord);
    if (info.a <= 0.001) {
        discard;
    }
    vec2 offset = (info.rg - 0.5) * 2.0;
    vec4 scene = texture2D(screenTex, texcoord + offset);
    scene.rgb += info.b;
    gl_FragColor = scene;
}
