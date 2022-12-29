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

## core-foundation
Macos again. And the previous one is supposed to worn on GNUStep, what's that?

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

Oh yiss, qml is nice. I's so easy to create a simple interface with, with
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
Immediate mode, skip.

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
But I remember being very interested in flutter myself, kind of liking it's
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
- Overall: approve, very easy to start using right no

## imgui

Immediate mode, so no. Imgui for C++ was nice, so try using this and see how it
goes.

## iui

Abandoned, based on abandoned C library, which was in mid-alpha, as author says.

## KAS
Time spent: a couple of hours (so far) + a day

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
