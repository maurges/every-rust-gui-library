# Let's try every gui toolkit for rust

Here are the things I want to see:

1. Easily create my own widgets
2. Easily compose widgets within each other
3. Widget's internal layout is not affected by what it's embedded into

What do I try it on? Let's do the same as
[this](https://www.boringcactus.com/2020/08/21/survey-of-rust-gui-libraries.html)
article and do a to-do list. We'll decompose an entry into it's own widget.
We'll add ability to save and load.

Let's go through all the libraries in https://www.areweguiyet.com/#ecosystem
on 2022-12-03 and test them.

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
Ugh, mesa again? I've recently been having problems with steam segfaulting
either in mesa, or in pulseaudio, and with telegram segfaulting in mesa.
Fucking linux.

Ok, so the button doesn't work. Fuck.

Hidpi?

The examples on the website are in C and python, lol.

Why are there both inline styles and CSS? I mean, I thought it was only
historical coincidence in web, but here it's used as well.

Let's try adding a press handler to the whole body, like in the tutorial.

Welp, it doesn't work as well. It seems input handling is broken on linux. Oh that linux with weird IO again.

A shame, but I expected to dislike the approach, so.

```
morj@blackflame:~/projects/gui/azul-example> du -h -d0 .
797M    .
```
DAYUM BOI, chonki librari

## cacao
Macos. I do have a mac, but nah. And it doesn't align with my future goals.

## conrod-core
Unmaintained. Also it's immediate mode.

## core-foundation
Macos again. And the previous one is supposed to worn on GNUStep, what's that?

Ho-ho, gnustep is a reimplementation of nextstep environment for other OS-s. It
uses objective C though and I don't want to set all of this bullshit up right
now, but it might just be an option maybe if I'm bothered.

## CXX-Qt
I like qt, but I remember last time I tried it failed in a lot of ways. Let's
see how this one fares today.

Ok the tutorial fucking skipped setting up the build process. Thankfully there
is a minimal example to copy from. So alright, at least the rust part builds
now.

Pinning self, whoa. This is rare.

I need to register everything from c++. That's not ideal, but not that bad, not
a lot of code to write. The example uses qml, I wonder if it's possible to use
qwidgets instead. But even just qml is pretty good, I myself like qml more.

Oh ok, build is explained in the tutorial, but very late. Alright.

    sudo zypper in libQt5QuickControls2-devel

Only 150 KB, very nice.

Fuck yeah! This was not bad.

So this means I can provide models from rust and use them from qml. Reactivity
and the whole shebang coming for cheap.

Oh yiss, qml is nice. I's so easy to create a simple interface with, with
states, reactivity, layouts. All the basic controls are there, and while they
look weird, they are not that ugly, just very basic.  
Now that layout is done, let's do this with a proper model.

## Dioxus

Apparently uses tauri, which is just a webview. Ah fuck. Well still, let's see.

НУ ЧТО НАРОД ПОГНАЛИ НАХУЙ, ЁбАНЫЙ В РОТ

This is why people complain about rust builds maybe, crates.io refresh takes so
fucking long. And why does it need to refresh, azul was 10 minutes ago.  
Oh god, my memes are done and it's still resolving deltas.  
    error: no matching package named `dioxius` found
????

Ah lol, extra i.

```
Compiling webkit2gtk-sys v0.17.0
error: failed to run custom build command for `gdk-sys v0.15.1`

Caused by:
--- stderr
`"pkg-config" "--libs" "--cflags" "gdk-3.0" "gdk-3.0 >= 3.22"` did not exit successfully: exit status: 1
error: could not find system library 'gdk-3.0' required by the 'gdk-sys' crate
```

Ughhh ok? Is there any way to use qt web view?

Nope.

A shame, but I expected to dislike the approach, again.

## Dominator

What a name.

DOM? Really, again? Is it because rust gui programmers are former servo
engineers?

```
Running the examples

Just do yarn and then yarn start (it will take a while to compile the dependencies, please be patient)
```

Ughhhhhhhhhh  
That's a no from me, dawg.

Yeah, even hello world is yarn. Ok. I hope this becomes a good web browser, but
I don't want to develop for fucking web again.

## druid

The one everyone knows about.

Oh no, I see gtk in deps.

FUCK

So do I install gtk for this, and also go back to dioxus? I'm willing to do it
for druid as people've been saying it's alright, but fuck dioxus with it's web,
honestly.

Ok, it's note the whole of gtk
```
The following 5 NEW packages are going to be installed:
  gdk-pixbuf-devel libjpeg62 libjpeg62-devel libtiff-devel typelib-1_0-GdkPixdata-2_0

5 new packages to install.
Overall download size: 495,8 KiB. Already cached: 0 B. After the operation, additional 1,5 MiB will be used.

The following 6 NEW packages are going to be installed:
  fribidi-devel graphite2-devel harfbuzz-devel libdatrie-devel libthai-devel pango-devel

2 packages to upgrade, 6 new.
Overall download size: 898,3 KiB. Already cached: 0 B. After the operation, additional 3,7 MiB will be used.
```

Oh no, it is all of gtk, as I now see while looking how to get atk. Shit.
Opensuse doesn't pack atk, so I'm boned. Why the hell does gtk require it
/in rust/ anyway?

Kind of weird. Opensuse has a gnome variant as well, with the same repos, but no atk-devel.

## egui
Immediate mode, skip.

## fltk

I can build the c++ library from source, or download a tarball. Well, I have
the compiler, why the hell not build it.

X11 and opengl, lol. What is this, 2018?

The author also provides all linux libraries, even a nixos shell, impure
though. Nice.

fltk-sys compilation is silent. What I like about haskell build systems is that
compilation of libraries is shown as well.

Still pretty fast:
    Finished dev [unoptimized + debuginfo] target(s) in 1m 29s

Ah fuck:
```
= note: /usr/lib64/gcc/x86_64-suse-linux/12/../../../../x86_64-suse-linux/bin/ld: cannot find -lpango-1.0: No such file or directory
        /usr/lib64/gcc/x86_64-suse-linux/12/../../../../x86_64-suse-linux/bin/ld: cannot find -lpangoxft-1.0: No such file or directory
        /usr/lib64/gcc/x86_64-suse-linux/12/../../../../x86_64-suse-linux/bin/ld: cannot find -lpangocairo-1.0: No such file or directory
        collect2: error: ld returned 1 exit status

error: could not compile `fltk-example` due to previous error
```
Because I removed pango after druid. Ok, this one is mentioned in the repo, let's bring it.

```
The following 6 NEW packages are going to be installed:
  fribidi-devel graphite2-devel harfbuzz-devel libdatrie-devel libthai-devel pango-devel

6 new packages to install.
Overall download size: 554,8 KiB. Already cached: 0 B. After the operation, additional 3,7 MiB will be used.
```

And, success!

Alright, I'm trying an example with buttons, and state management sucks. Also
it's ugly as death. Should explore how to make my own widgets.

Setting colors to buttons doesn't work. Ok, in some other frameworks I would
have to make buttons from scratch, so I'm glad I have buttons at least. Now to
check if I can make them from scratch also.

Here's a problem of several parts.
1. set_label takes a reference to a string
2. Callbacks are moved into set_callback and borrow checker thinks they outlive the current function
3. Ergo you need to keep state in `Rc<RefCell<State>>`, and render it to some string stored in `Rc<RefCell<String>>`
4. You can't obtain a reference to a value in RefCell

So at this point I have read only the first example and the api docs. Let's read the "tutorial" some more.

There's an example with message passing. It works and it's neat how typesafe it
is. Buuut it's absolutely not composable. A widget cannot have its internal
state, because all events are processed in one place.

Actually that's not true. If I'm the one to create the widget, I can put its state inside. Ok.

There are layouting types, but using them requires implementing all of
WidgetExt for your widget, which is 96 methods. There is widget_extends macro
which implements some methods, but not as part of the trait. Fuuck, no
layouting it is then.  
Well I mean, I could implement my own macro, it's a thing you do once.  
Wait, I can't at all. Because it's an unsafe trait with potential future
breakage.

Ok, here's another problem: handle and redraw can't depend on data in self. So
you can't for example make a button that looks different after each press.
Well, you can't make it from raw boxes and stuff, you can still bullshit your
way with changing the default button backgrounds. Or something.

Oh no, actually you can! You just need to put your widget state in a separate
value, embed it into the widget, and clone that into the handlers. It needs to
be shared, so wrap everything into rc, et voila.

I had a lot of problems dynamically adding stuff to a layout. This library is
veery low level, so you need to call redraws on your own. But still, the
biggest problem is the lack of layouting for non-standard widgets. I could
write my own ones though. Ok, it seems it's an ok library, let's move to the
real thing.

I tried to implement my own column layout, and i didn't work. It has size, but
it's invisible.  
Oh wait, it did, but I fucked up! There's a weird interaction with scroll.  
And not just scroll, the positioning is very fucked. Let's go and read the docs
instead.

There is a whole book turns our. Written pretty well. The documentation for
this is pretty good honestly, it's the code quality and weird limitations for
non-default widgets that are the problem. Plus layouting being shit and no easy
way to make custom layouts. So yeah, in the book the process of creating custom
widgets is described, but not custom layouts. Here's a thought: why not do
positioning by anchoring instead for the todo example?

Layouting by anchoring works pretty well. Also you can use derived widgets as
their parent via Deref, so not all is lost!

It's hard to understand when you need to call redraw explicitly, and when
resized and other stuff will handle it for you.

I'm trying to create the todo app, and suddenly nothing my new widget doesn't
fucking draw. Even if I create it statically, it's gone!

Fuck, now it magically works. This piece of shit is very brittle.

After I managed it to show, adding new functionality to the widget is really
not bad. Adding editing capability was very easy.

Oh shit, weird effects when resizing the window. Everything suddenly squishes.
I thought it would stay the same and start scrolling instead.

And it's done. Dialogue interface is nice. Or at least it would be, if there
was an option to check if the file dialogue has completed successfuly. Maybe
that's done with the get_files, but ehhhhh I'm tired of this.

Initially I wanted the save and load buttons to be below, but that would be
suuuuuuuch a huge pain to layout, so I didn't do it. Points taken from fltk,
although I knew it yesterday already.

Overall, this is an ok library. Really good for small visualizations, but not
really good for big apps, unless you want to roll out your own layouting, which
is not as simple as I'd thought initially. But here it would compete with
immediate mode libraries, and I don't know if it would win there. In terms of
design it's ugly as 1998, and even gtk style doesn't help. Is it gtk1 or what?
One advantage is great input support, and it seems to support accessibility.

- Default widgets: enough, ugly
- Creating simple widgets: simple
- Creating compound widgets: medium, unintuitive
- Layouting: easy for built-in widgets, very hard for custom
- Custom layouts: very hard
- Reactivity: not present
- Setting up: very easy
- Documentation: good
- Tutorial: present, I didn't need it much
- Diving into source: possible and not bad
- Overall: approve, a very viable option to use now
