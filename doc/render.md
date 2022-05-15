# Render prototype

## Features
- Render to different layers.
- Mixing layers for presentation.
- Colored mesh.
- Textured mesh.
- Fixed mesh and texture buffers.
- Scene-based render.

## Resource management
- Asynchronous resources loading.
- Possibility to check loading state of each resource.
- Load batches of resources with fixed size for each frame.
- Resource know how to reload self in case of engine reset.
- Resource may become unloaded, in case of, for example, render device changed.

## Future
- Size of resources batch depends on GPU performance.