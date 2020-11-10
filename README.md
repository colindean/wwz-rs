# wwz-rs

An attempt at cloning [wwz](https://github.com/oilshell/wwz) in Rust.

Use cases:

* Serve a website built with a static site generator, but compressed to save
    storage space on space-limited shared hosting or reduce container size
* Serve a compressed logfile directory decompressed

Goals:

* Serve static files out of a ZIP file
* Cache up to some size limit of uncompressed files in memory or in memcached

