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