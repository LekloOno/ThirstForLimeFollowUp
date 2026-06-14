# 2026.06.12

- [2026.06.12](#20260612)
    - [Crosshair Editor](#crosshair-editor)
    - [VFX wind trails](#vfx-wind-trails)
    - [Ledge vault](#ledge-vault)
    - [Double jump propeller input rework](#double-jump-propeller-input-rework)
    - [Fixes](#fixes)
- [2026.06.13](#20260613)
    - [Double jump propeller input UI](#double-jump-propeller-input-ui)
- [2026.06.14](#20260614)
    - [Jump pad rework \& bugs](#jump-pad-rework--bugs)
    - [Double jump propeller input UI fix](#double-jump-propeller-input-ui-fix)
    - [Update tragus defaults](#update-tragus-defaults)
    - [Sequence Spawner random position clip fix](#sequence-spawner-random-position-clip-fix)

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