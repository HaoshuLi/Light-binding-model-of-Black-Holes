# Light-binding-model-of-Black-Holes

This light binding model of black holes is based on the theory of general relativity.
It is simply a ray tracer, using the fact that the equation of the light rays is the equation of the null geodesics.

This project is totoaly written in the Rust programming language.

I have not finish this project yet. However, I have finished a part of this which is the case of Schwarschild Black Hole. According to spheric symmetry, we have only one free degree in this case. I put my code of this case in 'Schwarschild' branch.

Some quote on my code:
1. The file 'ode.rs' is an ODE solver using Runge Kutta method. We need an ODE solver here since the equation of light is an ODE;
2. The file 'equation.rs' is the specific ODE corresponding to equation of null geodesics near a Schwarschild Black Hole.
3. The file 'main.rs' transforms the original background to the result image, which represents what we see with the existence of a Schwarschild Black Hole.

To build this code, just run
$ cargo build
To run the generated program, put an image in the current directory and run
$ cargo run
, you'll see the final image in the same directory.
