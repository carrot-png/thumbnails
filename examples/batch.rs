use std::{
    env::args,
    fs::{read_dir, DirEntry},
    io,
    path::Path,
};

use thumbnailer::Thumbnailer;

fn main() {
    let thumbnailer = Thumbnailer::new(250, 250);

    let input = args().nth_back(1).unwrap();
    let output = args().nth_back(0).unwrap();

    let input = Path::new(&input);
    let output = Path::new(&output);

    let thumb = |entry: &DirEntry| {
        match thumbnailer.get(&entry.path()) {
            Ok(img) => {
                let mut out = output.join(entry.file_name());
                out.set_extension("png");
                img.save(out).unwrap();
            }
            Err(err) => println!("{:?} ({:?})", err, entry.file_name()),
        };
    };

    visit_dirs(input, &thumb).unwrap();
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
