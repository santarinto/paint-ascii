#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Scroll,  // Alt+H (Hand / pan)
    Pencil,  // Alt+B
    Eraser,  // Alt+E
    Fill,    // Alt+G
    Pipette, // Alt+I
    Select,  // Alt+M
}

impl Tool {
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Scroll => "Hand / Pan (Alt+H)",
            Tool::Pencil => "Pencil (Alt+B)",
            Tool::Eraser => "Eraser (Alt+E)",
            Tool::Fill => "Fill (Alt+G)",
            Tool::Pipette => "Pipette (Alt+I)",
            Tool::Select => "Select (Alt+M)",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tool::Scroll => "✋",
            Tool::Pencil => "✏",
            Tool::Eraser => "⌫",
            Tool::Fill => "🪣",
            Tool::Pipette => "💧",
            Tool::Select => "⛶",
        }
    }
}

pub const DEFAULT_PALETTE: &str = 
    "██▓▒░☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■*#+=-:. ";

#[derive(Debug, Clone)]
pub struct ToolState {
    pub current_tool: Tool,
    pub brush_char: char,
    pub palette: Vec<char>,
    pub custom_palette_text: String,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            current_tool: Tool::Pencil,
            brush_char: '*',
            palette: DEFAULT_PALETTE.chars().collect(),
            custom_palette_text: DEFAULT_PALETTE.to_string(),
        }
    }
}
