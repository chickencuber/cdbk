# CBDK
`A Way to bundle and install packages for linux`
## Why
I was annoyed that I had to manually create desktop files to get appimages in my app menu, so I made this so that people who make apps can give a bundle that does this for you. While it was made for appimages, there is nothing stopping you from using other types of executable files too.
## Requirements
- Only works on linux(possibly bsd too, but untested)
- cargo
- xdg-utils
- desktop-file-utils
- shared-mime-info
## Installation
### Manual
```bash
git clone https://github.com/chickencuber/cdbk.git # clone the repo
cd cdbk
cargo install --path . # install the repo
cdbk setup # setup system integration
```

### Install with Cargo
```bash
cargo install cdbk
cdbk setup # setup system integration
```
## How to use
### Create a bundle
create a directory with required files inside(you can check [this](./testbundle/) for more info)
and run `cdbk package [directory name]` and you'll have a new bundle created
### Install a bundle
there are 2 ways to install a bundle, either run `cdbk install bundle.cdbk`, or in your favorite file manager, open the file with cdbk package installer
### Extract a bundle
you can extract a bundle with `cdbk extract bundle.cdbk`, which will let you get to the files the bundle is made with
### Uninstalling an installed bundle
run `cdbk remove "bundle name"` and it will install the bundle
### Uninstalling cdbk
if you want to uninstall cdbk, make sure to run `cdbk uninstall` first to remove all stuff it changed, then you can uninstall it using cargo
## Known issues
- if using a glib based file manager, you may have to also run `gio mime application/x-cdbk cdbk.desktop` if it doesn't show in the open with menu
## Planned
- a way for bundles to auto update
[TODO](./TODO.md)
