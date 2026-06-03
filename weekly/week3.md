- [2026.05.19](#20260519)
    - [Export fix](#export-fix)
- [2026.05.20](#20260520)
    - [Optimization on enemy pooling](#optimization-on-enemy-pooling)
    - [Roaming mode](#roaming-mode)
    - [Slow mo pickup](#slow-mo-pickup)
    - [Map design](#map-design)
    - [Fixes](#fixes)
- [2026.05.21](#20260521)
    - [Tweaks](#tweaks)
    - [Fixes](#fixes-1)
    - [Enemies rendering](#enemies-rendering)
- [2026.05.22](#20260522)
    - [Fixes](#fixes-2)
    - [Outlines \& enemies color](#outlines--enemies-color)
- [2026.05.23](#20260523)
    - [GaudioProcessTree](#gaudioprocesstree)
    - [Juice](#juice)
- [2026.05.24](#20260524)
    - [Fixes](#fixes-3)
    - [Scoreboard update](#scoreboard-update)
- [2026.05.25](#20260525)
    - [Optimization](#optimization)
    - [Post-feedback first adjustments](#post-feedback-first-adjustments)
    - [Reload system overhaul](#reload-system-overhaul)
    - [Linux export](#linux-export)


# 2026.05.19

### Export fix

The export was riddled with errors, that were not present on editor/debug mode.

- Explicitly include .ini files in export
- Add a client side .sln
- Audio Bus parsing error fix
- Fixed the scoreboard ui - was using "Tab" control node wrong.
- DATA_WeaponRegistry - don't rely on LoadDir - built an explicit json registry instead.
- Remove some leftover gdscript benign warnings
- Disabled physics interpolation on spectator camera as it is animated on frame time anyways.

# 2026.05.20

### Optimization on enemy pooling

I made wrong assumption that made my pooling system very unefficient.

Typically, I thought moving the enemies' meshes out of the render pipeline would stop computing their skinning, but it's not true.

I also thought Stopping the ragdoll physics simulation would fully disable it, but constraints and other stuffs can still be computed.

There's a few other thing, like animation tree still playing, sub nodes that could still run, etc, that I could have done.

So i explicitly disabled ragdoll physics, set the enemies process mode to disabled, disabled skeleton, animation tree, etc. The performance improvement was massive, from ~140 to 240fps on my machine in the best scenarios.


### Roaming mode

I was planning on adding a slow mo pickup, and I wanted to experiment a bit with map design to place the pickup. It would be convenient to be able to freely roam on the map to test things out, so I added a roaming mode, with no enemies and infinite time, that the player can also use to experiment with the map and look out for secrets.

### Slow mo pickup

A major pickup, like damage buff, that slows down time for a brief amount of time.

\+ Sound design
\+ effect tweening

### Map design

Adapted the map to add the slow mo pickup.

Added a new area in the map, as well as a special rollout route to reach the pickup.

### Fixes 

- GaudioProcessTree PitchScale hook on AUD_StreamPlayer was wrong, and recursing on itself.
- Hide play button when playing
- Add a guard to ensure it is not possible to start a game as it is already started anyways.
- More fixes on the bugged sequence spawner (it was eventually fully fixed a few weeks later ..)

# 2026.05.21

### Tweaks

- Scoreboard new score highlight style
- Longer slow mo
- Slow mo UI and adapted damage buff UI to match it
- More configurable pickup spawner with "one shot" option to spawn the item only once, and "initial delay" to make the spawner start with a different delay than its cycle delay, typically to start early.

### Fixes

- Event naming convention partially updated
- Sequence Spanwer dynamic pool fix - properly bind newly created enemies to main sequence hooks
- Added missing reload settings in the controls binds config menu
- Clean some benign unvalid disconnection errors
- Properly cleanse player effects on game reset.
- Fix critical accuracy db request bug - rounded for no reason, leading to either 0 or 1

### Enemies rendering

- Decimated the enemy models for much lower poly count.
- Externalized the enemy materials animations on a dedicated handler, to abstract the responsibility away and make it more modular and adaptative.
- Centralize the enemy builder resource
- Outlines and X-ray shader

# 2026.05.22

### Fixes

- Missing disable hook on sequence Spawner and update on enemy ready.
- Use https on submodules. They are public anyways, ssh makes the repo un-pullable from other users.

### Outlines & enemies color
- sRGB explicit linear conversion for outline shader color to better match picked color.
- Update enemies color dynamically on (some) UI elements, instead of on use.
- Full outlines settings (thickness, opacity, color)


# 2026.05.23

### GaudioProcessTree

Updated GdPTree for a more robust pitch time scaling approach, leaving relative pitch scale free to use at runtime. 

### Juice

- Added head *ding* hit sounds that scale pitch with damages.
- Enhanced the damage indicators with a clearer style and font, adaptative animations, direct, stacking, etc.
- Added low cost grid hit decals shader on enemies.


# 2026.05.24

### Fixes

Fixed Fps Limiter-MaxFps communication borken by interoperability.

The two settings would not correctly influence each others.

### Scoreboard update

Update scoreboard (DTO + API + Client view) to show only the best score for each player + new score, new pb, general pb ... 

Enhanced style for clarity.

# 2026.05.25

### Optimization

- Compute enemies aim only when necessary
- Reintroduced the pooling and preloading system for weapon trails
- Optimize moving trails using dedicated shaders with instance properties instead of forcing mesh regeneration through cpu calls

### Post-feedback first adjustments

- Don't cancel sprint on slide/crouch/jump, just delay
- LedgeClimb - do not keep momentum, except if player is supergliding
- Death crouched bug fix - Crouch wasn't properly reseted on exit tree as it didn't have the time to physic-check the uncrouching.
- Improve the item picker, pickups and vacuum by making them much snappier, for a more predictable and less frustrating experience.
- Buff armor

### Reload system overhaul

Extracted the reload system from the weapon manager into a dedicated and enhanced component.

The new reload system is divided in different steps :
- **Unload**
  - **optionnal** depending on the weapon's type.
  - **step** in which currently loaded ammos are unloaded, to let new ammos be loaded (for example, removing the emptied magazine)
- **Insert**
  - **step** in which news ammos are inserted in the weapon, for example, a magazine or a shell. (for now, it is thought as a bulk insert, but it can later be adapted to support successive individual shells/bullets loading)
- **Chamber**
  - **optionnal** depending on the weapon's type.
  - **step** in which an ammunition is loaded in chamber.
  - **skipped** if a bullet was already in chamber, for example if the player did not fully empty the magazine on an automatic weapon.
- **Recover**
  - **step** to get the weapon operationnal after completing the full reload.
  - **skipped** if the player cancels the reload during this step, for example by switching weapon, quick melee, etc.

This new system provides more flexibility to the player, is thus less punitive, and provides more depth and skill (with various exploitable cancels/skips), and more realistic/immersive, and can easily be bind to an animation and sound system that follows these hooks.

This system also introduce auto-reload actions, on shooting dry, or when canceling the reload and coming back to the weapon, to make the experience much more fluid and intuitive.

### Linux export

Configured the export for linux and tested it.