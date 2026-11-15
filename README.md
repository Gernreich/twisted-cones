# Twisted Cones

**[← All projects](https://gernreich.github.io/)**

**[Open the variations](https://gernreich.github.io/twisted-cones/)**

A cone, offset so one side runs up the z axis, and twisted around that axis. Every
slice is level with the x/y plane and touches the z axis. The page shows 26
variations on a contact sheet (slice shape, how fast it twists at the base and at
the tip, the curve it tapers along, copies, a gap at the axis, a hollow wall, and a
stack of 3 mm layers), and a mixer that combines any of them, all lit and turnable
in 3D. The taper curves are linear (a cone), geometric (a horn), power 2.5 (a bell),
parabolic (a swell), hyperbolic, elliptic and cosine.

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

The base cone runs from z = 0, where its slice is 32.66 mm across, to a point on
the z axis at z = 1166.43, turning once on the way. (The four sweeps turn τ per
1000 mm of rise, which is 1.166 turns over the same height.)
At full size it makes 388 layers of 3 mm ply; at ¼ scale, 97.

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
