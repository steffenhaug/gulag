goals or whatever:
- learn about the rust wrapper by making
  - mandelbrot (simple, input independent)
  - voronoi noise (simple, uses uniform input)
  - game of life (simple cellular automata, manip textures)
  - images from texture (picture and generated)
  - 3d surfaces (maybe some multivariate function plots)
  - text rendering

- next, abstract away the things learned to a framework
  for creating shader pipelines, vertex buffers from normal
  rust structures, loading textures etc etc

- next, try some more advanced things, like cellular automata
  based physics simulations, compute shaders, maybe some
  simple game rendering (proc gen textures or wahtever)

fun projects afterwards:
- plotting software, 3d animated plots


todo:
- make shaders and vertex arrays unrepresentable without a gl context on a type level
- runetime type check of shader uniform locations
- 