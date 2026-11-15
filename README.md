# Twisted Cones

**[← All projects](https://gernreich.github.io/)**

**[Open the variations](https://gernreich.github.io/twisted-cones/)**

A cone, offset so one side runs up the z axis, and twisted around that axis. Every
slice is level with the x/y plane and touches the z axis. The page shows 31
variations on a contact sheet (slice shape, how fast it twists at the base and at
the tip, the curve it tapers along, copies, and a gap at the axis), and a mixer that
combines any of them, makes it hollow with a wall
from 0.2 to 10 mm thick (to print with less filament), cuts it off at 10% to 100% of its height (each of
the taper, the twist and the shape change just stops there, or is ticked to finish at the cut), adds a circular base 1 to 100 layers of 3 mm thick, and
downloads the shape as an STL file. An x,y slider sets the
width from 0.1 to 100 times; at 1 the shapes are four times as wide as the first
sweeps (about 150 mm out from the axis). Gap, wall and base stay true mm, all lit and turnable in 3D. The taper curves are
linear (a cone), geometric (a horn), a power curve (1 − u)^k with k on a slider from
0.1 to 5 (2.5 is a bell, 0.5 a dome), parabolic (a swell), hyperbolic, a closed hyperbola (the hyperbola shifted to end in a point), the
closed hyperbola turned τ/4, logarithmic, the log turned τ/4 (an exponential),
elliptic and cosine. Turning a curve τ/4 here means turning its profile a quarter
turn in the height-by-size square, so it bulges out where it used to bulge in.

It grew in four steps, each kept as its own page:

1. [Twisted ellipse sweep](https://gernreich.github.io/twisted-cones/series/twisted-ellipse-sweep.html):
   an ellipse from (10, 0, 0) to (0, 0, 190), y within ±10, rising 1000 while it
   turns once and shrinks 10 : 1 about its end on the z axis; with its mirror.
2. [Spiral ellipse sweep](https://gernreich.github.io/twisted-cones/series/spiral-ellipse-sweep.html):
   the same ellipse moved out to (20, 0, 0)–(10, 0, 190), shrinking about its centre
   while the centre spirals in from radius 15 to 1.
3. [Spiral circle sweep](https://gernreich.github.io/twisted-cones/series/spiral-circle-sweep.html):
   a level circle centred on that spiral, its edge touching the z axis: a slanted
   cone, twisted.
4. [Elliptic cone sweep](https://gernreich.github.io/twisted-cones/series/elliptic-cone-sweep.html):
   level 1 : 2 ellipses instead of circles. A straight cone tilted until one side
   lies on the axis would also have elliptical level slices, but this cone is so
   slender (33 mm across, 1166 mm tall) that it leans only 0.80° and its slices
   are 0.999902 round, so the ellipse is set directly.

The four sweeps are 1166.43 mm tall. The variations page scales that design down
to 240 mm tall so it prints on a Bambu Lab P1S (256 × 256 × 256 mm build volume),
then builds it four times as wide as the sweeps: at x,y × 1 the base cone's slice
is 26.9 mm across at z = 0 and it reaches a point on the z axis at z = 240, turning
once on the way. The measurements show the printed size and whether it fits the
P1S; the grid under the model is its 256 × 256 plate. (The four sweeps turn τ per
1000 mm of rise, which is 1.166 turns over their height.)

## Files

- `index.html`: the contact sheet and the mixer. Saved mixer combinations stay in
  the viewer's browser.
- `series/`: the four pages above.
- `src/main.rs`: recomputes every figure the pages quote, from the formulas the
  pages draw with, and fails if any is off. Build and run with
  `cargo build --release && ./target/release/twisted_cones`; `Cargo.toml` names the
  crate and its authors, and `Cargo.lock` pins the build.

The 3D views use the Three JS library, release r128, loaded from cdnjs.

## Licence

CC0 1.0: see `LICENSE`.

## Credits

The shapes and the twisted, offset cone are Michael Gernreich's ideas;
[Claude](https://www.anthropic.com/claude) (Anthropic), working in Claude Code,
wrote the pages, the checks and this README at his request. Commits Claude made
carry a `Co-Authored-By: Claude` line.
