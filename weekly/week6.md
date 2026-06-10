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