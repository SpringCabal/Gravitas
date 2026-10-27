#version 330 compatibility
layout(location = 0) in vec2 corner;
layout(location = 1) in vec4 originAge;
layout(location = 2) in vec4 directionLife;
layout(location = 3) in vec4 forceDelay;
layout(location = 4) in vec4 sizeRotation;
layout(location = 5) in vec4 exponentsPalette;
layout(location = 6) in vec4 animationStrengthHeat;
layout(location = 7) in vec4 scaleColors;
uniform vec4 colors[128];
uniform float ageFrame;
uniform int distortion;
uniform float time;
varying vec2 noiseCoord;
varying vec2 radial;
varying float strength;
varying float heat;

void main() {
    float age = ageFrame - originAge.w;
    float life = clamp((age - forceDelay.w) / directionLife.w, 0.0, 1.0);
    if (exponentsPalette.w < 0.0 || age <= forceDelay.w || age > forceDelay.w + directionLife.w) {
        gl_Position = vec4(-2000.0); return;
    }
    vec3 curve = vec3(1.0) - pow(vec3(1.0 - life), abs(exponentsPalette.xyz));
    if (exponentsPalette.y < 0.0) curve.y = 1.0 - curve.y;
    if (exponentsPalette.z < 0.0) curve.z = 1.0 - curve.z;
    vec3 position = originAge.xyz + directionLife.xyz * curve.x + forceDelay.xyz * (1.0 - pow(1.0 - life, abs(scaleColors.z)));
    float size = sizeRotation.x + sizeRotation.y * curve.y;
    float angle = sizeRotation.z + sizeRotation.w * curve.z;
    if (distortion != 0) angle = -angle;
    mat2 rotation = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
    vec4 eye = gl_ModelViewMatrix * vec4(position, 1.0);
    eye.xy += rotation * ((corner - 0.5) * size);
    gl_Position = gl_ProjectionMatrix * eye;
    gl_ClipVertex = eye;
    float colorPosition = life * scaleColors.y;
    int first = int(colorPosition);
    int last = int(min(float(first + 1), scaleColors.y));
    int offset = int(exponentsPalette.w);
    gl_FrontColor = mix(colors[offset + first], colors[offset + last], fract(colorPosition));
    gl_BackColor = gl_FrontColor;
    gl_TexCoord[0] = vec4(corner, 0.0, 1.0);
    noiseCoord = (corner + animationStrengthHeat.xy * time * scaleColors.w * 0.01) * scaleColors.x;
    radial = corner * 2.0 - 1.0;
    strength = animationStrengthHeat.z * (1.0 - life) * 0.01;
    heat = animationStrengthHeat.w;
}
