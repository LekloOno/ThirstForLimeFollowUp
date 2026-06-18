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

### Window mode fix

Windowed mode was buggy, fighting against OS-level optimization. One of the problem was that windows(OS) optimizes windows(program windows eh) that cover the entire screen as fullscreen, and it seems its using the fixed project viewport size as the "windows size" for some reason ? The current, quirky, fix, is to use a different view port size than screen size.

### Num inputs rounding

Numerical input tied to settings or sliders would display floating point mess, as they would simply display the raw value when inputed from an external source.

It now applies its "decimal" property rounding to the visual for readability sake (only visual, no repercusion on the actual values).