use std::{fs, path::Path};

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn main() {
    println!("System-One OmniSilicon Hardware Discovery");

    let pci = Path::new("/sys/bus/pci/devices");

    let Ok(devices) = fs::read_dir(pci) else {
        println!("PCI sysfs unavailable; discovery deferred.");
        return;
    };

    for entry in devices.flatten() {
        let path = entry.path();
        let vendor = read(&path.join("vendor"));

        if vendor != "0x1002" {
            continue;
        }

        let device = read(&path.join("device"));
        let class = read(&path.join("class"));

        println!(
            "AMD device: {} device={} class={}",
            entry.file_name().to_string_lossy(),
            device,
            class
        );
    }

    let accel = Path::new("/dev/accel");

    if let Ok(entries) = fs::read_dir(accel) {
        for entry in entries.flatten() {
            println!(
                "Accelerator node: {}",
                entry.path().display()
            );
        }
    }

    println!("Hardware discovery complete.");
}
