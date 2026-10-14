# ASCII Paint (`paint-ascii`)

A fast, lightweight, and modern ASCII/Unicode art editor and TUI mockup design tool written in Rust with [`egui`](https://github.com/emilk/egui).

Designed specifically for editing TUI interfaces, terminal mockups, ANSI art, and classic text graphics with rich colors, character palettes, and keyboard-centric workflows.

---

## Features

- 🎨 **TUI Console Palette Profiles**
  - Switch between popular terminal color schemes from the top bar:
    - **Konsole Breeze** (default dark TUI background `#232629`)
    - **VGA / Linux Console** (pure black `#000000`)
    - **Dracula** (`#282a36`)
    - **Solarized Dark** (`#002b36`)
    - **XTerm Classic** (`#000000`)
  - Dynamically updates the canvas background and all 16 ANSI colors (8 standard + 8 bright).

- ✏ **Drawing & Editing Tools**
  - **Pencil** (`Alt+B`): Draw with active character and color.
  - **Color Brush** (`Alt+K`): Recolor existing characters on the canvas without altering their glyphs.
  - **Eraser** (`Alt+E`): Erase characters back to blank canvas cells.
  - **Flood Fill** (`Alt+G`): Fill contiguous areas of identical characters.
  - **Symbol Pipette** (`Alt+I`): Pick character and color from the canvas (auto-switches to Pencil upon selection).
  - **Color Pipette** (`Alt+P`): Pick color from the canvas (auto-switches to Color Brush).
  - **Selection** (`Alt+M` or `Shift + Left Drag`): Select rectangular regions to edit, type into, clear, or move.
  - **Delete Row** (`Alt+R`): Delete or clear rows with shift modes (*Shift Up*, *Shift Down*, *Clear*).
  - **Delete Column** (`Alt+D`): Delete or clear columns with shift modes (*Shift Left*, *Shift Right*, *Clear*).

- ⌨ **Terminal Cursor & Selection Text Typing**
  - **1×1 Terminal Cursor**: Press any arrow key when no selection exists to create a 1×1 cell at `(0, 0)`. Typing immediately enters characters and advances the cell to the right across the row without automatic newline wrapping.
  - **Selection Text Box**: Typing into any rectangular selection writes text starting from the upper-left corner with auto-wrapping, `Enter` (new line), and `Backspace`.
  - **Blinking Cursor**: Live blinking cursor indicator showing the active typing position.
  - **Block Drag & Drop**: Click and drag any selected area to move it across the canvas with color preservation and full undo/redo support.
  - **Crop Canvas**: Crop the entire canvas down to the active selection.

- 🔍 **Smooth Zooming & Pan**
  - **`Ctrl + Mouse Wheel`**: Smooth, responsive zooming (from `0.25×` to `5.0×`) with scroll locking to prevent canvas drift.
  - **Zoom Tool** (`Alt+Z`): Left-click to zoom in, right-click to zoom out.
  - **Hand / Pan** (`Alt+H` or Middle-click drag): Freely pan around large canvases.

- 💾 **Export & Import**
  - **ANSI Art Export** (`.ans`, `.ansi`): Full ANSI SGR color escape sequences compatible with terminal viewers and BBS software.
  - **Plain Text** (`.txt`, `.asc`, `.ascii`): Clean ASCII/Unicode text files.
  - **PNG Render** (`.png`): High-resolution raster image export with custom dimensions.
  - **Image to ASCII / Braille Converter**: Import raster images (PNG, JPEG, WebP) and convert them directly to ASCII or Braille art with adjustable brightness and contrast.
  - **Robust Clipboard**: Copy and paste regions with full color preservation, Wayland (`wl-copy`) and X11 (`xclip`) system fallbacks, and expandable import.

---

## Keyboard Shortcuts

### Tool Selection

| Shortcut | Tool | Description |
| :--- | :--- | :--- |
| `Alt + B` | **Pencil** | Draw characters with active glyph and color |
| `Alt + K` | **Color Brush** | Paint color onto existing characters without changing glyphs |
| `Alt + E` | **Eraser** | Erase characters to blank cells |
| `Alt + G` | **Flood Fill** | Fill contiguous matching cells |
| `Alt + I` | **Symbol Pipette** | Sample symbol & color from canvas (auto-switches to Pencil) |
| `Alt + P` | **Color Pipette** | Sample color from canvas (auto-switches to Color Brush) |
| `Alt + M` | **Select** | Box selection tool |
| `Alt + H` | **Hand / Pan** | Pan the canvas viewport |
| `Alt + Z` | **Zoom** | Magnifier tool (Left-click zoom in, Right-click zoom out) |
| `Alt + R` | **Delete Row** | Remove or clear hovered row |
| `Alt + D` | **Delete Col** | Remove or clear hovered column |

### Selection & Cursor Navigation

| Shortcut | Action | Description |
| :--- | :--- | :--- |
| `←` `→` `↑` `↓` | **Move Selection / Spawn Cursor** | If no selection: spawns a 1×1 cursor at `(0, 0)`. If selection exists: moves it |
| `Shift + ←` `→` `↑` `↓` | **Resize Selection** | Expands or contracts the active selection box boundaries |
| `Any printable key` | **Type Text** | If selection exists: types text inside frame (advances 1×1 frame rightwards; wraps inside multi-cell box) |
| `Backspace` | **Delete / Step Back** | Inside selection: erases previous character and steps cursor back |
| `Enter` | **New Line** | Moves cursor to the beginning of the next line inside selection |
| `Esc` | **Deselect / Cancel** | Clears active selection frame or cancels block move |
| `Delete` | **Clear Selection** | Clears all characters inside the active selection |

### Canvas Navigation & Mouse Actions

| Shortcut / Gesture | Action | Description |
| :--- | :--- | :--- |
| `Ctrl + Mouse Wheel` | **Smooth Zoom** | Zoom in / out centered on canvas with scroll-locking |
| `Shift + Left Drag` | **Create Selection** | Quickly draw a rectangular selection anywhere on canvas |
| `Left Drag inside Selection` | **Move Block** | Drag and drop selected content to a new position |
| `Middle Click` | **Quick Pipette** | Sample character and color from hovered cell |
| `Middle Drag` | **Pan Canvas** | Scroll canvas in any direction |
| `Right Click Drag` | **Quick Eraser** | Erase cells under pointer |
| `Right Click on Selection` | **Context Menu** | Open context menu (Copy, Cut, Paste, Delete, Crop, Deselect) |

### Clipboard & File Operations

| Shortcut | Action |
| :--- | :--- |
| `Ctrl + C` / `Alt + C` | Copy active selection (or entire canvas if none) to clipboard |
| `Ctrl + X` / `Alt + X` | Cut active selection to clipboard |
| `Ctrl + V` / `Alt + V` | Paste clipboard at selection or hovered cell (clipped to canvas) |
| `Ctrl + Shift + V` | Import clipboard (automatically expands canvas to fit text) |
| `Ctrl + Z` / `Alt + Z` | Undo last operation |
| `Ctrl + Y` / `Ctrl + Shift + Z` | Redo last operation |
| `Ctrl + S` / `Alt + S` | Save plain text file (`.txt`) |
| `Ctrl + O` / `Alt + O` | Open plain text or ANSI file (`.txt`, `.ans`, `.asc`) |

---

## Installation & Building

### Prerequisites

Ensure you have Rust (edition 2024 / Rust 1.85+) installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Linux, ensure common GUI development dependencies are present:

```bash
# Debian / Ubuntu / Mint
sudo apt-get install -y libxkbcommon-dev libwayland-dev libx11-dev libasound2-dev

# Fedora
sudo dnf install -y libxkbcommon-devel wayland-devel libX11-devel alsa-lib-devel

# Arch Linux
sudo pacman -S --needed libxkbcommon wayland libx11 alsa-lib
```

### Build & Run

Clone the repository and run:

```bash
# Run in debug mode
cargo run

# Build optimized release binary
cargo build --release

# Run release binary
./target/release/paint-ascii
```

### Running Tests

Run the full test suite (27 unit and integration tests):

```bash
cargo test
```

---

## License

This project is licensed under the [MIT License](LICENSE).
