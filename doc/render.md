# Render prototype

## Features
- Scene-based render.
- Render scenes to window on top of each other.
- Drawing colored mesh.
- Drawing textured mesh.

## Resource management
- Asynchronous resources loading.
- Possibility to check loading state of each resource.
- Load batches of resources with fixed size for each frame.
- Resource know how to reload self in case of engine reset.
- Resource may become unloaded, in case of, for example, render device changed.

## Future
- Size of resources batch depends on GPU performance.
- Render to different targets.
- Use targets as textures.
- Copy targets to textures.
