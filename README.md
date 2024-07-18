# winc - Windows Capture

Pronounced "wink"

[![Crates.io](https://img.shields.io/crates/v/winc)](https://crates.io/crates/winc)
[![Docs.rs](https://docs.rs/winc/badge.svg)](https://docs.rs/winc)
![License](https://img.shields.io/crates/l/winc)

A crate for windows screen capture.

## Usage

See [tests.rs](./src/tests.rs) for examples.

```rust
#[test]
fn region_screenshots() {
    let monitors = get_all_monitors().unwrap();
    let mut capturers = Vec::new();

    for monitor in monitors {
        let p0 = monitor.info.rect.top_left();
        let p1 = p0.translate(100, 100);
        let region = RECT::from_corners(p0, p1).translate(100, 100);
        let capturer = get_monitor_capturer(Rc::new(monitor), region);
        capturers.push(capturer);
    }
    std::fs::create_dir_all("target/capture").unwrap();

    let mut images = Vec::new();
    for capturer in capturers.iter() {
        // capture
        let capture = capturer.capture(&mut Metrics::None).unwrap();

        // save image
        let mon_name_good = capturer.monitor.info.name.replace(r"\\.\", "");
        let path = format!("target/capture/region-{}.png", mon_name_good);
        capture.save(path).unwrap();
        images.push(capture);
    }

    assert_no_transparency(&images);
}
```

## Attributions

Some code from https://github.com/TeamDman/Cursor-Hero which used some code from https://github.com/nashaofu/screenshots-rs/ commit 999faac06f85bd93638c2a9cda6cbb25ad9f5c73

My understanding is that the licenses used in our projects are compatible. If something is wrong, please let me know!

The modifications aim to reduce redundant work for successive screen capture calls.

Might also be interesting:

- https://github.com/rhinostream/win_desktop_duplication/tree/master
- https://github.com/rustdesk/rustdesk
- https://github.com/RustBuddies/desktop-sharing
- https://github.com/mira-screen-share/sharer/blob/main/src/capture/wgc/display.rs
