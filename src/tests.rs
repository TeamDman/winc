
#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{monitor_region_capturer::{get_full_monitor_capturers, get_monitor_capturer}, prelude::{get_all_monitors, get_monitor_infos}};

    #[test]
    fn names() {
        get_monitor_infos().unwrap().iter().for_each(|info| {
            println!("{:?}", info);
        });
    }

    #[test]
    fn full_screenshots() {
        let capturers = get_full_monitor_capturers().unwrap();
        std::fs::create_dir_all("target/capture").unwrap();

        capturers.iter().for_each(|capturer| {
            let capture = capturer.capture(&mut None).unwrap();
            let mon_name_good = capturer.monitor.info.name.replace(r"\\.\", "");
            let path = format!("target/capture/full-{}.png", mon_name_good);
            capture.save(path).unwrap();
        });
    }

    #[test]
    fn region_screenshots() {
        let monitors = get_all_monitors().unwrap();
        let mut capturers = Vec::new();

        for monitor in monitors {
            let p0 = monitor.info.rect.top_left();
            let p1 = p0 + IVec2::new(100, 100);
            let region = IRect::from_corners(p0, p1);
            let capturer = get_monitor_capturer(Arc::new(monitor), region);
            capturers.push(capturer);
        }
        std::fs::create_dir_all("target/capture").unwrap();

        capturers.iter().for_each(|capturer| {
            let capture = capturer.capture(&mut None).unwrap();
            let mon_name_good = capturer.monitor.info.name.replace(r"\\.\", "");
            let path = format!("target/capture/region-{}.png", mon_name_good);
            capture.save(path).unwrap();
        });
    }

    #[test]
    fn capture_avg() {
        let capturers = get_full_monitor_capturers().unwrap();
        std::fs::create_dir_all("target/capture").unwrap();

        for _ in 0..100 {
            capturers.iter().for_each(|capturer| {
                let capture = capturer.capture(&mut None).unwrap();
                let (mut tot_r, mut tot_g, mut tot_b) = (0, 0, 0);

                for pixel in capture.enumerate_pixels() {
                    let image::Rgba([r, g, b, _]) = pixel.2; // Destructure the Rgba struct
                    tot_r += *r as u64;
                    tot_g += *g as u64;
                    tot_b += *b as u64;
                }
                let size = capture.iter().count() as u64;
                print!(
                    "{} -- avg: {:?}\t",
                    capturer.monitor.info.name,
                    (tot_r / size, tot_g / size, tot_b / size)
                );
            });
            print!("\n");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    #[test]
    fn screenshot_speed() {
        let capturers = get_full_monitor_capturers().unwrap();
        let mut durations = Vec::new();
        for _ in 0..100 {
            capturers.iter().for_each(|capturer| {
                let start = std::time::Instant::now();
                let _ = capturer.capture(&mut None).unwrap();
                let duration = start.elapsed();
                durations.push(duration.as_millis());
            });
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let avg = durations.iter().sum::<u128>() / durations.len() as u128;
        println!("avg: {:?}ms", avg);
    }
}
