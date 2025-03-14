use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TextStyle: u32 {
        const BOLD = 1 << 0;
        const ITALIC = 1 << 1;
        const FIXED = 1 << 2;
        const UNDERLINE = 1 << 3;
        const STRIKETHROUGH = 1 << 4;
    }
}
