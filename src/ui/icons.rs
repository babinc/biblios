/// Nerd Font icons for elegant UI
///
/// These icons require a Nerd Font to be installed in your terminal.
/// Recommended fonts: JetBrainsMono Nerd Font, FiraCode Nerd Font, Hack Nerd Font
///
/// Download from: https://www.nerdfonts.com/

// Book and Reading Icons
pub const BOOK: &str = "󰂽";  // nf-md-book_open_variant
pub const BOOK_MULTIPLE: &str = "󱓷";  // nf-md-book_multiple
pub const BIBLE: &str = "󰮭";  // nf-md-book_cross
pub const BOOKMARK: &str = "󰃃";  // nf-md-bookmark
pub const BOOKMARK_OUTLINE: &str = "󰃀";  // nf-md-bookmark_outline

// Navigation Icons
pub const CHEVRON_RIGHT: &str = "";  // nf-fa-chevron_right
pub const ARROW_RIGHT: &str = "";  // nf-fa-arrow_right
pub const CIRCLE: &str = "●";  // nf-fa-circle
pub const CIRCLE_OUTLINE: &str = "○";  // nf-fa-circle_o

// Status Icons
pub const CHECK: &str = "";  // nf-fa-check
pub const STAR: &str = "";  // nf-fa-star
pub const HEART: &str = "";  // nf-fa-heart
pub const CLOCK: &str = "";  // nf-fa-clock_o

// Mode Icons
pub const VIM: &str = "";  // nf-dev-vim
pub const KEYBOARD: &str = "󰌌";  // nf-md-keyboard
pub const EYE: &str = "󰈈";  // nf-md-eye
pub const EYE_OFF: &str = "󰈉";  // nf-md-eye_off

// UI Icons
pub const SEARCH: &str = "";  // nf-fa-search
pub const SETTINGS: &str = "";  // nf-fa-cog
pub const HELP: &str = "";  // nf-fa-question_circle
pub const HOME: &str = "";  // nf-fa-home

// Testament Icons
pub const OLD_TESTAMENT: &str = "󰴪";  // nf-md-alpha_o
pub const NEW_TESTAMENT: &str = "󰰾";  // nf-md-alpha_n

// Progress Icons
pub const PROGRESS_0: &str = "";  // nf-fa-circle_o
pub const PROGRESS_50: &str = "󰪞";  // nf-md-circle_half
pub const PROGRESS_100: &str = "";  // nf-fa-circle

/// Get icon for a book based on its position in the Bible
pub fn book_icon(book_name: &str) -> &'static str {
    // Old Testament books
    let old_testament = [
        "Gen", "Exod", "Lev", "Num", "Deut",
        "Josh", "Judg", "Ruth", "1Sam", "2Sam",
        "1Kgs", "2Kgs", "1Chr", "2Chr", "Ezra",
        "Neh", "Esth", "Job", "Ps", "Prov",
        "Eccl", "Song", "Isa", "Jer", "Lam",
        "Ezek", "Dan", "Hos", "Joel", "Amos",
        "Obad", "Jonah", "Mic", "Nah", "Hab",
        "Zeph", "Hag", "Zech", "Mal",
    ];

    if old_testament.contains(&book_name) {
        OLD_TESTAMENT
    } else {
        NEW_TESTAMENT
    }
}

/// Get book category icon
pub fn category_icon(book_name: &str) -> &'static str {
    match book_name {
        // Law (Torah)
        "Gen" | "Exod" | "Lev" | "Num" | "Deut" => "󰪫",  // nf-md-script

        // History
        "Josh" | "Judg" | "Ruth" | "1Sam" | "2Sam" | "1Kgs" | "2Kgs" |
        "1Chr" | "2Chr" | "Ezra" | "Neh" | "Esth" => "󰄉",  // nf-md-clock_outline

        // Wisdom
        "Job" | "Ps" | "Prov" | "Eccl" | "Song" => "",  // nf-fa-lightbulb_o

        // Prophecy
        "Isa" | "Jer" | "Lam" | "Ezek" | "Dan" | "Hos" | "Joel" |
        "Amos" | "Obad" | "Jonah" | "Mic" | "Nah" | "Hab" | "Zeph" |
        "Hag" | "Zech" | "Mal" => "󰓎",  // nf-md-message_alert

        // Gospels
        "Matt" | "Mark" | "Luke" | "John" => "",  // nf-fa-heart

        // Acts
        "Acts" => "󰙨",  // nf-md-run

        // Epistles
        "Rom" | "1Cor" | "2Cor" | "Gal" | "Eph" | "Phil" | "Col" |
        "1Thess" | "2Thess" | "1Tim" | "2Tim" | "Titus" | "Phlm" |
        "Heb" | "Jas" | "1Pet" | "2Pet" | "1John" | "2John" | "3John" | "Jude" => "",  // nf-fa-envelope

        // Revelation
        "Rev" => "",  // nf-fa-eye

        _ => BOOK,
    }
}
