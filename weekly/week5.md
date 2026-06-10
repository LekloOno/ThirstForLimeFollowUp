# 2026.06.02

- [2026.06.02](#20260602)
    - [TraGUS updates](#tragus-updates)
    - [Quality settings](#quality-settings)
    - [Quality preset](#quality-preset)
    - [Confirm dialogs](#confirm-dialogs)
    - [Return buttons](#return-buttons)
    - [Double light setup](#double-light-setup)
- [2026.06.03](#20260603)
    - [Enemies aim \& rotation rework](#enemies-aim--rotation-rework)
    - [Optimizations](#optimizations)
    - [Better ledge climb](#better-ledge-climb)
    - [Fixes](#fixes)
- [2026.06.04](#20260604)
    - [Recoil system](#recoil-system)
    - [Misc](#misc)
    - [Fixes](#fixes-1)
- [2026.06.05](#20260605)
    - [Scripted spawn position](#scripted-spawn-position)
    - [Low health cues](#low-health-cues)
    - [Enemy rotator/aimer](#enemy-rotatoraimer)
    - [Fixes](#fixes-2)
- [2026.06.07](#20260607)
    - [Misc](#misc-1)


### TraGUS updates

Dirtiness tracking - update a flag when settings are modified to be able to check if there's unsaved changes

Changed signal value bug fix - the emited value was not the actual effective value, which is missleading. Updated it so it does emit with the effective value.

### Quality settings

Added various new settings for video quality
- Shadows (positionnal & directionnal)
- Ambient occlusion
- SSIL
- SSR
- Glow quality
- Anti aliasing (not yet very stable with the current outlines shader)


\+ localization  
\+ default config

### Quality preset

Added a preset button for quality settings, that automatically update the different individual quality settings.

### Confirm dialogs

Some menus interractions deserved a confirm dialog.
- Quitting
- -leaving/surrender game
- saving/aborting settings changes.

Also slightly changed the style of the confirm dialog.

### Return buttons

Some menus did not include a return button. I'm used to rely on pressing escape, but some users might be confused. Having return clickable buttons wherever it could be required ensures the user is not lost !

We could later add a Escape key hint next to the button. 

### Double light setup

Shadows can be difficult to render, especially on low-end devices. There's muitliple techniques to render them, but to consume little resources, they will tend to either look very blocky, have clear different resolution steps, be grainy/have acnee, and many other artifacts, very obvious to the user.

One way to completely avoid all these issues, is through light baking.

Light baking is a process that allows to pre-render complex lighting behaviors as textures and masks. I already used it to compute indirect and ambient lighting, which can help produce pretty realistic looking light with very little resources.

But it is also possible to bake casted shadows in the light baking process, into a shadow mask.

Yet, this shadow mask is of course baked FROM static elements, and FOR static elements.
It means it cannot be used to cast shadows onto dynamic elements, and dynamic elements do not contribute to this mask.

For example "it cannot be used to cast shadows onto dynamic elements" - let's take a static umbrella, with which the shadow mask is baked, it will thus cast a static shadow onto the terrain. But if a character goes under this umbrella, it will not receive that static shadow, thus remain fully lighted if no dynamic shadow is produced by the umbrella, which will look odd.

On the other hand, "dynamic elements do not contribute to this mask", it means if the character now moves out of the umbrella, if the terrain only rely on its static baked shadow mask, it won't receive the shadow of the character, as it is dynamic.

The best compromise would be to go with a hybrid system, combining static and dynamic shadows.

We could simply overlay shadow mask and dynamic shadows, but the shadow mask is almost always better looking that the dynamic shadows, and overlaying both can still let us see all the artifacts produced by the dynamic lighting, in places where we could simply fully rely on the shadow mask (that is, the static elements).

So instead of an overlay, we can split the light in two -
- One light is casted on the static elements only, (e.g. the terrain) and has shadows casted by the dynamic elements - the terrain can thus receive the casted shadows of dynamic elements (character going out of the umbrella example)
- The other light is casted on every elements but the static ones, and has shadows casted by every element, including static ones - the dynamic element can thus receive the casted shadows of the terrain (character under the umbrella example), but the terrain does not receive its own shadow (avoid blocky overlay).

In this setup, the static elements will always only receive the combination of the static shadows from itself + the dynamic shadows from dynamic elements, no mix up.
The dynamic elements will receive all shadows.

# 2026.06.03

### Enemies aim & rotation rework

The aiming and rotation behaviors are now extracted as independant components, to unbloat the E_Enemy script a bit.

Besides, they have been reworked.

The rotation now introduces a lerping speed for the rotation, it is not instant anymore, and a "behind" delay has also been introduced : when the player is behind the enemy, it won't turn around right away, but have a little configurable delay, and as long as the player is considered behind the enemy, he won't shoot.

This notably finally allows the player to benefit from the back stab mechanic !

### Optimizations

I further optimized the game with very slight changes -

I pushed occlusion culling further, by completely covering the map with the right occluders, and occluded masks.

I made so enemies animations are stopped whenever they are out of screen, centralized aim randomization, and made so sight detection and spread computation is done on an evenly spread 16 ticks, that is, each enemy with a given id computes spread and sight detection for every `tick & 15` equal to `id & 15`.

### Better ledge climb

The ledge climb would not feel super intuitive in some cases, and its detection margin weren't great, leading to unexpected behaviors.

The ledge climb works by multiple spatial detection, but notably, it is only possible to do a ledge climb if the "high head" cast is not obstructed. This cast was performed in the direction of the playe's view, but it was not representing the actual climb intent well.

The detection now uses the ledge normal as the head cast direction instead, with a simple ray cast. Might use a shape cast later on to avoid edge cases were the ray cast passes in small holes.

The ledge climb stop margin and minimum height margin were also not ideal, and could lead in some object bein unexpectedly un-climbable, or some climb to last longer than expected when trying to climb over very narrow platforms. They have been adjusted.

Besides, it is now possible to cancel ledgeclimb by pressing the backward key.

### Fixes

- Sprint mode would not update properly dued to ambiguous UserSetting.Instance reference.
- Dash would cancel melee completely if the active weapon is melee, although it should just sleep it.

# 2026.06.04

### Recoil system

Recoil is now fully frame independant !

The system was already "delta-scaled" but accumulation of accuracy errors could lead to significant difference by frame rate.

The recoil system is now fully controlled in the fixed-physics process, and interpollated in frame process to keep the visuals smooth.

The responsibility of the camera control and its API have also been clarified, and is now more robust.

### Misc
- Scoreboard entries now also have a hover/click sound
- Backstab angle is now a little wider, thus more forgiving
- A little background wind sound

### Fixes

- Switch to pre-condition was wrong, the direct switch to weapon was hard-stucked if you holstered the same weapon previously.
- Some buttons were not bound to their hover/click sound.

# 2026.06.05

### Scripted spawn position

Added an option in SC_SequenceSpawner to spawn enemies randomly in a list of positions.

### Low health cues

Added some cues to help the player feel that he's in danger -
- a low health red pulsing vignette when his health is getting low (flesh health)
- filtering and reverb on the sfx bus when he's near death

### Enemy rotator/aimer

Made the rotator and aimer modules fully independant.

Rotator would be responsible for updating the shooting state when the player was in the back of the enemy.

This was made to avoid recomputing angles, but it is not worth the spaghetti.

Much clearer to have truly independant modules.

###  Fixes

- Melee did not benefit from damage pickup
- Keybinding settings weren't properly updating the "hasBeenModified" flag on settings server
- Random position generator for SC_SequenceSpawner now iterates on occluded space, to try and find a free position, avoiding spawning ennemies inside walls.


# 2026.06.07

### Misc
- Made enemy an PHX_ListenPoolObject instead of simple pool object
- Balanced the double jump, its physics and reduced cost