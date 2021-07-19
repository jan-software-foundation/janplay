use term::color;

pub fn print() {
    let mut term = term::stdout().unwrap();
    let mut colors: [[u32; 3]; 7] = [
        [216, 160, 223],
        [171, 156, 223],
        [154, 167, 223],
        [143, 223, 211],
        [151, 223, 143],
        [223, 204, 157],
        [223, 168, 168],
    ];
    
    // If terminal doesn't support 24 bit color, don't
    let mut color_enabled = match std::env::var("COLORTERM").unwrap_or(String::from("")).as_str() {
        "truecolor"|"24bit" => { true }
        _ => { false }
    };
    if std::env::var("NO_COLOR").is_ok() { color_enabled = false; }
    
    // alright i hate myself for this
    let letters: [[&str; 5]; 7] = [
        [ // J
            "     _",
            "    | |",
            " _  | |",
            "| |_|",
            " \\___"
        ], [ // A
            "   _",
            " / \\",
            "/ _ \\",
            " / ___ \\",
            "/_/   \\_\\"
        ], [ // N
            "    _   _",
            "  | \\ |",
            " |  \\|",
            "| |\\",
            "_| \\_"
        ], [ // P
            " ____",
            " |  _ \\",
            " | |_)",
            "  |  __/",
            "|_|"
        ], [ // L
            "  _",
            "| |",
            " | |",
            "| |___",
            "   |_____"
        ], [ // A
            "        _",
            "      / \\",
            "     / _ \\",
            " / ___ \\",
            "/_/   \\_\\"
        ], [ // Y
            " __   __",
            "\\ \\ / /",
            "\\ V / ",
            "| |  ",
            "_|  "
        ]
    ];
    
    for i in 0..5 {
        for j in 0..7 {
            if color_enabled {
                print!("\x1B[38;2;{};{};{}m", colors[j][0], colors[j][1], colors[j][2]);
                for k in 0..3 {
                    colors[j][k] = (colors[j][k] as f32 * 0.9).round() as u32;
                }
            }
            
            print!("{}", letters[j][i]);
        }
        print!("\n");
    }
    
    term.fg(color::WHITE).unwrap();
    print!("\n");
    print!("Use !quit to quit or !help to list available commands.");
    print!("\n");
}