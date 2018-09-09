#version 150 core

in vec4 a_Pos;
in vec2 a_Uv;

uniform Transform {
    mat4 u_Transform;
};

out vec2 v_Uv;

void main() {
    v_Uv = a_Uv;
    gl_Position = a_Pos * u_Transform;
}
