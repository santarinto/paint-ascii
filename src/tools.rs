use crate::color::ColorId;
use crate::model::{ColShiftMode, RowShiftMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Scroll,    // Alt+H (Hand / pan)
    Zoom,      // Alt+Z (Magnifier / Лупа)
    Pencil,    // Alt+B
    Eraser,    // Alt+E
    Fill,      // Alt+G
    Pipette,   // Alt+I
    Select,    // Alt+M
    DeleteRow, // Alt+R (Delete Row / Удалить строку)
    DeleteCol, // Alt+D (Delete Column / Удалить столбец)
}

impl Tool {
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Scroll => "Hand / Pan (Alt+H)",
            Tool::Zoom => "Zoom / Лупа (Alt+Z)",
            Tool::Pencil => "Pencil (Alt+B)",
            Tool::Eraser => "Eraser (Alt+E)",
            Tool::Fill => "Fill (Alt+G)",
            Tool::Pipette => "Pipette (Alt+I)",
            Tool::Select => "Select (Alt+M)",
            Tool::DeleteRow => "Delete Row / Удалить строку (Alt+R)",
            Tool::DeleteCol => "Delete Column / Удалить столбец (Alt+D)",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tool::Scroll => "✋",
            Tool::Zoom => "🔍",
            Tool::Pencil => "✏",
            Tool::Eraser => "⌫",
            Tool::Fill => "🪣",
            Tool::Pipette => "💧",
            Tool::Select => "⛶",
            Tool::DeleteRow => "⊟",
            Tool::DeleteCol => "◫",
        }
    }
}

pub const DEFAULT_PALETTE: &str = 
    "██▓▒░☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■*#+=-:. ";

#[derive(Debug, Clone)]
pub struct ToolState {
    pub current_tool: Tool,
    pub brush_char: char,
    pub brush_color: ColorId,
    pub palette: Vec<char>,
    pub row_shift_mode: RowShiftMode,
    pub col_shift_mode: ColShiftMode,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            current_tool: Tool::Pencil,
            brush_char: '*',
            brush_color: ColorId::Default,
            palette: DEFAULT_PALETTE.chars().collect(),
            row_shift_mode: RowShiftMode::default(),
            col_shift_mode: ColShiftMode::default(),
        }
    }
}
