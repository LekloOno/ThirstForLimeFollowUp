# Week 7

- [Week 7](#week-7)
- [2026.06.16](#20260616)
    - [FistsFire charge ui](#fistsfire-charge-ui)
    - [Elaborating a pitch](#elaborating-a-pitch)
- [2026.06.17](#20260617)
    - [Hit sound various work](#hit-sound-various-work)
    - [Quick GaudioProcessTree fix](#quick-gaudioprocesstree-fix)
    - [Further project documentation](#further-project-documentation)
- [2026.06.18](#20260618)
    - [Hit sound tweaks](#hit-sound-tweaks)
    - [More intuitive wall climb](#more-intuitive-wall-climb)
    - [Pickup related things](#pickup-related-things)
    - [Scores server status feedback](#scores-server-status-feedback)
    - [Window mode fix](#window-mode-fix)
    - [Num inputs rounding](#num-inputs-rounding)
- [2026.06.22](#20260622)
    - [Ledge climb check](#ledge-climb-check)
    - [GC pressure exploration](#gc-pressure-exploration)
    - [Allow weapon use in round start](#allow-weapon-use-in-round-start)
    - [New enemy incoming !](#new-enemy-incoming-)
    - [Fixes](#fixes)

# 2026.06.16

### FistsFire charge ui

Some UI to show the fists fire charge, and availability.

### Elaborating a pitch

In prevision of the meeting next week, I started to build a condensed pitch, trying to make the quite niche goal of the project a little more understandable to anyone.

[See](https://docs.google.com/document/d/17fMLzRhGlCepeqF8gGYqdo9ovhICEIocWbpH2pzWGpY/edit?usp=sharing)

# 2026.06.17

### Hit sound various work

Added armor and barrier specific hit sounds, and reworked a bit the flesh, headshot, and kill sound.

Besides, the kill sound pitch now scales with multi-kills.

### Quick GaudioProcessTree fix

Fixed a bug on initialization of time scale.

### Further project documentation

[See](https://docs.google.com/document/d/17fMLzRhGlCepeqF8gGYqdo9ovhICEIocWbpH2pzWGpY/edit?usp=sharing) - Added very brief discussion of the visual & sound references, as well as a quick view of the background story.

# 2026.06.18

### Hit sound tweaks

Continued tweaking hit sounds.

### More intuitive wall climb

Wall climb can now be performed even with no prior speed, and even if the player is in a falling state.

When on ground with no speed, instead of the initial wallclimb hop, the player simply jumps.

I plan to rework the whole wall interractions later, so I kept it simple for now.

### Pickup related things

Fixed a bug where the slow mo pickup loading UI was using the viewport of the damage buff pickup.

Made the loading ui a little more visible, and quickly cleaned some simple warnings in the pickup related codes.

### Scores server status feedback

While waiting for the score API to respond, we now display a little loading icon.
Besides, if any error occurs, it is displayed in the scoreboard.

The scoreboard is also now scrollable.

### Window mode fix

Windowed mode was buggy, fighting against OS-level optimization. One of the problem was that windows(OS) optimizes windows(program windows eh) that cover the entire screen as fullscreen, and it seems its using the fixed project viewport size as the "windows size" for some reason ? The current, quirky, fix, is to use a different view port size than screen size.

### Num inputs rounding

Numerical input tied to settings or sliders would display floating point mess, as they would simply display the raw value when inputed from an external source.

It now applies its "decimal" property rounding to the visual for readability sake (only visual, no repercusion on the actual values).

# 2026.06.22

### Ledge climb check

The new more forgiving ledge climb check was too permissive. It was possible to position at a specific angle at the edge of a wall where it would be possible to ledgeclimb, although it should not, and thus, to infinitely ledge climb to space.

The check was
- If we can move forward, with no collision, abort
- If there's a collision, check along its normal if there's another collision at "head level", using a simple raycast.

Originally, instead of a raycast, we were doing a shape cast, with the same width that of the player. The problem was it was too restrictive, in some scenarios, the head cast would indeed collide with a wall, typically in tight corner, although the player can realistically climb over the ledge with no problem, he would just get pushed a bit on the side of the wall.

The ray cast is a little too permissive however. So now, the new approach is instead to use a thinner hitbox, both for the head cast AND the player. Having a thinner hitbox for the headcast would just push the issue of the raycast in a more niche edge case, so not really solving the problem. Instead, the "can move forward" check is performed with a thinned hitbox, and the head cast uses the same width, thus, there cannot be the same false positives as with ray cast as both hitboxes uses the same width, there cannot be false negatives dued to the head cast, there's just false negative dued to the initial move forward check. These false negatives are much more acceptable, as it could be understandable that the player can ledgeclimb when the ledge is barely on the edge of his far right/left. How would he even grab on the ledge in such scenario .. I think it is the best compromise ! 

To play test to confirm that this false negative is fully intuitive to players.

### GC pressure exploration

A player had a problem with periodical freezes, that scales with framerate. The most obvious suspicion was a GC pressure problem, but it seems not to be (to further test with the said player).

A few optimizations have still been done, but we will need more specific tests to investigate this issue.

### Allow weapon use in round start

Weapons were disabled at the start of the round, during the start count down. It is now possible to use them, to do some specific setup, like pre-charging the melee, but later, it could also be, use explosive weapons to propulse yourself, or anything else.

### New enemy incoming !

Wip on a new floating enemy - this included many works -
- New enemy base script
- An interface for general enemy components, with template methods
- various components (floating mover, target acquirer, ...)
- rework of simple enemy spawner with a pool system

### Fixes
- Weapon manager did not apply the same rules to forward primary input release as primary input press, although some weapon might use the release as others use the press. For instance, "disabling" the fists wouldn't fully disable it, since it was still possible to send a release, that actually sends the hit.