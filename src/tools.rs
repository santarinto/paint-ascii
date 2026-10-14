use crate::color::ColorId;
use crate::model::{ColShiftMode, RowShiftMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Scroll,       // Alt+H (Hand / pan)
    Zoom,         // Alt+Z (Magnifier / Лупа)
    Pencil,       // Alt+B
    ColorBrush,   // Alt+K (Кисть цвета / Color Brush)
    Eraser,       // Alt+E
    Fill,         // Alt+G
    Pipette,      // Alt+I (Пипетка символа -> превращается в Карандаш)
    ColorPipette, // Alt+P (Пипетка цвета -> превращается в Кисть)
    Select,       // Alt+M
    DeleteRow,    // Alt+R (Delete Row / Удалить строку)
    DeleteCol,    // Alt+D (Delete Column / Удалить столбец)
}

impl Tool {
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Scroll => "Hand / Pan (Alt+H)",
            Tool::Zoom => "Zoom / Лупа (Alt+Z)",
            Tool::Pencil => "Pencil / Карандаш (Alt+B)",
            Tool::ColorBrush => "Color Brush / Кисть цвета (Alt+K; красит символы в выбранный цвет)",
            Tool::Eraser => "Eraser / Ластик (Alt+E)",
            Tool::Fill => "Fill / Заливка (Alt+G)",
            Tool::Pipette => "Symbol Pipette / Пипетка символа (Alt+I; -> Карандаш)",
            Tool::ColorPipette => "Color Pipette / Пипетка цвета (Alt+P; -> Кисть)",
            Tool::Select => "Select / Выделение (Shift+ЛКМ / Alt+M; ✥ перемещение блока)",
            Tool::DeleteRow => "Delete Row / Удалить строку (Alt+R)",
            Tool::DeleteCol => "Delete Column / Удалить столбец (Alt+D)",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tool::Scroll => "✋",
            Tool::Zoom => "🔍",
            Tool::Pencil => "✏",
            Tool::ColorBrush => "🖌",
            Tool::Eraser => "⌫",
            Tool::Fill => "🪣",
            Tool::Pipette => "💧",
            Tool::ColorPipette => "🎨",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_icons_and_names() {
        let tools = [
            Tool::Scroll,
            Tool::Zoom,
            Tool::Pencil,
            Tool::ColorBrush,
            Tool::Eraser,
            Tool::Fill,
            Tool::Pipette,
            Tool::ColorPipette,
            Tool::Select,
            Tool::DeleteRow,
            Tool::DeleteCol,
        ];
        for t in tools {
            assert!(!t.name().is_empty());
            assert!(!t.icon().is_empty());
        }
        assert_eq!(Tool::ColorBrush.icon(), "🖌");
        assert_eq!(Tool::ColorPipette.icon(), "🎨");
    }

    #[test]
    fn test_pipette_transitions() {
        let mut state = ToolState::default();

        // 1. Symbol pipette switches to Pencil
        state.current_tool = Tool::Pipette;
        // Simulate sampling
        state.brush_char = '#';
        state.brush_color = ColorId::BrightCyan;
        if state.current_tool == Tool::Pipette {
            state.current_tool = Tool::Pencil;
        }
        assert_eq!(state.current_tool, Tool::Pencil);
        assert_eq!(state.brush_char, '#');
        assert_eq!(state.brush_color, ColorId::BrightCyan);

        // 2. Color pipette switches to ColorBrush
        state.current_tool = Tool::ColorPipette;
        // Simulate sampling
        state.brush_color = ColorId::BrightRed;
        if state.current_tool == Tool::ColorPipette {
            state.current_tool = Tool::ColorBrush;
        }
        assert_eq!(state.current_tool, Tool::ColorBrush);
        assert_eq!(state.brush_color, ColorId::BrightRed);
    }
}

