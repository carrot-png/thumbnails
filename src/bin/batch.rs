use std::{env::args, path::Path};

use thumbnailer::thumbnail::Thumbnailer;

fn main() {
    let thumbnailer = Thumbnailer::new(250, 250);

    let input = args().nth_back(1).unwrap();
    let output = args().nth_back(0).unwrap();

    let input = Path::new(&input);
    let output = Path::new(&output);

    for entry in input.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let thumb = thumbnailer.get(&entry.path());

            match thumb {
                Ok(img) => {
                    let mut out = output.join(entry.file_name());
                    out.set_extension("png");
                    img.save(out).unwrap();
                }
                Err(err) => println!("{:?} ({:?})", err, entry.file_name()),
            }
        }
    }
}
