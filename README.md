# Rust PPM Viewer

This is a simple (technically) Cross-platform Rust-based PPM Viewer. 

I built this because I couldn't actually find a simple light weight PPM Viewer that accepted text files. I could have also been a tad bit lazy. Anyway, I needed a PPM viewer since that is the format of the 'Ray Tracing in One Weekend' project by Peter Shirley. I ended finding a version to view PPM files but in Java. I don't have java installed and I didn't want to install it. So I made this little tool.

After playing around with it, and reading some of the 'spec' documents (wikipedia), I figured it wouldn't hurt to extend it and create a nice little cross-platform utility for it. I used cross platform libraries, but I'm not sure if it would build. I'll test later versions, but currently there is a Windows build under the releases page.

I found something that looks like the SPEC sheet, so I am also using this as reference. http://paulbourke.net/dataformats/ppm/

# Usage

```bash
ppmviewer.exe "filename.ppm"
```

A window will pop up that matches the size of the input image.

## Change Log

### v1.1 => Road to v2.0

- Ensures Comments are parsed out of the ASCII and Binary versions. i.e they are not processed and stripped before algorithm runs.
- Adds the option to prevent image from being redrawn, as it eats CPU and GPU cycles unnecessarily.
- Adds the ability to read `P6` PPM File format. That is the Binary version of the `P3` format.
- Fixes Resize issues, ensures that window can now resize dynamically


### v1.0.1

Lol I messed up, and I kept certain values when I shouldn't.

- Fixes issue where buffer was being generated using (256x256), effectively making your PPM data fit inside this buffer, so any file larger than 256,256 would be forced into it. It would result in misaligned images with parts of the image missing.
- Added new options inside `Cargo.toml` to reduce the amount of bloat (i.e debug, etc.) from the final executable.
- Removes issues that the Rust-linter found, just make the code not have many errors.

# Roadmap

Ensure support for the other formats in the family, i.e: portable pixmap format (PPM), the portable graymap format (PGM) and the portable bitmap format (PBM). Right now, I am using the ASCII version of the format as that is what I had been rendering to. I would also like to add support for the binary versions.

- Add Suport for PGM, PBM
- Ensure all the formats can be accepted in either Text (ASCII), or binary format.
- Add better CLI instructions and interactions.
- Provide a simple GUI so the application can browser for files.

## Known Issues

- Large files take too long to load
- Opens the file a minimum of two times (once for the header, and then again for the image data)