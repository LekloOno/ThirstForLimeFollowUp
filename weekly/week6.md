# Week 6

- [Week 6](#week-6)
- [2026.06.09](#20260609)
    - [Crosshair editor](#crosshair-editor)
    - [ui\_num\_input propagation fix](#ui_num_input-propagation-fix)
    - [Time scaler](#time-scaler)
- [2026.06.10](#20260610)
    - [Jump mechanics tweak](#jump-mechanics-tweak)
      - [Fatigue](#fatigue)
      - [Coyote time](#coyote-time)
    - [Crosshair editor](#crosshair-editor-1)
      - [Renderer](#renderer)
      - [Layers order edit](#layers-order-edit)
      - [Presets \& Saved](#presets--saved)
    - [Tres scanner](#tres-scanner)
- [2026.06.11](#20260611)
    - [Crosshair editor](#crosshair-editor-2)
      - [Fixes](#fixes)
      - [Save, import, export system](#save-import-export-system)
- [2026.06.12](#20260612)
    - [Crosshair Editor](#crosshair-editor-3)
    - [VFX wind trails](#vfx-wind-trails)
    - [Ledge vault](#ledge-vault)
    - [Double jump propeller input rework](#double-jump-propeller-input-rework)
    - [Fixes](#fixes-1)
- [2026.06.13](#20260613)
    - [Double jump propeller input UI](#double-jump-propeller-input-ui)
- [2026.06.14](#20260614)
    - [Jump pad rework \& bugs](#jump-pad-rework--bugs)
    - [Double jump propeller input UI fix](#double-jump-propeller-input-ui-fix)
    - [Update tragus defaults](#update-tragus-defaults)
    - [Sequence Spawner random position clip fix](#sequence-spawner-random-position-clip-fix)
- [2026.06.15](#20260615)
    - [Crouch mode setting](#crouch-mode-setting)
    - [Slam rework](#slam-rework)
    - [TraGUS .NET interoperability rework](#tragus-net-interoperability-rework)

# 2026.06.09

### Crosshair editor

A new very modular crosshair editor, saved into your user settings.
You can create mutliple layers and combine them in various ways.

The editor would benefit some more quality of life and UI clarity, but the goal was just to provide a functionnal feature to the player, we'll dive in polishing later.

### ui_num_input propagation fix

ui_num_input gdscript would not propagate its changes properly to possible settings bind. It was initially intentionnal, but make binding to settings less convenient, so it got fixed in a better way, to keep the initial intentions while still properly propagating.

### Time scaler

I use a wrapper of engine ticks to follow time stamp with active time scale, that allows to have mechanics based on such tim stamps to scale with the engine time scale, typically powering slow-mo consistently for anything time-related.

This wrapper was unaccurate, as errors could accumulate over time. This notably led to the leaderboard reporting full run time of 77 seconds instead of 80.

The problem was that, the wrapper accumulates time in milliseconds (and microseconds) every tick, but the time elapsed between two ticks might contain fraction of such units.

Typically, a tick is 16.67 ms, as the game runs at 60 ticks/sec, so each tick, the wrapper would lose 0.67 ms when trying to store it in the ms accumulator.

Over 80 seconds, this 0.67 unaccuracy builds up to exactly 3200 ms, explaining exactly the "77 seconds" bug (in db, it is actually 76800 ms, so indead, exactly 80000 - 3200, rounded up to 77 seconds.)

The wrapper now takes this into account, and safely accumulates the ms/us fractions.

# 2026.06.10

### Jump mechanics tweak

#### Fatigue
Ledge climb now resets the jump fatigue, to make movement more fluid. It'd be common for a player to use a ledge climb to gain height, and combine with a jump for even more height. It'd be a bit frustrated to be locked by a fatigue jump, or have to wait for a full jump. Reseting the fatigue does not make it overpowered, and allows much more fluidity.

#### Coyote time
A coyote time was in the feedback list since a little while, it was quite easy to implement, so here it is.

### Crosshair editor

#### Renderer
Renderer now renders from bottom layer to top, so the top layer in the resource inspector is the one rendering on top.

#### Layers order edit
Added calls to move up, down and to some shape layers in the crosshair data, as well as new corresponding hooks and UI and adapted renderer syncer.

#### Presets & Saved
The editor now comes with a list of selectable presets, and also allows the user to save its own crosshairs and reuse them.

### Tres scanner

I use a little rust script to save my weapon assets as json registries, since relying on `res://` directory exploration is not reliable in export build in godot 4.6.

I updated this script to be more flexible, so it's easier to use for any kind of data. Notably, so I can save crosshair presets.

# 2026.06.11

### Crosshair editor

#### Fixes

Fixed a few bugs in the presets system, notably protect presets from edition.

#### Save, import, export system

The editor now comes with a fully functionnal import/export/save system.
You can save custom crosshairs, and share them by exporting/importing them in a pretty straight forward UI.

# 2026.06.12

### Crosshair Editor

Clean some code, refacto in more reusable and maintanable way + externalized all the "mode to behaviors" in clean extensions methods.

### VFX wind trails

Created a shader that traces some wind trails, with configurable density, frequency, trail length, speed, etc.

This shader is now used along the player's speed, like the wind sound that was already setup, and another layer is played when the player performs a ledge vault.

### Ledge vault

Ledge vault now has vfx (read above) as well as an associated sound.

### Double jump propeller input rework

The previous bind mode was unintuitive.

To do a double jump, you had to press jump right before pressing a dash input.

This was designed to avoid conflict between multiple jump-related intentions, but it wasn't a great solution.

The jump+dash input mode is still available, but now, the player can pick among 3 modes :
- jump + dash, as previously
- jump fallback + height - the double jump uses the same jump buffer as other jump related abilities, but it is the last consumer, and will only consume it if the player is above a certain minimum height to avoid bhop conflict.
- dedicated input - simply setup a dedicated input.

The user can mix different modes, as he wishes, although jump + dash and jump fallback + height can't really be used together.

### Fixes

- GL_Picker was not cleansing damage multiplier, we moved responsibility of cleansing effects to the GL_Picker previously, but forgot to handle damage multiplier. 


# 2026.06.13

### Double jump propeller input UI

Added the proper ui elements for the user to be able to edit the new double jump modes and options.

# 2026.06.14

### Jump pad rework & bugs

Jump pad scene and script have been reworked to be easier to edit and play around with.

Besides, the sound ""bug"" has been fixed, the jump pad now correctly uses a spatial sound instead of a 1D player.

The jump pad + dash bug has also been (partially fixed). The bug itself won't happen anymore (dashing into jump pad would propulse the player with the dash speed, so, incredibly high speed), but the fix is temporary, it's not a true dash cancel, ideally, the player's dash should get cancelled on jump pad hit, and the jump pad properly triggered. Currently, it's just the force of the dash that is cancelled, but the dash still resets the player's speed at its end, cancelling the jump pad momentum too.

Jump pad also now have a small subtle light to make it more obvious it is interractible.

### Double jump propeller input UI fix

It wasn't possible to unselect both dash and jump modes in double jump mode ui. It is now possible.

### Update tragus defaults

Updated outlines default settings, and added the double jump mode default setting entry.

### Sequence Spawner random position clip fix

The new spawning option, to use a list of specific positions, was not checking for potential clipping. Typically, two enemies could spawn at the exact same position, at the same time, making them clip and resulting in unexpected physics soup.


# 2026.06.15

### Crouch mode setting

Added a crouch mode setting, to allow hold or toggle mode.

### Slam rework

Slam has been extracted from the dash code, it is now an independant component, just like the double jump prior to that.

Besides, its input mode has been reworked. It now has similar modes to the double jump -
- Dedicated input, for an explicit input to be used
- Quick crouch/tap crouch - triggers when the crouch is quickly started and stopped, that is a quick tap of the input in hold crouch mode, and a double tap in toggle mode.
- Dash+Crouch, the old method, that triggers when dash is pressed right after a crouch input.

The slam now also has its own dedidated sound.

The independant nature of this new slam made a a bug immerge, that was pre-shoted, even before it was actually witnessed, but that I kept as I thought it might be actually interesting - it should be possible to start a dash as you're still slamming, and this would base the dash velocity on the slam velocity, which can grant huge amount of speed. (It should also be possible to slam as you're dashing, but there isn't much obvious benefits out of this.)

Indeed, it was observed in play test, but as for now, I feel this mechanic is kinda fun, and don't plan to fix it. Maybe we should better control the extend to which this mechanic can work, eventually. For now, I'll leave it as is.

### TraGUS .NET interoperability rework

TraGUS was based on huge code smell.

I was using a static instance inside the base abstract class, thinking it could act as a static reference in a rust trait, but c# works differently, and the instance would be overriden by each new setting.

For most of the code, it did not make much differences. Many of the ui glue was developped in gdscript, so not using the static reference but the autoload reference. But it would eventually break, and it did, while developping the crouch mode setting.

I reworked the way UserSetting is thought for .NET interoperability, by only introducing the concept of static instance inside a UserSetting<T> super class, ensuring the instance is unique to each implementor.

Besides, I added even more .NET support, with different levels of wrapping, notably a UserSetting<T, U> which allows to handle UserSetting with explicit and strict types, as well as a familly of templates that preimplement most of the boilerplate, for diverse types, such as different numeric types, enums, flags, etc.