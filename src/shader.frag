#version 440 core

in vec4 gl_FragCoord;

out vec4 color;

uniform unsigned int u_height;
uniform unsigned int u_width;

#define LIM 100

int iter() {
  /* Normalize coordinates to obtain c. */
  float wh = float(u_width) / float(u_height);
  float m = min(float(u_width), float(u_height));
  float c_re = gl_FragCoord.x / (0.5 * m) - wh;
  float c_im = gl_FragCoord.y / (0.5 * m) - 1;

  /* Iteration starts at 0. */
  float re = 0.0;
  float im = 0.0;

  int N;
  for (N = 0; N < LIM; N++) {
    float tmp = re;
    re = (re * re - im * im) + c_re;
    im = 2.0 * tmp * im + c_im;

    /* D = squared distance */
    float D = re * re + im * im;
    if (D > 4) {
      break;
    }

  }

  return N;
}

void main() {
  int N = iter();

  float x = N == LIM ? 0.0 : float(N) / float(LIM);
  color = vec4(x, 0.5 * x, 0.0, 1.0);
}
