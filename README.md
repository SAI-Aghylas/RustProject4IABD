# RustProject4IABD
Projet 2 Rust: retrycer  
Group members : SAI Aghylas | SARNI Juba Saadi | OUMEDDAH Merzouk  
Group nember : 6  
This is a Rust project, which contains a raytracer implementation  
It generate an PPM image format which we can modify and manipulate whith our previous Rust project (Lib PPM)  

## To run the program
cd raytracer  
cd app  
  
To generate a black & white image with sphere and cube shapes :  
cargo run --release scenes/black_white.json ../black_white_shapes.ppm  
  
To generate sphere with color :  
cargo run --release scenes/color_sphere.json ../color_sphere.ppm
  
To generate cube with color :  
cargo run --release scenes/color_cube.json ../color_cube.ppm  
  
To generate sphere and cube with color :  
cargo run --release scenes/color_sphere_cube.json ../color_sphere_cube.ppm

## Qestions : 

#### Is OpenGL use raytracing to render 3D scene? How does this work? What the benefits of both methods? :
No openGl does not use raytracing, but uses rasterization. Rasterizing is very often used to render 3D graphics and this in real time such as those used in video games. Resterization transforms a 3D scene into a 2D scene and this in real time. The way it works is that the rasterizer checks the set of triangles that make up the 3D scene and determines which ones will be visible or not from the current perspective. After that there is an analysis on the light sources as well as other details related to the environment for adding light and colors to the pixels of each triangle. In conclusion, what there is to remember is that rasterization with OpenGL is much faster but remains less realistic unlike raytracing which is quite slow but much more realistic

#### What is shadering? How is it computed? Why smart people do raytracing in shaders?:
A shader is a small program that runs on a graphics card or GPU that manipulates a 3D scene during the rendering pipeline phase before the image is printed on the screen. With shaders we have the possibility of creating a range of rendering effects and this quickly enough for applications using them in real time such as video games. the papeline phase is made up of four stages which are: 
-Model transform.  
-View transform.  
-Project transformation.  
-viewport transform. 
We have two types of shaders:  
-Vertex shader can manipulate the attributes of vertices which are the corner points of a polygon.  
-Fragment shader is computing the color of each pixel.  
The reason why smart people do raytracing in shaders may be because it's the best way to do raytracing with parallelism computing and harness the power of GPUs
