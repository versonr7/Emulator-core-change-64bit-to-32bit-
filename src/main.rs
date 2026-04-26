use std::io::{self, Write};

fn main() {
    let mut power = 100;
    let mut door_l = false;
    let mut door_r = false;

    loop {
        // تنظيف الشاشة ليعطي شعور الجرافيكس المتحرك
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); 
        
        println!("\x1b[93m--- 🐻 FNAF TERMUX EMULATOR: PRO CONSOLE --- \x1b[0m");
        println!("🔋 Power: \x1b[92m{}%\x1b[0m | 🕒 Night: 1", power);
        println!("-------------------------------------------");
        println!("         [ OFFICE MONITOR ]      ");
        
        let l_status = if door_l { "\x1b[91mCLOSED\x1b[0m" } else { "OPEN" };
        let r_status = if door_r { "\x1b[91mCLOSED\x1b[0m" } else { "OPEN" };

        println!("  LEFT DOOR: [{}]    RIGHT DOOR: [{}]", l_status, r_status);
        println!("-------------------------------------------");
        println!("  \x1b[94m(A)\x1b[0m Toggle Left Door");
        println!("  \x1b[94m(D)\x1b[0m Toggle Right Door");
        println!("  \x1b[94m(W)\x1b[0m Open Cameras");
        println!("  \x1b[91m(X)\x1b[0m Shutdown Emulator");
        println!("-------------------------------------------");
        
        print!("🕹️ Press Key > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim().to_lowercase();

        match cmd.as_str() {
            "a" => { door_l = !door_l; power -= 2; },
            "d" => { door_r = !door_r; power -= 2; },
            "w" => println!("\n🎥 Accessing CAM 1A... Freddy is still there."),
            "x" => break,
            _ => (),
        }

        power -= 1;
        if power <= 0 { println!("💀 POWER LOST. IT'S OVER."); break; }
    }
}
