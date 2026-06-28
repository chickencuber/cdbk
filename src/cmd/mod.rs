mod package;
pub use package::*;
mod extract;
pub use extract::*;
mod install;
pub use install::*;
mod setup;
pub use setup::*;
mod list;
pub use list::*;
mod remove;
pub use remove::*;
mod uninstall;
pub use uninstall::*;
mod open;
pub use open::*;


pub const HELP: &str = r#"usage: cdbk [subcommand]
=====package=====
packages a directory into a cdbk bundle

cdbk package [path to directory]

--- aliases ---
pkg
pack
=====extract=====
extracts the contents of a cdbk bundle

cdbk extract [path to bundle]

=====install=====
installs a cdbk bundle to the system

cdbk install [path to bundle]

--- aliases ---
i
=====setup=====
sets up the desktop file and mime type

cdbk setup

=====uninstall=====
undoes what setup does

cdbk uninstall

=====list=====
lists installed bundles 

cdbk list 
=====remove=====
removes an installed bundle 

cdbk remove [bundle name]

--- aliases ---
r
=====open=====
opens a gui to handle the bundle

cdbk open [path to bundle]
"#;
