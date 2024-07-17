# winc - Windows Capture

A crate for windows screen capture.

## TODO

- `image` feature flag
- `id` (`fxhash`) feature flag
- `metrics` (`indexmap`) feature flag

## Attributions

Some code from https://github.com/TeamDman/Cursor-Hero which used some code from https://github.com/nashaofu/screenshots-rs/ commit 999faac06f85bd93638c2a9cda6cbb25ad9f5c73

My understanding is that the licenses used in our projects are compatible. If something is wrong, please let me know!

The modifications aim to reduce redundant work for successive screen capture calls.

Might also be interesting:

- https://github.com/rhinostream/win_desktop_duplication/tree/master
- https://github.com/rustdesk/rustdesk
- https://github.com/RustBuddies/desktop-sharing
- https://github.com/mira-screen-share/sharer/blob/main/src/capture/wgc/display.rs
