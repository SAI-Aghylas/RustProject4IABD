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

