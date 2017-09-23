// Rectangle vertex shader
#version 150 core

in vec4 a_Pos;
in vec2 a_Uv;
out vec2 v_Uv;

uniform mat4 u_Model;
uniform mat4 u_View;
uniform mat4 u_Proj;

void main() {
  v_Uv = a_Uv;
  gl_Position = u_Proj * u_View * u_Model * a_Pos;
}
