use sdl2::rect::Rect;

pub struct CopyTool {
    pub src: Rect,
    pub dst: Rect,
}

impl CopyTool {
    pub fn new(src: Rect, dst: Rect) -> Self {
        Self {src, dst}
    }
}

pub struct TextureMaker;

pub enum BrickSprite {
    White, LightRed, Orange,
    LightBlue, LightCyan,
    Magenta, LightGreen, Yellow,
    Blue, Red, Green, Cyan,
    Silver1, Silver2, Silver3, Silver4, Silver5, Silver6,
    Gold1, Gold2, Gold3, Gold4, Gold5, Gold6,
}

impl TextureMaker {
    fn brick(&self, sprite: BrickSprite, dst: Rect) -> CopyTool {
        let (w, h): (u32, u32) = (32, 16);
        match sprite {
            BrickSprite::White => CopyTool::new(Rect::new(0, 0, w, h), dst),
            BrickSprite::LightRed => CopyTool::new(Rect::new(0, 15, w, h), dst),
            BrickSprite::Silver1 => CopyTool::new(Rect::new( 0, 31, w, h), dst),
            BrickSprite::Gold1 => CopyTool::new(Rect::new(0, 47, w, h), dst),
            BrickSprite::Orange => CopyTool::new(Rect::new(31, 0, w, h), dst),
            BrickSprite::LightBlue => CopyTool::new(Rect::new(31, 15, w, h), dst),
            BrickSprite::Silver2 => CopyTool::new(Rect::new(31, 31, w, h), dst),
            BrickSprite::Gold2 => CopyTool::new(Rect::new(31, 47, w, h), dst),
            BrickSprite::LightCyan => CopyTool::new(Rect::new(63, 0, w, h), dst),
            BrickSprite::Magenta => CopyTool::new(Rect::new(63, 15, w, h), dst),
            BrickSprite::Silver3 => CopyTool::new(Rect::new(63, 31, w, h), dst),
            BrickSprite::Gold3 => CopyTool::new(Rect::new(63, 47, w, h), dst),
            BrickSprite::LightGreen => CopyTool::new(Rect::new(95, 0, w, h), dst),
            BrickSprite::Yellow => CopyTool::new(Rect::new(95, 15, w, h), dst),
            BrickSprite::Silver4 => CopyTool::new(Rect::new(95, 31, w, h), dst),
            BrickSprite::Gold4 => CopyTool::new(Rect::new(95, 47, w, h), dst),
            BrickSprite::Blue => CopyTool::new(Rect::new(127, 0, w, h), dst),
            BrickSprite::Red => CopyTool::new(Rect::new(127, 15, w, h), dst),
            BrickSprite::Silver5 => CopyTool::new(Rect::new(127, 31, w, h), dst),
            BrickSprite::Gold5 => CopyTool::new(Rect::new(127, 47, w, h), dst),
            BrickSprite::Green => CopyTool::new(Rect::new(159, 0, w, h), dst),
            BrickSprite::Cyan => CopyTool::new(Rect::new(159, 15, w, h), dst),
            BrickSprite::Silver6 => CopyTool::new(Rect::new(159, 31, w, h), dst),
            BrickSprite::Gold6 => CopyTool::new(Rect::new(159, 470, w, h), dst),
        }
    }
    // Brick sprites 32 * 16 px
    // White : 0, 0
    // LightRed : 0, 15
    // Silver1: 0, 31
    // Gold1 : 0, 47
    // Orange : 31, 0
    // LightBlue : 31, 15
    // Silver2 : 31, 31
    // Gold2 : 31, 47
    // LightCyan : 63, 0
    // Magenta : 63, 15
    // Silver3 : 63, 31
    // Gold3 : 63, 47
    // LightGreen : 95, 0
    // Yellow : 95, 15
    // Silver4 : 95, 31
    // Gold4 : 95, 47
    // Blue : 127, 0
    // Red : 127, 15
    // Silver5 : 127, 31
    // Gold5 : 127, 47
    // Green : 159, 0
    // Cyan : 159, 15
    // Silver6 : 159, 31
    // Gold6 : 159, 470 
    // Ball sprites : 16 * 16 px
    // Ball1 : 0, 63
    // Ball2 : 15, 63
    // Ball3 : 31, 63
    // Ball4 : 47, 63
    // Ball5 : 63, 63
    // Ball6 : 79, 63
    //fn ball(&self) {} 
    // Rockets : 16 * 40 px
    // Yellow : 0, 79
    // Blue : 15, 79
    // Backgrounds
    // Props
    // Vessels
    // Powerups
}
