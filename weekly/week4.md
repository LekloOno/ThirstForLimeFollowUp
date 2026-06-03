- [2026.05.26](#20260526)
    - [VFX\_MovingLightObject jitter fix](#vfx_movinglightobject-jitter-fix)
    - [Allow unscaled reload](#allow-unscaled-reload)
    - [Properly reset weapon Reloader on init \& reset on roaming](#properly-reset-weapon-reloader-on-init--reset-on-roaming)
    - [Fixed halted meaning for dash](#fixed-halted-meaning-for-dash)
- [2026.05.27](#20260527)
    - [Weapons sounds arch](#weapons-sounds-arch)
    - [Sound design](#sound-design)
    - [UI](#ui)
    - [Sequence Spawner bug fix](#sequence-spawner-bug-fix)
- [2026.05.28](#20260528)
    - [Proper resets](#proper-resets)
    - [First approach of the auto Sprint](#first-approach-of-the-auto-sprint)
- [2026.05.29](#20260529)
    - [Auto Sprint tweaks](#auto-sprint-tweaks)
- [2026.05.30](#20260530)
    - [Add quick start bind for the auto sprint.](#add-quick-start-bind-for-the-auto-sprint)
- [2026.05.31](#20260531)
    - [Settings menu overhaul](#settings-menu-overhaul)
- [2026.06.01](#20260601)
    - [Quick auto sprint bug fixes](#quick-auto-sprint-bug-fixes)


# 2026.05.26

### VFX_MovingLightObject jitter fix

With the reintroduced pooling system, pooled projectile would get teleported on spawn, but they were using physiscs interpollation, which led to visible jitter, as the interpollation was trying to handle the transform reset.

Disabled completely interpolation on the projectile anyways as their movements are handled by a visual shader, thus already on frame time.


### Allow unscaled reload

Make so reload time is not influenced by time scale, so the slow mo power up is more flexible to use.

### Properly reset weapon Reloader on init & reset on roaming

Weapon reloader wasn't properly reseted on ammos init, chamber and other flags could not be set correctly. Entering roaming was not initializing weapons either.

### Fixed halted meaning for dash

The "Halted" method on weapons handler was ambiguous and mixing different pseudo-halting concepts. Separated concerns.

# 2026.05.27

### Weapons sounds arch

Added low and dry ammos sound hooks for the weapon to play audio cues when ammos are getting close to empty, as well as when no ammos is left.

Added reload sound hooks for the different steps and steps transitions of the reload system.

### Sound design

Reload modular sounds and variants for each reloading steps and transitions of both the G0Z-BRT and P3-W.

Dry and low ammos base sounds.

### UI

Ammos notifications for low loaded ammos, low global ammos, very low global ammos, & empty ammos.

\+ Annimations

\+ Created a tween helper resource, that encapsulates all the redundant parameters to configure a tween.

### Sequence Spawner bug fix

There were a stack double push left from the previous sequence spawner architecture, that could lead to bugs.


# 2026.05.28

### Proper resets

Fixed the benign unrested "bug" at the end of a run which implied reworking & improving slighlty the life cycle of some elements.
- Reset UI temporary elements
  - Kills markers
  - Hit markers
  - Damage markers
  - Score markers
- Reset loadout order
- Reset reload/switching
- Reset camera rotation
- Reset camera animations
- Added an explicit "teleport" and rotation set on player node, to handle interpollation reset, velocity init, etc.

### First approach of the auto Sprint

The auto sprint was pretty tough task.

There's many underlying questions - how automatic should it be, what are the triggers, do we want some delays, how to guess the true intent of the player, when to interrupt ...

Simply sprinting whenever the player can is not truly the intent of the player and leads to confusing behaviors, we might start sprinting mid gun fight, be taken of guard and struggly to land shots.

One difficulty notably resides in the fact that the game allows to sprint fully side ways, and also allows to sprint while shooting. It's just very difficult to do with manual sprinting, as it requires many successive inputs.

With fully auto sprint, it would be extremely overpowered.

The final approach (developped further the 2026.05.29 and 2026.05.30) is the following -

The base triggers are 
- Pressing "forward" key
- if "forward" is already pressed and :
  - a key is pressed
  - crouch/slide is stopped
  - ADS is stopped
  - grace period ends

Grace period is a system that stops sprint on shot, and automatically relaunch sprint when ended, the grace period is extended as long as the player keeps shooting, and intended at balancing the overpowered straffe shoot sprinting ability that gives auto sprint.

Initially, I wanted the user to still be able to shoot with auto sprint, by overriding the grace with a clear intent, but it revealed to be difficult to correctly balance, and not make confusing to use. Often, the user would start a sprint in grace although he didn't really meant to.

I decided to simply drop the sprint + shoot with auto sprint.

Later, I added the ability to quick start the sprint on grace with a dedicated configurable input, and also to configure the grace delay.

I might later make so the delay is minned out on the shooting weapon fire rate time.

# 2026.05.29

### Auto Sprint tweaks

Slightly modified approach to the grace system of the auto sprint.
see [commit changes](https://github.com/LekloOno/Get-Deaded/commit/850a6aacd3be33429e51ce228c90ab1c41facdba).

# 2026.05.30

### Add quick start bind for the auto sprint.
Allows to start the auto sprint early in the shooting grace period.

# 2026.05.31

### Settings menu overhaul

- Improved readability with clear categories, more compact layout, etc.
- More maintanable structure with diverse reusable and tweakable subscenes, standardized modules (numeric inputs, reset buttons, ..)
- Embed everything in scroll containers
- Disabled state theme for unused inputs in keybinds config
- Few bug fixes
  - Do not reject scale setting on disabled scale, keep in cache
  - Correctly update max fps on enable/disable
  - ui_num_input - correctly intialize to attached slider
  - TraGUS line edit - fix float parsing
- Sprint options
  - Mode - Toggle, Hold, Auto
  - Auto delay
  - Auto quick start bind

# 2026.06.01

### Quick auto sprint bug fixes

Some intialization was missing, and missleading callbacks.