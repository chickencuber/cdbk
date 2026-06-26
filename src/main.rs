mod cmd;
mod locations;
mod manifest;
use cmdparsing::cmd;


//TASK(20260625-151415-931-n6-971): add the ui

//TASK(20260625-180218-804-n6-581): add logic related to allowing projects to update
cmd! {
    help: cmd::HELP;
    cmd::package => "package" | "pkg",
    cmd::extract => "extract",
    cmd::install => "install" | "i",
    cmd::setup => "setup",
    cmd::list => "list",
    cmd::remove => "remove"|"r",
    cmd::uninstall => "uninstall",
}



