# 2026.06.09

### Crosshair editor

A new very modular crosshair editor, saved into your user settings.
You can create mutliple layers and combine them in various ways.

The editor would benefit some more quality of life and UI clarity, but the goal was just to provide a functionnal feature to the player, we'll dive in polishing later.

### ui_num_input propagation fix

ui_num_input gdscript would not propagate its changes properly to possible settings bind. It was initially intentionnal, but make binding to settings less convenient, so it got fixed in a better way, to keep the initial intentions while still properly propagating.