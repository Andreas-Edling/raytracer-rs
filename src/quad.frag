#version 330 core

uniform sampler2D tex_sampler;
in vec2 tex_coord;
out vec4 Color;

void main()
{
    //Color = vec4(tex_coord, 0.6f, 1.0f);
    Color = texture(tex_sampler, tex_coord);
}