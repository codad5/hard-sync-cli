use fli::Fli;

use crate::app::commands::sync;





pub fn init(){  
    let mut app = Fli::init("my app",  "Help sync files between two directories");
    let sync_command = app.command("sync", "Sync files between two directories");

    sync_command.default(sync);
    sync_command.option("-i --init", "Initialize hard-sync in the current directory", sync);
    sync_command.option("-r --reverse", "Sync from target to base", sync);
    sync_command.option("-b --both", "Sync from target to base and base to target", sync);

    app.run();
    
}
