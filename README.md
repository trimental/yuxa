# Yuxa

Yuxa is an upcoming cross platform window management and graphics toolkit built on [Winit](https://github.com/tomaka/winit) that aims to provide clean/simple drawing abstractions.

# Aims

* Clean and simple buffer based drawing abstractions
* Only support desktop platforms (for the moment)
* Try for as much pure rust as possible

# Order of platforms

Supporting all desktop platforms is a goal of Yuxa however due to the main developer ([trimental](https://github.com/trimental))'s
time constrants and personal setup, some platforms will tend be more supported then others. Below is a general order of support.

Wayland (Linux) -> X11(Linux) -> Windows -> Macos

# Why build upon winit?

[Winit](https://github.com/tomaka/winit) is a great rust library that provides cross platform window management and is well maintained however it lacks any graphics APIs for
drawing to its windows. Yuxa provides these framebuffer based drawing APIs.

Another similar crate is [Glutin](https://github.com/tomaka/glutin) that also builds upon [Winit](https://github.com/tomaka/winit) to provide drawing APIs by way of opengl.
Opengl is a uniform and well supported cross platform graphics API however sometimes simpler graphical APIs (CPU framebased ones) can be great when you don't need the full 
power of something like opengl.
