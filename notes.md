# Let's try every gui toolkit for rust

Here are the things I want to see:

1. Easily create my own widgets
2. Easily compose widgets within each other
3. Widget's internal layout is not affected by what it's embedded into

What do I try it on? Let's do the same as
[this](https://www.boringcactus.com/2020/08/21/survey-of-rust-gui-libraries.html)
article and do a to-do list. We'll decompose an entry into it's own widget.
We'll add ability to save and load.

## Azul

Oh no, CSS, that's a bad sign.

Requres a shared library. The binary release is windows-only. Ok, let's build it.

```
morj@blackflame:~/projects/gui/azul-example> mkdir lib
morj@blackflame:~/projects/gui/azul-example> git clone https://github.com/fschutt/azul --depth=1
morj@blackflame:~/projects/gui/azul-example> cd azul/azul-dll/
morj@blackflame:~/projects/gui/azul-example/azul/azul-dll> cargo build --release
Finished release [optimized] target(s) in 2m 42s
```

Damn, who are those people copmlaining about rust being slow to build, this is pretty fast still.

> On Linux or Mac the operating system needs the library to be in the LD_PRELOAD path
You fucking what? Well whatever, if the lib is good this can be fixed.

Yay, the hello world doesn't build. Let's see.

Ok, so some of the docs have gone out of date. You don't need to use
LD_PRELOAD, but you need to instruct rust to link with the library with build
script. Also the hello world example I've now taken from another place in the
repo. It runs! And it segfaults on closing the window!

```
Thread 1 "azul-example" received signal SIGSEGV, Segmentation fault.
0x0000000000000008 in ?? ()
Missing separate debuginfos, use: zypper install Mesa-dri-debuginfo-22.2.4-333.2.x86_64 Mesa-libEGL1-debuginfo-22.2.4-333.1.x86_64 Mesa-libglapi0-debuginfo-22.2.4-333.1.x86_64 libLLVM15-debuginfo-15.0.5-1.1.x86_64 libX11-devel-debuginfo-1.8.2-2.1.x86_64 libX11-xcb1-debuginfo-1.8.2-2.1.x86_64 libXau6-debuginfo-1.0.10-1.1.x86_64 libXext6-debuginfo-1.3.5-1.1.x86_64 libbrotlicommon1-debuginfo-1.0.9-1.10.x86_64 libbrotlidec1-debuginfo-1.0.9-1.10.x86_64 libbz2-1-debuginfo-1.0.8-4.7.x86_64 libdrm2-debuginfo-2.4.114-3.1.x86_64 libdrm_amdgpu1-debuginfo-2.4.114-3.1.x86_64 libdrm_nouveau2-debuginfo-2.4.114-3.1.x86_64 libdrm_radeon1-debuginfo-2.4.114-3.1.x86_64 libedit0-debuginfo-20210910.3.1-1.11.x86_64 libexpat1-debuginfo-2.5.0-1.1.x86_64 libffi8-debuginfo-3.4.4-1.1.x86_64 libfreetype6-debuginfo-2.12.1-3.2.x86_64 libgbm1-debuginfo-22.2.4-333.1.x86_64 libgcc_s1-debuginfo-12.2.1+git537-1.1.x86_64 libglvnd-debuginfo-1.5.0-1.1.x86_64 libglvnd-devel-debuginfo-1.5.0-1.1.x86_64 liblzma5-debuginfo-5.2.8-1.1.x86_64 libncurses6-debuginfo-6.3.20221105-30.1.x86_64 libnvidia-egl-wayland1-debuginfo-1.1.11-1.1.x86_64 libpciaccess0-debuginfo-0.17-1.1.x86_64 libpng16-16-debuginfo-1.6.39-1.1.x86_64 libstdc++6-debuginfo-12.2.1+git537-1.1.x86_64 libwayland-server0-debuginfo-1.21.0-1.2.x86_64 libxcb-dri2-0-debuginfo-1.15-1.4.x86_64 libxcb-dri3-0-debuginfo-1.15-1.4.x86_64 libxcb-glx0-debuginfo-1.15-1.4.x86_64 libxcb-present0-debuginfo-1.15-1.4.x86_64 libxcb-render0-debuginfo-1.15-1.4.x86_64 libxcb-shape0-debuginfo-1.15-1.4.x86_64 libxcb-sync1-debuginfo-1.15-1.4.x86_64 libxcb-xfixes0-debuginfo-1.15-1.4.x86_64 libxcb1-debuginfo-1.15-1.4.x86_64 libxml2-2-debuginfo-2.10.3-2.1.x86_64 libxshmfence1-debuginfo-1.3.1-1.1.x86_64 libz1-debuginfo-1.2.12-2.1.x86_64
(gdb) where
#0  0x0000000000000008 in ?? ()
#1  0x000000000000000b in ?? ()
#2  0x0000000000000118 in ?? ()
#3  0x0000000000000000 in ?? ()
```
Ugh, mesa again? I've recently been having problems with steam segfaulting either in mesa, or in pulseaudio, and with telegram segfaulting in mesa. Fucking linux.
