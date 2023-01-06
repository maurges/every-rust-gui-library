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

Time spent: an hour max.

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

A couple of days later I decided to give this a second chance and try it on my
mac. You know, since linux versions are hard to make for people regularly,
let's give them some slack.  
The first thing is that I forgot how to build this thing, and had to look it up
again. Minus point.  
The second thing is that it doesn't even run:

    morj@mac:~/Projects/every-rust-gui-library/azul-example> ./azul_run.sh cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/azul-example`
    azul.App.run(x11)

What the hell is this? The mac support is mentioned on the website, and people
would expect it to just work.

But to cut the author some slack again, mac is even worse than linux, because
to do anything you have to buy a special expensive laptop, and because
everything breaks between versions so easily. If I didn't have my work macbook,
I would be hating on mac as much as on windows.

## cacao
Macos. I do have a mac, but nah. And it doesn't align with my future goals.

## conrod-core
Unmaintained. Also it's immediate mode.

The last release wasn't that long ago, and the author has said that they still
use it for their projects. Still, they say that egui is better in every way
except default style, so why not believe them and just use egui in the first
place? I mean, not a lot of people would consider using conrod at this point.
Doubly so for me, since immediate mode libraries are not what I want.

## core-foundation
Macos again. And the previous one is supposed to work on GNUStep, what's that?

Ho-ho, gnustep is a reimplementation of nextstep environment for other OS-s. It
uses objective C though and I don't want to set all of this bullshit up right
now, but it might just be an option maybe if I'm bothered.

## CXX-Qt
Time spent: a couple of hours.

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

Oh yiss, qml is nice. It's so easy to create a simple interface with, with
states, reactivity, layouts. All the basic controls are there, and while they
look weird, they are not that ugly, just very basic.  
Now that layout is done, let's do this with a proper model.

Oh crap, there is no AbstractItemModel. Well, I have a shitty idea to bypass
it. I can keep the model in qml with ListModel, and on button press send it to
rust via a series of scalar sends.

And now I'm suddenly hit with a cryptic error. I think it's caused by me using
QUrl without the necessary qualifications of a nuclear physicist. Argh, typical
c++ bindings.

Oh right, it's because rust-analyzer fucked up my file. This is inconvenient,
so I can't use rust-analyzer anymore?

Oh, a problem with pins. Projecting to a mutable pin consumes the whole self.
So you can't modify multiple fields in one method. This is stupid.  
I think this can be bypassed by storing all data inside a separate struct, and
keeping it as a separate field. For now I'm bypassing it in a different way.

In the end I had to bypass it with the separate state, yes.

Alright, the example is working and it wasn't that painful. Then again, I am
pretty familiar with qml and what to expect. Now the biggest problem with this
library is the lack of AbstractItemModel. The biggest upside with this library
is that it Just Works. Setting up was very easy, and because it builds a rust
library and links it all with c++ toolchain, I can apply qt's c++ guides to
build it to other platforms.

    576M    build/

Ahh what? Didn't expect it to be that big. Still better than azul's hello world
though

- Default widgets: enough, alright-ish looking
- Creating simple widgets: simple
- Creating compound widgets: simple
- Layouting: simple
- Custom layouts: never tried actually, the default ones are great
- Reactivity: simple
- Setting up: easy
- Documentation: great for qt, ok for library
- Tutorial: present, assumes familiarity with qt
- Diving into source: didn't have to
- Overall: approve. You'll need some creativity to bypass the limitations

Now I try to run it on mac again, and I can't build it with nix, because
corrosion doesn't work with it. I could explore the options for offline
corrosion, but nah. I also tried building it with nix-shell, but this thing
requires wrapQtAppsHook, so nada. Minus points for ease of setting up.

## Dioxus
Time spent: 3 evenings (so far)

Apparently uses tauri, which is just a webview. Ah fuck. Well still, let's see.

НУ ЧТО НАРОД ПОГНАЛИ НАХУЙ, ЁбАНЫЙ В РОТ

This is why people complain about rust builds maybe, crates.io refresh takes so
fucking long. And why does it need to refresh, azul was 10 minutes ago.  
Oh god, my meme scrolling are done and it's still resolving deltas.  
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

Alright, round two after dealing with qt. I'm trying this on mac, the hello
world is very easy to set up.

> Coming from React, the Scope object might be confusing
Ha-ha, react. Who do you think I am.

God damn it, rust-analyzer doesn't work through macros. I guess every library
that relies on them gets minus points for that. Buut, with qml I also don't get
rust-analyzer, because it's not even rust, so.

Hah. I tried an example from the book, and it didn't run. Then I noticed that
the book page has a "run" button near all the examples. So I press it, and it
doesn't run as well.

You can splice data into strings. How safe is that? Can you XSS the desktop
apps as well now? Let's see.  
Not terrible, the tag control characters are escaped.

Well, the tutorial is outdated. I can't find the standalone render function
anywhere in the docs.

You can embed NodeList inside rsx, convenient. I guess that's how you make
widgets. Now I wonder if you can do it dynamically.

You can't clone nodelist. This is stupid.

You can use a lot of regular rust in rsx. Space separates node expansions, so I
wonder how bad they parse this thing.

What I really don't like about CSS is that it has zero discoverability. How do
I know which styles I can add to a button? How do I make text field take all
the available space? I just need to google it or ask Dima.

The parser only recognizes elements starting from upcase letter. This is
counterintuitive, since elements are regular rust functions, so you need to
disable wrong casing warning.

Some of the default elements have the attribute types confused. My own element
has a bool prop, and I pass a bool, but when "input" has a bool prop, I need to
pass a string.

What's the difference between hook and state? I see that hooks don't tell the
component that it needs to redraw, so why would you use hooks at all?

> A single component will be called multiple times
Sounds like immediate-mode with fewer updates.

Ah ok. It seems that hooks can be easily updated, but state needs a ceremony to
update, and updating state will redraw.

The tutorial is out of date again on `use_state`. It doesn't return a tuple
anymore.

Why is it called hook? Makes no sense.

"Rules of hooks". Well ok, qml state management also has its rules, but this
sounds silly, like go's rules of channels.

use_state is a kind of a hook, not like they are unrelated concepts. Alright. I
feel like learning quantum physics, while working with fltk felt like doing
fucking gui programming. The two are not that different, in both cases you have
to bend backwards to persist state, and you can forget to redraw stuff.

There's one way to pass a component to a child: create a new type for the
child. So this doesn't work if you have dynamic children. And I immediately
thought of a way to make it work: provide both a vector and an index.

> calling "consume" state can be a rather expensive operation to perform during
> each render
Ok, then what do I do?

It recommends use_read for lifting state to pure components. This doesn't exist
anymore.
> This is the beauty of dioxus

So I'm mostly done with the tutorial, the remaining chapters seem irrelevant.
Well, it seems I do need to use those workarounds I thought of before. I got to
say, this seems somewhere in the middle between qml and fltk: not very
declarative and reactive and automatic, but not absolutely stateful. For
example, there are no simple views into model, you need to explicitly pack your
model into hooks and view into them. But at least there are hooks. Although I
wonder if it's even better than just passing Rc around and calling needs_update
by hand: the turorial lists so many ways to make hooks slow.

Let's try to do it with hooks/models at first.

Why do I need to provide context in the initialization of the hook? Can't it be
a method that is used standalone?  
Also with how often this library changes, makes it very easy to believe that
something is an oversight or bad design.

Whoa, this is even worse than qt-cxx's macros, because now I can't use
autocompletion inside them. This sucks so much.

A million krona question: how do you access an element's internal data? I can't
name an element at all. Fuuuuck. I see a couple of ways: on content change I
update some state, or write some javascript.

Hah, did you think I would write javascript? The approach with content change
is not that bad code-wise. It just doesn't work: I call needs_update, and the
view doesn't get updated. What the fltking fuck is this?

While washing the dishes I had a thought. The only way to add dynamicity to
your app is state, and working with state is a pain. Furthermore, once a
component is created you can't access it, get its internal state, so you have
to pass Rc-s everywhere - dependency injection of sorts? This means that your
component can't react to each other, and this means that you can't layout your
components in relation to other components. Not just layout! In qml it's
trivial for one component to have the same color as another, the same content,
and they will be updated automatically. Here, you need to pass it through
hooks. And then you get a problem with components not fucking redrawing.

Well, I tried several options and it still doesn't redraw. Let's call Arkadii
and see his opinion.

Great news: in react there's same еботня with taking changes from input fields
into your own state. This is the problem with modern development, we have to
think up roundabout ways to extract data that the computer already knows, that
your system already knows, but which is hidden from you by your framework.
Well, that's what someone like JB would say, but really this pattern is
everywhere, you can say this is the essence of math.

So, the shitty thing here was passing state completely into child components,
instead of projection. I did this because I want the state to be mutable, and I
can't pass mutable projections. Or can I? The first approach is to wrap every
member in rc-refcell and pass those as prop, and mutate them. Since I only need
to update the component itself and not outer data, this works.  
The second approach is like with input fields: add a callback to child widget,
and set my own state from the callback. This seems to be more general, but a
lot worse in terms of performance, since I need to update every child it seems.  
Also both those approaches don't solve the redraw problem, it seems. If they
do, it's a very brittle problem that you could break by accident.

It seems I can't add event handlers to my own widgets. This is because I need
to pass it in props, and Props need to be PartialEq, and EventHandler is not
Eq. I also can't pass just functions, because the macro automatically converts
them into EventHandler-s.

"When working with large sets of inputs, you might be quickly tired of creating
use_state for each value" they say, and then provide no solution. Lol.

I reread this while fixing merge conflicts, and here are some things I now
believe different:
- Type confusion is not just in properties, it's also in events.
- Both here and in fltk I pass state with rc-refcell, and call redraws by hand.
  Where are the upsides of using this?

Alright, I rewrote to rc-s. It didn't help, obvously. Then I added another call
to needs_update in the same place I tried before, and now it fucking works. Ok.

Top kek: in checkbox update event the data is not bool, but string.

Alright, and now a million dollar question: how do I create a file saving
dialog?

I think a sizeable number of rust people have a conception of a prelude module
very different from haskell guys. In haskell it's a convenient reexport module,
but if you want you can import everything in a more structured way. In dioxus
it's a module where you get stuff from, period.

Ughhh, I can't create a dialog? And no transitive dependency allows me to? Well
that's stupid. I wonder if there's a standalone rust library for native file
dialogs which one would use, but I don't want to look for one now.

Once again I'm reminded that people shitting on haskell because of immutability
are retards. It's the exact same in rust! It's because structs with data and
structs with behaviour are different!

Ok, added saves and loads. I'm tired of this so I'm not exploring how to do
native dialogues, just use a builtin location. It was pretty easy to do, but I
ran into the "problem" from the paragraph above. More like stupidity that this
library forces me to do it. Also I forgot to add "redraw" and it didn't redraw,
ugh. And it seems that, again, consecutive presses of "load" don't redraw it,
even though the state is changed, and the button is pressed. So this is very
unreliable.

I was told that in react instead of onchange callbacks for input, you can use
use_ref and ref for the field. There is some use_ref function, but no ref
field.

Conditional rendering is a nice concept. Assembling your tree from some input
is very intuitive.

What's not intuitive is that mapping to html is weird, so I can't use MDN.
What's worse is that it's not in the docs, it seems rsx macro does some magic.
I can't set input's value to text, it expects "Arguments", but arguments are
not documented. And input itself is not documented either! After a bit I
remembered that I could try string interpolation instead of setting to a
string, and it worked. This is a stupid limitation.

Would be nice to have autocloning of rcs into closures in rust, but I already
complain about autoderefs being too magical, so I don't know. Do I want
c++-style very explicit lambdas?

Ohhhh fuck, hooks are stupid. That's why I had rc-refcell, I forgot. So a hook
returns you just a mutable reference to your data. You can't use that reference
in more than one callback, obviously. So you're boned.

Lol, I can see update artifacts after pressing the done button, what the fuck.
So much for web being fast.

You can use state instead of hooks and now you don't need rcs, they are hidden
there for you. Damn, so now at every junction you need to make a choice.

So, at this point I have reached (almost) feature-parity with other
implementations, but I didn't achieve layout parity. I reaaaaally don't want to
bother with CSS right now. Instead I'm going to do a second round, where I make
every implementation look the same.

So, funny story. Today I asked my web friends about the use of hooks and state
to get text field value, and there was an argument between them about good
practices, what you should do and what you shouldn't. I take away from that two
things: noone in web knows what to do, and they don't fucking care about
performance. Keep in mind my sample size is only slightly better than one since
I know them well.  
Also they were talking about react refs, which would be a neat solution and
which this library doesn't have.

- Default widgets: enough, alright looking, everyone is used to them
- Creating simple widgets: medium
- Creating compound widgets: simple to medium, problems with props
- Layouting: easy until it's very hard
- Custom layouts: just kill yourself
- Reactivity: medium
- Setting up: very easy except on linux
- Documentation: medium to bad
- Tutorial: present, outdated, very necessary
- Diving into source: didn't try
- Overall: only if you like web

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
Time spent: 3-4 evenings? I forgot to time.

The one everyone knows about.

Oh no, I see gtk in deps.

FUCK

So do I install gtk for this, and also go back to dioxus? I'm willing to do it
for druid as people've been saying it's alright, but fuck dioxus with it's web,
honestly.

Ok, it's not the whole of gtk it seems
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

Kind of weird. Opensuse has a gnome variant as well, with the same repos, but
no atk-devel.

Round 2, trying it on mac. The example from readme doesn't build, fucking
classic.

Well, it really requires gtk on linux, no way around it. They are working on
pure wayland and x11 backends to replace it, but not currently available. Sad.
I hope by the time the examples in readme are not outdated, it will be ready.

WHAT the FUCK, 0.7.0 was released in january 2010? So it's not the examples
that are outdated, it's my library copy!  
So let's try from HEAD. I try, and I get "generic associated types are
unstable". But wait, weren't they stabilized recently? The hell is going on?  
Ah ok, I'm using outdated rustc turns out. Without the one feature I was very
waiting for.

Lol it's a mac app, closing the window doesn't kill the program. Lol,
inconvenient.

Widget creation looks like fltk. I wonder who came first.

Alright, this looks very foreign to me, so time to read the tutorial.

> This book is written against the latest code on git, so some examples may not
> work with 0.7.0.
Ha, ok. I gave too little credit to the guys, but that's because I remember the
author saying that druid is starting to focus more on experiments and eschews
stability. So I thought this is it, no onboarding for the new versions.

Flex row tries to center horizontally, but flex columnt aligns to the top. Mmmm
ok.

Align doesn't do anything. Dafuck.

Lens chaining is verbose, but that's what you get for not having custom
operators. Though I wonder if + could be reused here.

Oh shit, the online docs are also for 0.7. I forgot, time to use a local copy.

Passing env by hand, because you also don't have monad readers in rust `>_<`

> localization is currently half-baked
I wonder how does localization work in qt.

Well, this was a short tutorial with only the very very basicx. I guess now I
need to experiment and use my own judgement? Awful, no wonder programmers burn
out so quickly.

What I gather from the tutorial is that ALL state is top-down, from parent to
child. Only parent keeps the state, the children just get a view into it. It
seems they can modify the view, and they declare what is the type of state they
themselves require. My questions now before I learn more about it: how painful
is it to create all the state at the top? Can you serialize and deserialize
state? Can the widget modify its projection? Can two widgets have the same view
into state?  
Already it seems you can't serialize in the general case, because arc can be
state and it can contain anything.

From the hello example, the textbox doesn't modify the state explicitly, but it
gets the projection into it as a model, a-la qml. This is pretty neat. Now,
another question: can I have a textbox point not into state? Into void maybe?
Can a lens modify the content meaningfully?

Two textboxes can point into the same state projection. This is neat.

must_fill_main_axis behaves vety weird. I don't understad the layouting.  
I want to debug it but I can't find something like a qt rectangle.

I have no fucking idea how flex layouting works, specifically spacers. How do I
make a widget occupy all available vertical space? How can I add maximum
spacing inside the parent?  
I have a feeling that here, in qml terms, widgets only have implicit sizes, and
you can't set size explicitly at all. Wait, that's wrong, there is SizedBox.
But it's inconvenient to use. And you can't size a widget in terms of another
widget, because that's state and state is hard to manage.

Ok, this layouting is getting me nowhere. Let's do the most simple way and be
done with it. So far I am very pleased with performance, find some core ideas
interesting, but the whole execution could be better, or at least needs a
better tutorial to flexing.

Wow, I don't know how centering works as well. This shit is too weird.

Playing with state: state type is constrained in the widget, so you can't fuck
this up in runtime. Good, I didn't give enough credit to the autors. Now how do
I project state for my custom widgets, and have it be generic?

Fuck, I was trying to make Vec work, but turns out I need im::Vector.

If you go through the examples, there is a specific widget for creating a list
of items, a-la Repeater. Pretty convenient.

Wow, fuck me, when creating a button with a callback I got a huuuuge lifetime
error. Because rust type inference is weird and works differently depending on
if you create a local, or just make an expression. This sucks.

Creating a label with dynamic data sucks just as much, but now even without a
local. Fuck, this would be so much better if it were garbage-collected. If it
were haskell. Maybe one day I could port this approach to haskell, with native
lenses, with native immutability, with easier callbacks.

Now here's a hard part: separate widget state into internal and external.  
It wasn't so hard.

So you have widget sum with Either, and you have widget catamorphism with List.
One could create widgets for anything similar using the default interface,
which is nice.

I really like how if it compiles, it works. Best part of rust. The worst part
of rust is, of course, that sometimes it's hard to fucking make it compile,
like here when lifetime inferrence goes to shit. It goes to shit in presence of
Into traits, and thankfully this library provides a way to bypass those traits,
like dynamic(), from_label() and such.

Fucking lol, every app in my mac freezes when it tries to open a save dialogue.
I thought it was just buggy firefox, but no.

Saving and loading is weird. Is it possible for a subcomponent to open a save
dialogue? Yes. Is it possible for the subcomponent to then act on the result of
the dialogue? Only if the app allows it. So you need to set up a message
passing system for this to work. I like the qml approach more, even though it
also becomes a non-locality spaghetti.

And with this, druid is done. I could have styled it, but oh my god flexes are
weird. The default widgets look neat, modern you might say. Creating new
widgets is pretty easy. But you can't style the existing controls, so that's a
minor downside. Although so far only qml and css-based things allow for
styling, and in very different ways, and for qml it's kind of .. strange? that
they do. It's nice that it's there, but why did they do it if it still requires
creating new widgets.

So, I really liked the approach with state and lenses. It was very intuitive
and easy to combine, but kind of boilerplatey, since you need to declare your
state separately. Would be nice to have the state hidden with async maybe? I
don't know, how would lenses even work then.

I couldn't grok the flexes approach, but on the other hand I specifically
avoided the examples as to not spoil myself on the todo list solution present
there. When I did dive into the examples, they were very easy to understand and
I immediately grasped the mechanics. Those were lenses, state, and meta-widgets
like List and Either.

Alright, alright, let's try it without gtk on linux now, does the alpha work?
I'm not using any advanced features after all.

```
The following NEW package is going to be installed:
  libxkbcommon-devel
  libxkbcommon-x11-devel
```

Oh, zbus. Isn't that the incredibly bloated one?

Alright, it builds and works. But the input field behaviour is very wrong,
whereas on mac it was only slightly wrong. Scheisse.

```
morj@blackflame:~/projects/gui/druid-exampe> du -h target/debug/druid-exampe
186M    target/debug/druid-exampe
```
Shieeeeeeeet, rust

- Default widgets: enough, good looking, weird text behavior
- Creating simple widgets: medium, long
- Creating compound widgets: simple
- Layouting: basic rows and cols are trivial, but I didn't understand the complicated stuff at all
- Custom layouts: medium, very long
- Reactivity: simpler than qml
- Setting up: very easy
- Documentation: medium with good examples, except flexes
- Tutorial: present, outdated, very necessary
- Diving into source: didn't need to
- Overall: approve, even though flexes are hard

## egui
Time spent: entire Colors II by BTBAM

Immediate mode, skip.

Alright, right now I'm procrastinating before going to qt again with
qmetaobject. I don't know, am I afraid to be disappointed that the state hasn't
moved far, or that I can't even build it like orbtk before it? Anyway, let's
look into egui, why not. Immediate mode is also important sometimes, and
sometimes it's just a better way to do things, like how most of non-web guis
I've written professionally were imgui in c++. So, let's look at the most
popular rust gui library I guess.

Uh-huh, the author says the idea was to have minimal dependencies, but it's 180
now, more than orbtk. To be fair, that's not egui per se, that's eframe, but
still, come on.

This is pretty neat. The default style is too small.

Alright, forget what I said about dependencies: ron, serde and rfd pull in
another 280. What the fuck, rust, what is this js shit.

What's the deal with gui libraries and adding a text field to a check box? It's
a check /box/, not a check /form/.

Hah, shit, buffers need to be persistent between frames, otherwise nothing gets
written. That makes sense, and it makes an inconvenience.

Well, this indeed was fast and pleasant. Imguis are nice. Yeah, the widgets
aren't great and I don't want to get into sizing, but for basic stuff this is a
really great library. Hearty approve.

    du -h -d1 target/
    2,3G    target/debug

Fucking hell, rust, what the shit is this?

- Default widgets: enough, good but not great style
- Creating simple widgets: dunno, but should be easy
- Creating compound widgets: simple until you want customization, then medium
- Layouting: well it's imgui. There are rows and columns and grids.
- Custom layouts: dunno, but it's imgui so probably easy
- Reactivity: immediate
- Setting up: trivial
- Documentation: no tutorial, so you need to be acquaintanced with some of the concepts. Apidocs are good.
- Tutorial: none
- Diving into source: didn't need
- Overall: great stuff, recommend

## fltk
Time spent: a full saturday and half of sunday.

I can build the c++ library from source, or download a tarball. Well, I have
the compiler, why the hell not build it.

X11 and opengl, lol. What is this, 2018?

The author also provides all linux libraries this depends on in a list, and
also a nixos shell, impure though. Nice.

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
2. Callbacks are moved into set_callback and borrow checker thinks they outlive
   the current function
3. Ergo you need to keep state in `Rc<RefCell<State>>`, and render it to some
   string stored in `Rc<RefCell<String>>`
4. You can't obtain a reference to a value in RefCell

So at this point I have read only the first example and the api docs. Let's
read the "tutorial" some more.

There's an example with message passing. It works and it's neat how typesafe it
is. Buuut it's absolutely not composable. A widget cannot have its internal
state, because all events are processed in one place.

Actually that's not true. If I'm the one to create the widget, I can put its
state inside. Ok.

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

I tried to implement my own column layout, and it didn't work. It has size, but
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

## flutter_rust_bridge

Do I want to do this? How many people would have a flutter toolchain ready, how
many know how to do it? Let's look at the tutorial and examples and see how
much dart code there is.

    This library is nothing but a code generator that helps your Flutter/Dart
    functions call Rust functions

So it's not a graphics framework, flutter is a graphics framework, and this
only allows you to call rust functions from it. Not sure this is what I want.  
On the other hand, with cxx-qt it was the exact same: you describe your
graphics in another language, and you call rust functions from it. I guess the
only difference is that I have qt toolchain already, and that I knew it would
take me an hour max. Also I never said I wasn't a hypocrite.  
But I remember being very interested in flutter myself, kind of liking its
approach when it came out. It's only recently that people discovered that it's
very easy to make slow apps with it.  
Maybe I'll go back to it later, when/if I'm more interested to try flutter
again.

TODO

## fui
Time spent: A full evening

A library by a single person. 40 stars on github. Last commit in august.
Alrighty, let's see. It's not like you can't /finish/ making a graphics
library, especiallly if all system stuff is done by libs you depend on.

0 issues, 0 pull requests `>_>`

```
error: failed to run custom build command for `fui_system v0.11.0`
--- stderr
thread 'main' panicked at 'failed to execute 'qmake' process: Os { code: 2, kind: NotFound, message: "No such file or directory" }', /home/morj/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/fui_system-0.11.0/build.rs:47:10
```

Hah, shit. So it's qt, which is cool, but opensuse uses qmake-qt5 instead of
just qmake. Who the hell uses qmake anymore anyway. Yes, I agree it's 1000
times better than cmake, but still.

HOLY SHIT IT'S SO UGLY, what the hell

Well, the api docs don't exist, and there's no tutorial. Alrighty.

Let's not forget, aside from the qmake thing, building it was very easy. But
that also means that it has to be compared to qt adjacent libraries, where I'm
not sure it would fare well.

```
thread 'main' panicked at '`spawn_local` called from outside of a `task::LocalSet`', /home/morj/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/fui_app-0.12.0/src/application.rs:105:35
```

So you need to start the gui from LocalSet. Alrighty. Why wouldn't the lib
itself handle it for you then.

Ughhh, how do you set size of a text field?

Judging by the docs, this is inspired by qml. I deduce that from attached
properties. The approach that the "declarative" syntax is just sugar for
builder is neat I think. Duid would look better with the same sugar.

What happens if you use local as a Property? I tried it now and it doesn't
break it seems.  
Well first of all, I won't be able to update it from callbacks, as they require
an rc-refcell.

Let's just play by the rules and finish this fast. I don't like this library,
but it seems simple to use.

Oh fuck, can I have internal state? Is it possible? Creating custom complex
widgets is not really explained anywhere. Alright, there is one example. You
need a sub-viewmodel. Ok, let's try.

Looking at other examples, the ui macro seems similar to that of dioxus: same
nesting, same magic places where you can write rust for real.

I'm thinking of abandoning this library. Wading through weird compilation errors
of incompatible Into bounds is so tiring when the examples are minimal and the
apidocs are without comments. This project was to show that rust has nice gui
libraries, and this one clearly isn't.

## Gladis
This seems to be part of gtk-rs now, so it's coming right up!

## GTK

Next in line, but there are problems installing it so I'm postponing.

TODO

## iced
Time spent: a couple of hours

I've never used elm before, or have tried the elm architecture. The example in
the readme is very simple to understand. Now, is there a tutorial? Do I need
one?

    Finished dev [unoptimized + debuginfo] target(s) in 7.58s

Fucking how long? What the shit did they do?

Ha-ha, todo example is there. I'm not going to look there of course, even after
I'm done, as I did with the others.

I read somewhere once that in elm it's very hard to do encapsulation. Here I'm
given a `Component` typeclass, which allows me just that. But for some reason
it's in a different crate.

So component needs to be able to produce message of any type. That means parent
needs to tell component how to map component messages to its own. I wonder why
not the alternative: to map those messages in parent itself. Ah ok, I thought
of a reason: if your component doesn't produce any messages at all, the current
option is easier to do. How often is a component so isolated?

Playing with controls tour, they look very like qml controls, but less ugly.
Accent color looks good. Being positioned in the middle of the window looks
good. Shouldn't be hard to make flexboxes like this in qml.

No animation on toggler! How do I do animations at all here? I haven't had that
question with any of the frameworks, but it's kind of very important.
Animations in qml are piss-easy for example.

Oh-ow, image doesn't load. Shiet.

No kinetic scroll in default scroll widget. At least it scrolls naturally.
Scroll remembers position when you go between pages.

Overall, the control quality seems good. Alright, let's try it.

Fuuuuuuck the build times are bad. This really grinds my gears.

I don't understand flexboxes again. I have copy-pasted the hello world example,
and it looks a lot worse now: wrong padding, wrong alignment.

Well, this elm architecture thing is pretty nice to work with! Creating a
simple widget is very simple. Layouting here is as hard as in druid in seems, I
just don't understand flexboxes. It doesn' seem like there is a lot to
understand there, but this little escapes me. And like, if I could ask the
right questions, finding the answers seems pretty easy, but I don't even know
what to ask. Something about paddings and alignments is not as I expect it to
be, and I haven't formulated what I expect and what is wrong.

The fuckery with renderer is weird. It's good that I can ignore it, but I would
like a tutorial.

    Finished dev [unoptimized + debuginfo] target(s) in 8.68s

Wow, it's getting even worse.

Alright, how do I do file dialogs? Ok, there aren't any. Shiet.  
Ok, people discuss using [rfd] for that, which is toolkit-agnostic. That means
I can add to dioxus too.

Why does all of rust love the fucking gtk so much? Why does rfd need whole gtk
for it?

Well, and it's done. This was extremely nice, if only every library was as
accessible and intuitive as this one. Actually no, there are only so many
"intuitive" to me approaches, and seeing something new is also good.  
This library is better fltk in a sense. You just write out your widget, handle
messages, and it works. And unlike fltk, you don't need to care about redraws,
that's done for you. But like fltk, something can very easily go wrong. It's a
miracle nothing did here: I expected data to not update.

Oh shit, and it doesn't update! Because I clone! Fuck!

Well, emitting a message from an encapsulated widget in a row of such is a
problem. Again I'm making shit up with setting explicit index.

Ok, it works and wasn't even that painful. The better approach here would be to
embed the state struct completely into widget, and maybe even use the special
state variable in the methods. Still, it wasn't great, especially seeing how
now I've forgotten to update the bool. I don't know, I did expect everyone but
qt to be bad at it, but druid was very good here. My opinion of iced is a bit
lower now, but I still think it's really solid and can recommend it, and
probably will use it again.

- Default widgets: enough, stylish
- Creating simple widgets: hard
- Creating compound widgets: medium
- Layouting: I don't understand flex
- Custom layouts: no idea TODO
- Reactivity: simple-medium, problems with subwidgets
- Setting up: very easy
- Documentation: good
- Tutorial: no, but wasn't needed
- Diving into source: didn't need to
- Overall: approve, very easy to start using right now

## imgui
Time spent: like 30 minutes

Immediate mode, so no. Imgui for C++ was nice, so try using this and see how it
goes.

Well, why not try this one too then. The egui author was saying as if this one
is as good, though they did this faint praise thing when you don't really love
it or something.

Why the fuck is init from example so long? Do I need all of this?  
I see, so this library is lacking an endorsed eframe thing. There are a couple
of options. The readme mentions imgui-winit-support, which as docs.rs reports
has some problems, lol.  
The main library mentions that you need to pick and choose your options, but
the pick is like 4 crates long. Fuck that. So let's just copy the support code
from example.

Wow, it also doesn't include a default font it seems. This is very bare-bones,
or should I say, embedding-oriented. Also brings me flashbacks of loading
system fonts at tempo and getting very weird UB in c++.

    thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /home/morj/.local/share/cargo/registry/src/github.com-1ecc6299db9ec823/imgui-0.9.0/src/fonts/atlas.rs:73:55

Fucking, fuck. This might be because I removed all fonts, so fiiiiine, let's
add something.  
Yep, it was fonts. Now japanese doesn't render, but that's ok.

Wow, the system integration is thin. It renders a window inside a system
window, the resizing and position is not synced. I don't remember having this
problem in c++.

I forgot how weird this is. To arrange widgets horizontally, you need to call
ui.same_line between them. This is stupid.

The positioning of widgets is controlled via cursor, which is more well defined
than egui, where I don't know how it works. The bad thing is that it's in terms
of window not parent, so you can't really control it. Althoughh, you can query
the current pos and layout from there. In any case, this is a pain and you
don't want to do it.

    imgui-example: ./third-party/imgui-master/imgui/imgui.cpp:7793: bool ImGui::ItemAdd(const ImRect&, ImGuiID, const ImRect*, ImGuiItemFlags): Assertion `id != window->ID && "Cannot have an empty ID at the root of a window. If you need an empty label, use ## and read the FAQ about how the ID Stack works!"' failed.

Fucking hell. Where did I do that?  
Huh, that was checkbox. So I can't have checkbox without text? Are you out of
your fucking mind? What is this with gui libraries and checkboxes?

Ughhhh, and the text input also has a label. What the shit is this?

Fucking lol, and now it aborts when I left click on the text input. Oh god, I
wonder if it's all rust fault or the original was like that.

Ok, the abort was because the checkbox and the input had the same label. So
what, if I have a lot of widgets, I need to create a different label for each?

Yep, I do. Alright, this shit is broken, I can't continue.

So from the api already egui was a lot better, but with the bugs and
null-terminated strings and weirdness, you just don't want to use this.

## iui

Abandoned, based on abandoned C library, which was in mid-alpha, as author says.

## KAS
Time spent: a couple of hours (so far) + a day + half a day

When I first read about a "toolkit abstraction layer", I thought this wouldn't
work. But the author pivoted and now it's just a toolkit itself without native
dependencies. From the readme, I sympathise a lot with the author's approach.

Whoa, whoa, macros for widgets? A bad sign, so far this approach has been shit.

    Finished dev [unoptimized + debuginfo] target(s) in 11.73s

Holy shit, this is even worse than iced.

The composition is weird. Why in the counter example the buttons are in a
different role than a label? What if I want to change the button text?  
Alright, that one was simple. The ones in layout declaration are anonymous, but
you can create named widgets as members.

Huh, fun. I tried dynamically changing the button text, and the sizing breaks.
How do you size elements anyway? The layouting is so weird.

Actually, the layouting is very easy. It's just qml's rows and columns (and
grids) with align() added. Not GridLayout though! Still the question stands:
how do I size the elements? Can I do it dynamically?

What happens to messages that are never handled? Do they pile, or are they
discarded? What if my message handling reads different messages on different
calls? It does so right now too.

Unlike Qt, there is a distinction between view and a widget. A widget is a view
with a model attached? But this is in stdlib only, the widget I myself wrote
switches from view to widget depending on constructor. Hmm. Also good news is
that sharing state is extremely easy, as long as your widgets support that. So,
good to know: write your widgets to use SharedRc and SingleView, or even
ListView and others. Yeah, so the distinction from qt is that there there were
properties as class fields, and here properties are first-class values. Class
field properties are easy to create and so everyone just did that, and first
class properties you need to account for.

Well, the tutorial was enlightening. This is an interesting framework, similar
to qt in some ways, and with very weird layouting and sizing. I want to say
that good thing about flexes is that even when I don't know how they work, the
layout looks ok, but here it looks ugly.

Trying to center a widget in the window with margins. Add margins(large) and
get weird errors about missing fields in the code generated by macros. Ugh.

Wow, sizing is fucking hard. Managers, rules, hints. I just want my widget
centered in the window! I hope the author sees this and adds a turorial for
custom sizing.

Alllrighty, it's been more than a week, let's see that I haven't forgotten
everything I learned about kas and write the app finally.

Checkbox is not a view. Right, because it's a widget. If I look into
kas::view::driver, I can find a CheckButton there, but I need a box with no
fucking label. Ok, it will be easier to do this with messages I guess.  
Or not. Because I can't assign a message to the checkbox, only a callback. Now
how do I send a message from a callback? Or maybe I just close on the model and
update it directly.

Wow, thinking about how I should structure my data is hard. There are a lot of
options, so I'm now getting lost in choice, and what's worse in my stage, a
paranoia of how my choice will fail down the line. Let's try the most stupid
approach first I guess.

I though it would be a good idea to make my TodoItem a view into todo state,
then create a list of states and a list view with TodoItem as a delegate. But I
can't figure out how can I do state projections, and without that it's
impossible/a pain in the ass with callback. I guess I already do this as a
callback pain though.. Let's revisit this actually, my thoughts might have led
me in the wrong direction.

Wow, ListView is hard. And the tutorial was only for single data. I could dive
into examples for this I guess, because I still give credit to the author and
believe they have thought of this.

I look at the example and it's really fucking hard, a lot of code and a lot of
impls. But in terms of boilerplate it is kind of similar to qt actually.

Even with this big example, I can't see how can I push data from view into
model. With messages only? Ehh that works as well, not worse than iced for
example.

Fuck, RefCell is not SharedData. Even though in the example the data is a
newtype around refcell and implements every required member. What, do I need to
do the same?

So, do I need to write a shit ton of code for my refcell for mutable data?
Let's do it, my mood is better now.

Turns out there wasn't much to implement. Or I don't know, because now when I
try to create a list view it doesn't compile and says that I need more
instances. Stupid piece of shit with too complicated types.

Welp, turns out there were borrow errors that rustc shows too late. I can't
implement SharedCell like this. Maybe something more involved is possible, but.
At this point I'm even more tempted to fucking give up on this.

So let me write down some "closing" thoughts now before I give up on this: this
library hides stuff from you in some places, but exposes too much complexity in
others. The layouts are too simplistic for real use, but custom models are too
complicated. And the worst part is that they are not so bad that you notice it
immediately, at first you think that that could work, but then it doesn't and
there's nothing you can do. I wanted to solve those problems myself by writing
my own layouts or models, but the toolkit doesn't give me enough instruments to
do that; some parts are forbidden, and other parts are unintentionally locked.

I decided to go back and redo it without model-view. Good thing is that I don't
have to redo the TodoItem, only add a get-state method. This is a good sign.

The bad news is that it freezes on launch and can only be shut by kill -9 for
some reason. Is this like when you allocate too much in haskell?

Hmm, yeah, might have been a random allocation or recursion problem. Or just
too big datatype built. I replaced a dialog::Window with a Window instance for
my struct, and now it runs ok, but the startup is still kind of slow. I guess I
need to give points to iced and druid here that their debug builds still run
very fucking fast. Also, damn this is kind of ugly. Sizing, spacing, what the
hell.

It seems there are no platform dialogues. Alright, I already know a crate for
that!

Why does config_mgr require fucking FnMut when it's called once? This sucks,
now I need to clone shit.

Ok, this was almost straightforward, if the api for managers was more sane it
would be even better. Why the hell do I need managers anyway? Why can't I just
do stuff directly? I guess half of that is for theming, but that's stupid and
noone actually likes theming.

Now a million dollar question: how do I switch between widgets dynamically?
Honestly this is one thing those vdom-inspired libraries did right, that you
can alter the tree however you like depending on everything. This gives you
nice freedom.

Well, I can't complain much, the stack is a pretty natural way to do this. I
think something like this could be done with an enum of possible widgets, but
then I would need to implement a lot of shit from Widget class. I need a macro
to derive that for active value, would be a really good solution.

Closing closing thoughts: kind of rough. Has some nice ideas with bad
implementation. The good thing is that I think those implementation problems
can be easily fixed with some manpower. Oh wait, not all of them, I have no
idea what is to be done with layouting, it's just so basic it's impossible.
Also it's a bit disappointing how it doens't bring any new ideas to the gui
world, but that's really not a bad thing. Tried and tested solutions, all that.
Reactivity here is weird, in that it's closer to fltk, but it has all that
model-view thing. The MV is really undercooked, it's hard to do anything
complicated. Simple stuff seems ok but requires a change of thinking, adding
explicit models to your struct fields. I might have gotten it wrong, since
after all I didn't manage to get it to work, so.

- Default widgets: enough, plain to ugly-ish
- Creating simple widgets: a lot of work
- Creating compound widgets: easy
- Layouting: only basic rows and columns
- Custom layouts: impossible
- Reactivity: simple and easy to understand, but not pervasive
- Setting up: very easy
- Documentation: good
- Tutorial: seems good at first, but model-view is lacking
- Diving into source: looked for model-view, sources are nice
- Overall: from alright to too complicated

## lvgl

This is an interesting thing. I once thought that it would be interesting to
create a gui library for pine watch or something similar, and here it is
already existing. It's not rust though, it's a c library. Do I need to
pre-install it? Does it run in immediate mode?

Fucking styles, this is stupid.

So this is a display library, similar to fltk I guess. It has basic layouting
it seems. Rust docs are absent, but I also can't find a link to C docs on their
github page. Alright, I found it on their website, but there are no apidocs
there. How do those people even live. Also from the rust example it seems that
it's immediate mode? You need to handle all events yourself in the main loop.

So this is not what I'm looking for, but an interesting project. Too minimal
for my preferences, but

For the records let's say that the main reason I skipped it was because desktop
is not even a first class target. You need an emulator-like environment to run
this, or so I gathered from the docs. The docs are not ideal, and the rust docs
are just missing, so.

## Makepad

No readme on crates.io, lol. Ah, it seems it's a placeholder. Fucking
squatters.

Is makepad.dev them? There are a lot of makepads, but this one uses webgl for
the website; and the website contains rust code; and the website doesn't have
any info on what the hell is this.

Ohh ok, found their github. They have some info there, and that website was
indeed them. That's the super new rust IDE with live reaload. Cool.

> To facilitate this, the styling of Makepad Framework applications is
> described using a DSL. Code written in this DSL is tightly integrated with
> the main Rust code via the use of proc macros

From ehhh to nyehhh.

Hah, the layouting is similar to KAS: you describe the element's flow and then
you put all children inside it. Actually what I didn't say back then about KAS
is that nesting flows for one widgets is a neat idea that is really easy to
keep in your head. That's one thing they did right. Here you would need to nest
widgets like qml, but you can still only operate on flows it seems.

Main event handling loop, ok. Can this be widget-local, or is this elm
architecture again?

Explicit draw call handling? What is this? Don't you just always draw on that,
why the complication? Uh-huh, it's because draws are immediate mode they say.
Huh. But also from this example it seems that handling can be nested, which is
good.

Ughhh what the fuck, explicitly get event from the button, explicitly update
label, explicitly redraw? This is worse than fltk.

So ok, this is woefully incomplete, has a crap api, no docs, no tutorial, no
official release, doesn't support linux or windows. But the author knows all
that maybe except for the api, so let's not linger on this and move on.

## native-windows-gui

Windows, lol

## OrbTK

Only one letter until the one I care about. This one I have heard things of,
but not very excited. Buut, an unexciting library can be pretty good, like iced
was. Let's read the first line from readme.

> It is with great sadness that I announce that OrbTk is sunsetting

FUCKING LOL. The reality can be so fun sometimes.

Interesting, one developer went on to make iced, and the other two went to
slint. I like them both, so now I /am/ interested in orbtk. Enough to read the
docs at least, don't wanna program I'm a burned out sad boy (or girl).

Huh, by the time of iced the devs learned to make pretty widgets, but here they
are still ugly. Not fui ugly, but android 1.0 ugly. Maybe it will also be that
they didn't learn to design a framework until their next one as well?

Who is this book for? Why does it not assume the familiarity with rust? This is
a new. Also funny how first three whole chapters are all introduction.

Similar to KAS, you emit events somewhere, and then catch them somewhere else,
doesn't need to be the same widget it seems. Like iced, the events need to be
the same type. I didn't really like how events in KAS were all basically
untyped, same as sending labels around in ruby, although you could use it more
safely with hand-rolled enums. Ehh actually now thinking about it, it's a
complicated matter and I'm not sure myself what is better. In rust I mean, it
would be really fucking great to use open sum types (row sums actually), but.

Who enforces the separation of state handling? What's stopping me from handling
the state in the callbacks? I mean, I guess if I play by the rules it's easier,
but I prefer it when the best option is the only option.

Either this thing is too complicated, or I'm too sleepy.

Oh hey, orbtk uses ron, nice. I approve.

    could not find native static library `SDL2main`

Huh, didn't a lot of other libraries use sdl without this problem? They might
not have since sdl has an option of using a bundled library, but here they
might be using a system one. So I installed a sdl2-devel, and now I have
/usr/lib64/libSDLmain.a, but this problem still persists!  
Ok, they mention you can use a feature "bundled" for that. And it also doesn't
work. Ughh.

Well, since I don't have a mac now, this will be be a pause or maybe the end
with orbtk. I don't remember what I read in those tutorials, so I don't have
any opinion really.

## qmetaobject
Time spent: day and a half of playing around

Oh boy. This should be the good one.

It even has qwebengine, whaat. Why are all the rust people even using gtk on
linux then?

Interesting, this build uses my kde's breeze theme instead of qml default. Also
it has a weird but with button recoil, where if you press it too quickly, it
won't execute the action.

Ohh, so for setters you can use a regular rust method, nice. Overall this
provides some pretty good apis it seems. Well, it seemed to me this way long
ago, but now I need to fucking sit down and write it.

Interesting, slint was co-authored by the main author of this crate. So it's
part orbtk, part qt, and I remember seeing there was a third person. Good
creds.

Qrc. It's better than xml.. or is it. At least malformed qrc is visible at
compile time, but there's a lot of room for mistakes still. I would like
something like yesod, would be cool, but there are still problems that qml
itself is too dynamic. Maybe qt6 with compilation fixes this, need to
investigate. Also I hope slint makes it better.

So thanks to the fact that I didn't use a real model with cxx-qt, I could port
the view from there without work, this is nice.

Why not first try the approach from cxx-qt and then do a proper model-view?
Let's show that cxx-qt is really not the thing you want.

Uh-huh, cxx-qt had auto camel case, but not here. It would be better to use
snake in qml, but easier to use camel in rust, so let's go.

Alright, so there is a problem that qml can't handle rust Strings, but this
library will happily allow you to pass them back. Actually, it's weird, since
you can accept Strings and it will autoconvert, but the return strings won't.
Not big of an issue, it might be a good idea at all to just use QString
everywhere maybe.

qinvokable from cxx-qt was more convenient than double defining stuff. I could
also single-define stuff, but I don't like the style. Although..

There is a SimpleListModel, but it's not mutable from qml. Inconvenient.

So at first I was thinking: how do I even use this simple list model? But the
solution is actually the natural one: you define the qml type the regular way
and instantiate it in qml, that's it. But yes, it isn't a drop-in replacement
for what was a qml's ListModel, you can't append to it etc. Now another
question: can I access an object from both qml and rust? Actually, what am I
even thinking about? I have somthing in my head about creating an object in
rust's main and passing it to engine. There are singletons, but that ain't
really that as they are created by engine itself. What else was there in c++

Well shit, there are a lot of problems for mutable list models. The one I'm
running into right now is that SimpleListItem doesn't allow setting by role.
This sucks, now I can't derive them anymore.

Ugh-huh, and the problem with derive + generic + type macros is not this
library, it's rustc, a 5 year old bug. Shit. I can't make qt methods this way.
So is the solution the same as qt - live with shitty monomorphic types? At
least one could write macros for that.  
I think this problem could be fixed in the crate: replace the type macros with
attribute macros and hand-written types. Or not replace, but give me an option
to use them. Kind of what they do with SimpleListModel internally, but for
other stuff as well.

I can convert from rust values to qvariant, but not back. This is stupid. I can
circumvent this with conversions via json, but they are very unsafe, in a c++
way. Well, let's do it with json. Also I don't really need to do it, but I want
a generic approach.

Right, this is failing now because I used a different json structure in qml
than the TodoItem proper. Because the model has weirdness with indexes or
something, I don't remember, and also to distinguish names. So ok, you know
what, let's just rewrite it to a proper model with proper functions.

Regarding setting a value from rust: that can be done with setContextProperty
on engine, I remember now.

Oh right, there is conversion from qvariant, it's hidden in
QMetaType::from_qvariant. Ok, this is nice. Now can I convert qstring and
others like this? Yes I can, a lot of rust types have the instance. Nice.

A lot of things happening and I'm not writing about them. It's somewhat hard
since I'm familiar with qml, but there are a lot of things happening here,
where I'm surprised and then fix it with another approach. The most recent one
being that I need to create property `count` myself, it's not derived from
rowCount. At the base of it all is that I'm still not doing the model from
scratch, but am still writing based on the yesterday's bullshit, and making it
work. Oh right, I still haven't checked if it's actually read-write, let's see.

Warnings about binding loops, how I missed you (not). Well, everything works
now. Turns out qt docs are really well, written and describe my exact problem
and solution. Who would have thought, huh. Well, this is a solution with double
model sharing, now let's export it immediately from list model.

Rewriting to a proper model was piss easy. This shows that in qml using a
proper solution is simple enough anyone could do it. This is good. Still I
don't know what to do with binding loops. I wonder if I can assign to model and
then restore the binding, or should I use a bi-directional binding, I remember
qt having those.

So there are a couple of inconvenient things in this library. Documentation for
one, just copy-paste stuff from qt docs, come on. I really think that mutable
list model should be present, or at least possible. It's impossible because of
rustc, yes, but the library authors could bypass it. Some conversions are weird
and how it allows you to pass rust types and then fails in qml with
"unsupported type", this is bullshit.

Still, I like qml and this is a pretty good implementation of qml in rust,
really easy to comprehend and use. A good candidate for me.

- Default widgets: enough, alright-ish looking
- Creating simple widgets: simple
- Creating compound widgets: simple
- Layouting: simple
- Custom layouts: can do with manual coordinate setting
- Reactivity: simple
- Setting up: easy
- Documentation: great for qt, ok for library
- Tutorial: none, but qt concepts map 1 to 1
- Diving into source: good sources where c++ is not mentioned
- Overall: very approve, this is a great choice for guis right now.

## qt_widgets

The first of two autogenerated bindings. Let's see how shit this is.

> Most of the generated functions are unsafe because raw pointers are not
> guaranteed to be valid

Mehhhhh. I don't know, let's look at examples and judge from there.

Even creating basic qml application requires unsafe.

It seems this was also made before pin stabilization, as it uses a weird idiom
with rc.

This looks from neat to almost horrible. The biggest red flag is pervasive
monomorphism, the slot "types" are just crazy. I don't think this would be
convenient for a project, even without all the unsafe.  
I now think that it would be interesting to port qt-widgets to a qmetaobject
system.

So this is really not for everyone and not for me, skip.

## relm

What is the relationship between this and relm4? They share the name, the elm
architecture, and gtk, but have zero common devs and target different gtk
versions. What is going on here?  
Oh, I found it. In the relm4 readme, they say that it's a from scratch
reimplementation. Ok, makes sense, still weird that the original is ongoing and
the og developer is not participating.  
Found also relm4 developers giving special thanks to antoyo.

Hm, so gtk and gtk4 are different crates. Which one did I have problems with
with other libraries?

## Relm4

As far as I understand, I had problems with gtk3 but not with gtk4 at least
yet. So let's try this one first. Start with reading the book. Books are nice.
Relm plain doesn't have one, they only have a four page tutorial.

It seems they are going through major updates? I'm going to look at the old
version.  
No I'm not, the old one assumes version 0.2, and the new one is 0.5. It's cool
that they still consider that one stable, but this one seems developed enough,
it's been more versions already.

    sudo zypper in libgraphene-devel gdk-pixbuf-devel gtk4-devel
    The following 12 NEW packages are going to be installed:
      gdk-pixbuf-devel gettext-its-gtk4 gtk4-devel gtk4-devel-tools libepoxy-devel libgraphene-devel libjpeg62 libjpeg62-devel libtiff-devel typelib-1_0-GdkPixdata-2_0
      typelib-1_0-Graphene-1_0 typelib-1_0-Gtk-4_0

Oh yay, it built!

Fucking lol, I copy-pasted the example from the readme, and it doesn't work
because I need to replace every mention of gtk with gtk4.

Another lol with the example is that they use u8. What are you sparing the
bytes for, my dudes.

The basic concepts part of the tutorial was kind of shit. Also they still
mention gtk everywhere.

Huh, the 0.5 version breaks some api. So I have an option of either beta or the
very old thing. Hmm. The beta also installs ok.

Alright, there were a lot of changes to 0.5, that makes it more interesting.
Now there are not just messages, but app input and output. And also everything
is incorporated into single Component trait.

The autogenerated docs are also wrong! What the hell is going on here. Yes,
it's totally on me that I'm using alpha software and getting bugs, but still,
this is weird.

Changing view is not automatic? This sucks, man. I hope there is a way to make
it better. Also interesting: in update_view I receive immutable self and
mutable widgets, but in update I receive a mutable self, so in theory I can
update widgets from there? Oh no, I can't, because widgets are not part of
self. At least in this example, might be possible to put rc there in theory,
but as always, better to play by the rules.

So what if I have a thousand widgets and a thousand messages? Do I need to
update all thousand every time? Isn't this dogshit slow, unless I decompose
stuff by hand, which I can't always do? The beauty of elm and react is
automatic diffing. Also I wonder, can I send messages from parent to children
for them to update themselves.

Huh, so is this where KAS got their ugly layouts from? I thought gtk or adwaita
uses flexes like druid and friends, but here it is, buttons the size of the
window. Well, not really. For some reason, the buttons have fixed width but
unfixed height here. No clue what's happening. But at least I know that gtk
apps can have precize layout and sizing, because I've seen gnome and they can
make it look pretty, so it's just the defaults that are weird.

Macros for ui again. Guys, think about my lsp with autocompletion!

This view macro has a lot of magic. Fucking, ughhh, I'm so tired of it. It's
not better than using a whole separate language, worse in that your error
messages are now shit and there is no lsp or other language support.

But the macro with the watch attribute does restore the elm-likeness, so it's
somewhat better. I wonder if it is optimized under the hood though.

The magic square bracket syntax is for cloning vars. It's fun how c++ was the
better one here with explicit variable storage in lambdas, and in rust you have
to work around.

The problem with a thousand widgets is explicitly mentioned in the tutorial.
Bravo, relm4 authors.

Huh, now that I have hbox inside vbox, it's independent of the window size
completely. Interesting, I assumed the default behaviour would be anchoring to
parent like in KAS, but it's only so for some widgets.

Fucking gtk, you can only set the window background color with styles, no
explicit method. What is this, web or something.

This tracker thing is nice, but now I have a warning about reset not used. Also
I don't understand when do I need to use a getter versus just getting the field
value.

Oh right, it was warning for reset because I do need to call it, otherwise the
old changes are tracked again. Um, this has very narrow use cases. Would be
good to just use fucking properties and signals and slots, not all this
bullshit. Like, with macros it's almost that, except I need to send handle
signals first.

I've been ildly reading the turorial and looking forward to implementing the
examples. And I've kind of liked what I've read about components, factoriesand
grids. Gtk did get some things right.

Component and FactoryComponent are different. This is stupid, I wonder if it's
gtk thing or relm thing. Althoughhhh, if one could easily wrap unrelated
widgets into it, this would be pretty good. Like in qml the widgets for a
repeater need to be of a specific shape to accept model input, the same happens
here. So ok, this is an interesting idea with interesting implications.

FactoryComponent is more similar to a regular Component, and so far the
tutorial has covered SimpleComponent only. God damn there's a lot of
boilerplate for everything, qml is still better.

Ugh, magical variables appearing in scope magically, like qml. I hate this. I
also hate that autocomplete doesn't work. Guys, just invent your language, it's
what you're already doing anyway!

Now here's a question: can I create a factory component without a factory?

Wait, wait, the parent thing is still stupid! Since I'm constraining myself not
on factory type, but on container type. In qml I can contain my delegates
inside anything. What I was thinking about was initial data being a type
parameter, which is also here, but you also can't do anything without it, since
it's rust, as strict typed language, yo.

output_to_parent_input is also constrained on component for some reason. Mmmmmm
right now I think this is very fucking inconvenient, but it was sort-of the
same way in iced, except without factories.

I can't add a component as a child of box because it's not `IsA<gtk::Widget>`.
That is almost an auto trait, except it's unsafe. So, ughhh, can I implement
it, or is this fltk situation again?  
Hmmmm, looking at some other examples, it seems by implementing a component, I
automatically get IsA<Widget>. Let's try.

Trying to derive Component from FactoryComponent and failing. Init types are
different. Fucking alright, I was trying to learn by trying, but instead I have
to go forward and read the fucking book, right? The bad thing is, knowledge
about factories is already enough to create the todo list, but I'm not
satisfied with components here and want to find out how they work. If I knew
about gtk in advance, I would have made a task where I need to use both
components in a factory, and components by themselves in a tree. Maybe create a
custom button or something. Or make search field into a separate widget!
