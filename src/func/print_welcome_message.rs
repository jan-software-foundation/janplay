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
            print!("\x1B[38;2;{};{};{}m", colors[j][0], colors[j][1], colors[j][2]);
            for k in 0..3 {
                colors[j][k] = (colors[j][k] as f32 * 0.9).round() as u32;
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