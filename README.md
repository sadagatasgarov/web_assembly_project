# New Project
> Based on [MoonZoon](http://moonzoon.rs/)

To use vendored sources, add this to your .cargo/config.toml for this project:

[source.crates-io]
replace-with = "vendored-sources"

[source."git+https://github.com/MoonZoon/MoonZoon?branch=main"]
git = "https://github.com/MoonZoon/MoonZoon"
branch = "main"
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
