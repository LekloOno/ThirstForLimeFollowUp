# 2026.06.12

- [2026.06.12](#20260612)
    - [Crosshair Editor](#crosshair-editor)
    - [VFX wind trails](#vfx-wind-trails)
    - [Ledge vault](#ledge-vault)
    - [Double jump propeller input rework](#double-jump-propeller-input-rework)
    - [Fixes](#fixes)

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