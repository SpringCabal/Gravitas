		#define barColor gl_MultiTexCoord1
		#define progress gl_MultiTexCoord2.x
		#define offset   gl_MultiTexCoord2.y

		uniform float glowAlpha;
		 void main()
		{
		// switch between font rendering and bar rendering
		if (gl_FogCoord>0.5f) {
		 gl_TexCoord[0]= gl_TextureMatrix[0]*gl_MultiTexCoord0;
		 gl_FrontColor = gl_Color;
		 gl_Position   = ftransform();
		 return;
		}
		if (gl_Vertex.w==1) {
		 gl_FrontColor = gl_Color;
		 if (gl_Vertex.z>0.0) {
		gl_Vertex.x -= (1.0-progress)*gl_Vertex.z;
		gl_Vertex.z  = 0.0;
		 }
		}else if (gl_Vertex.w==-2 ) {
		 gl_FrontColor = vec4(barColor.rgb,barColor.a*glowAlpha);
	if (gl_Vertex.z>1.0) {
		gl_Vertex.x += progress*gl_Vertex.z;
		gl_Vertex.z  = 0.0;
		 }
		 gl_Vertex.w  = 1;
		}else if (gl_Vertex.w==-3 ) {
		 gl_FrontColor = vec4(barColor.rgb,barColor.a*(glowAlpha/1.5));
	if (gl_Vertex.z>1.0) {
		gl_Vertex.x += progress*gl_Vertex.z;
		gl_Vertex.z  = 0.0;
		 }
		 gl_Vertex.w  = 1;
		}else if (gl_Vertex.w==-4 ) {
		 gl_FrontColor = vec4(barColor.rgb,0);
	if (gl_Vertex.z>1.0) {
		gl_Vertex.x += progress*gl_Vertex.z;
		gl_Vertex.z  = 0.0;
		 }
		 gl_Vertex.w  = 1;
		}else if (gl_Vertex.w>1 ) {
		 if (progress >= 0.92) {		// smooth out because else the bar wil overlap and look ugly at the end point
		gl_FrontColor = vec4(gl_Color[0]+(barColor.r/4),gl_Color[1]+(barColor.g/4),gl_Color[2]+(barColor.b/4),((0.08-(progress-0.92))*12.5)*gl_Color[3]);
			 }else{
		gl_FrontColor = vec4(gl_Color[0]+(barColor.r/4),gl_Color[1]+(barColor.g/4),gl_Color[2]+(barColor.b/4),gl_Color[3]);
		 }
	if (gl_Vertex.z>0.0) {
		gl_Vertex.x -= (1.0-progress)*gl_Vertex.z;
		gl_Vertex.z  = 0.0;
		 }
		 gl_Vertex.w  = 1.0;
		}else{
		 if (gl_Vertex.y>0.0) {
		gl_FrontColor = vec4(barColor.rgb*1.8,barColor.a);
		 }else{
		gl_FrontColor = vec4(barColor.rgb*0.9,barColor.a);
		 }
		 if (gl_Vertex.z>1.0) {
		gl_Vertex.x += progress*gl_Vertex.z;
		gl_Vertex.z  = 0.0;
		 }
		 gl_Vertex.w  = 1.0;
		}
		gl_Vertex.y += offset;
		gl_Position  = gl_ModelViewProjectionMatrix*gl_Vertex;
		 }
