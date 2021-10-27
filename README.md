#High-Level Architecture

#Scene Creation


#Converting Objects to Triangles

#Triangle Processing
This function is called once per frame.
For every vertex of every triangle of every object in the scene,
call the vertex shader, which returns the 4D (X, Y, Z, W)
clip-space coordinates of the vertex.
Then transform the clip coordinates to normalized device
coordinates (NDC) by dividing clip-space X,Y, and Z
by the clip-space W. If at least one NDC
coordinate doesn't belong to [-1:1) then the vertex is not visible and 
hence is clipped.
For every visible vertex we transform NDC coordinates to screen
coordinates by multiplying the ViewPort matrix (4x4) by NDC.
If none of the vertices of a triangle are clipped, we pass it on to
Tiler.

##Vertex Shader
Vertex shader is created by a user to customize vertex processing.
It is supplied as a function pointer.
##Tiler
This function finds out which triangle belongs to
which tile, and creates a list of triangles for every tile.
1. Find out the bounding box of the screen, and clip the triangle to it
2. Calculate barycentric coordinates of the triangle
3. For each tile:
   1. Evaluate barycentric coords in all the four corners of the tile
   2. If the barycentric coordinate is negative in all the four corners,
   the triangle is outside the tile. See explanation here:
   http://forum.devmaster.net/t/advanced-rasterization/6145
   3. If the triangle is inside the tile, add it to the respective list
##Building a BVH-Tree
[Ray-tracing only] This function builds the BVH-Tree acceleration
structure to speed up ray tracing.

#Tile Processing: Rasterization
This function is called for each tile in the frame.
At first, we allocate a local framebuffer and a local Z-buffer with the
size of the tile, and optionally preload them from the 
respective global buffers.
>For both the local framebuffer and the local Z-buffer we may
optionally add a sibling buffer, so that they act as "ping-pong" buffers,
which allows us working with one of them while preloading/offloading
> the other one from/to the respective global buffer in the
> background. Typically, this improves performance
only if DMA is available, which is not the case for a general-purpose
CPU.

Then, for each triangle of the tile we do the following:
1. find out the bounding box
2. calculate barycentric coordinates of the triangle
3. for every pixel of the bounding box:
   1. determine if the pixel is inside the triangle or not
   2. if it is inside, interpolate vertex attributes, 
   such as depth, normal, texture coordinates, etc.
   3. determine visibility using Z-buffer and the interpolated Z
   4. if the pixel is visible:
      1. call the fragment shader with
      the remaining interpolated attributes as its arguments
      2. save the output of the fragment shader to the local
      tile buffer
After processing all the triangles of the tile, flush the tile
buffer to the frame buffer, and optionally flush the local Z-buffer
to the global Z-buffer.
##Fragment Shader
Fragment shader is created by a user to customize fragment processing.
It is called for each fragment of the tile. In the corner case where
the fragment size is one pixel, it would become a pixel shader.
It is supplied as a function pointer. It takes interpolated vertex
attributes as its arguments, and returns the color of the respective
fragment.

#Tile Processing: Ray Tracing
This function is called for each tile in the frame. Then we cast a
ray from the camera through each pixel of the tile (we call this ray
the primary ray), and use the
ray tracing algorithm to find out if this ray intersects any of the
objects in the scene, and if it does then which object is the closest
to the camera. We have to use acceleration structures such as a BVH Tree
to speed up the process. After we have found the point of intersection,
we may recursively cast additional rays
from the intersection point towards other objects in the scene.
Although the processing of primary rays is inherently parallel,
it is very hard to implement it using SIMD (unlike rasterization, which
is SIMD-friendly). So we expect ray tracing to be much slower.