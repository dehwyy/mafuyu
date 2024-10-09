# Commit naming

## General

- {`TYPE`|`..MODIFIERS`} [{`FILES`/`FOLDERS`} or/and {`WHAT_WAS_DONE`}]

### Types

- `DOC` - documentation.
- `FIX` - fix unwanted behavior somewhere.
- `FEAT` - new feature.
- `REF` - code refactor (reorganized files, folders structures, etc.).
- `OPT` - code optimization (code power up!).
- `REW` - rewrite something.
- `APP` - app configuration change (f.e `svelte.config.js`).
- `REPO` - dev-only feature for repository(workspace) (f.e `prettier` config, `Makefile`, etc).
- `GIT` - change something related to `git`.

### MODIFIERS

Determines related stuff.

- `UI` - User Interface.
- `SERVER` - Backend.

### FILES/FOLDERS

Relative least easy-understanble path.

### WHAT_WAS_DONE

Notes about what was done by developer

## Examples

- [FEAT|UI] - `components/settings/bar.svelte` - add color change animation.
- [FIX|SERVER] - fix files static serve.
