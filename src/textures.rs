use sdl2::rect::Rect;

pub struct CopyTool {
    pub src: Rect,
    pub dst: Rect,
}

impl CopyTool {
    pub fn new(src: Rect, dst: Rect) -> Self {
        Self { src, dst }
    }
}

pub struct TextureMaker;

#[allow(dead_code)]
pub enum BrickSprite {
    White,
    LightRed,
    Orange,
    LightBlue,
    LightCyan,
    Magenta,
    LightGreen,
    Yellow,
    Blue,
    Red,
    Green,
    Cyan,
    Silver1,
    Silver2,
    Silver3,
    Silver4,
    Silver5,
    Silver6,
    Gold1,
    Gold2,
    Gold3,
    Gold4,
    Gold5,
    Gold6,
}

#[allow(dead_code)]
pub enum BallSprite {
    Ball1,
    Ball2,
    Ball3,
    Ball4,
    Ball5,
    Ball6,
}

#[allow(dead_code)]
pub enum VesselSprite {
    Size1,
    Size2,
    Size3,
    Size4,
    Size5,
    Size6,
    Size7,
    Size8,
}

#[allow(dead_code)]
pub enum PowerUpSprite {
    Rot1,
    Rot2,
    Rot3,
    Rot4,
    Rot5,
    Rot6,
    Rot7,
    Rot8,
}

#[allow(dead_code)]
pub enum PowerUpType {
    S,
    C,
    L,
    E,
    D,
    B,
    P,
}

#[allow(dead_code)]
pub enum BackgroundSprite {
    Polygon,
    PolygonDark,
    Blue,
    BlueDark,
    Green,
    GreenDark,
    Red,
    RedDark,
    Orange,
    OrangeDark,
    Purple,
    PurpleDark,
}

#[allow(dead_code)]
pub enum RocketSprite {
    Yellow,
    Blue,
}

#[allow(dead_code)]
pub enum PropsSprite {
    Cone1,
    Pyramid1,
    Molecule1,
    Molecule17,
    Explosion1,
    Cone2,
    Pyramid2,
    Molecule2,
    Molecule18,
    Explosion2,
    Cone3,
    Pyramid3,
    Molecule3,
    Molecule19,
    Explosion3,
    Cone4,
    Pyramid4,
    Molecule4,
    Molecule20,
    Explosion4,
    Cone5,
    Pyramid5,
    Molecule5,
    Molecule21,
    Explosion5,
    Cone6,
    Pyramid6,
    Molecule6,
    Molecule22,
    Explosion6,
    Cone7,
    Pyramid7,
    Molecule7,
    Molecule23,
    Cone8,
    Pyramid8,
    Molecule8,
    Molecule24,
    Pyramid9,
    Pyramid10,
    Pyramid11,
    Molecule9,
    Molecule10,
    Molecule11,
    Molecule12,
    Molecule13,
    Molecule14,
    Molecule15,
    Molecule16,
}

impl TextureMaker {
    pub fn brick(sprite: BrickSprite, dst: Rect) -> CopyTool {
        let (w, h): (u32, u32) = (32, 16);
        let xt = vec![0, 31, 63, 95, 127, 159];
        let yt = vec![0, 15, 31, 47];
        let x = match sprite {
            BrickSprite::White
            | BrickSprite::LightRed
            | BrickSprite::Silver1
            | BrickSprite::Gold1 => xt[0],
            BrickSprite::Orange
            | BrickSprite::LightBlue
            | BrickSprite::Silver2
            | BrickSprite::Gold2 => xt[1],
            BrickSprite::LightCyan
            | BrickSprite::Magenta
            | BrickSprite::Silver3
            | BrickSprite::Gold3 => xt[2],
            BrickSprite::LightGreen
            | BrickSprite::Yellow
            | BrickSprite::Silver4
            | BrickSprite::Gold4 => xt[3],
            BrickSprite::Blue | BrickSprite::Red | BrickSprite::Silver5 | BrickSprite::Gold5 => {
                xt[4]
            }
            BrickSprite::Green | BrickSprite::Cyan | BrickSprite::Silver6 | BrickSprite::Gold6 => {
                xt[5]
            }
        };
        let y = match sprite {
            BrickSprite::White
            | BrickSprite::Orange
            | BrickSprite::LightCyan
            | BrickSprite::LightGreen
            | BrickSprite::Blue
            | BrickSprite::Green => yt[0],
            BrickSprite::LightRed
            | BrickSprite::LightBlue
            | BrickSprite::Magenta
            | BrickSprite::Yellow
            | BrickSprite::Red
            | BrickSprite::Cyan => yt[1],
            BrickSprite::Silver1
            | BrickSprite::Silver2
            | BrickSprite::Silver3
            | BrickSprite::Silver4
            | BrickSprite::Silver5
            | BrickSprite::Silver6 => yt[2],
            BrickSprite::Gold1
            | BrickSprite::Gold2
            | BrickSprite::Gold3
            | BrickSprite::Gold4
            | BrickSprite::Gold5
            | BrickSprite::Gold6 => yt[3],
        };
        CopyTool::new(Rect::new(x, y, w, h), dst)
    }
    pub fn ball(sprite: BallSprite, dst: Rect) -> CopyTool {
        let (y, w, h) = (64, 16, 16);
        let x = match sprite {
            BallSprite::Ball1 => 0,
            BallSprite::Ball2 => 15,
            BallSprite::Ball3 => 31,
            BallSprite::Ball4 => 47,
            BallSprite::Ball5 => 63,
            BallSprite::Ball6 => 79,
        };
        CopyTool::new(Rect::new(x, y, w, h), dst)
    }

    #[allow(dead_code)]
    pub fn rocket(sprite: RocketSprite, dst: Rect) -> CopyTool {
        let (y, w, h) = (79, 16, 40);
        let x = match sprite {
            RocketSprite::Yellow => 0,
            RocketSprite::Blue => 15,
        };
        CopyTool::new(Rect::new(x, y, w, h), dst)
    }

    #[allow(dead_code)]
    pub fn background(sprite: BackgroundSprite, dst: Rect) -> CopyTool {
        let (w, h) = (64, 64);
        let xt = vec![0, 64, 128, 192, 256, 320];
        let yt = vec![128, 192];
        let x = match sprite {
            BackgroundSprite::Polygon | BackgroundSprite::PolygonDark => xt[0],
            BackgroundSprite::Blue | BackgroundSprite::BlueDark => xt[1],
            BackgroundSprite::Green | BackgroundSprite::GreenDark => xt[2],
            BackgroundSprite::Red | BackgroundSprite::RedDark => xt[3],
            BackgroundSprite::Orange | BackgroundSprite::OrangeDark => xt[4],
            BackgroundSprite::Purple | BackgroundSprite::PurpleDark => xt[5],
        };
        let y = match sprite {
            BackgroundSprite::Polygon
            | BackgroundSprite::Blue
            | BackgroundSprite::Green
            | BackgroundSprite::Red
            | BackgroundSprite::Orange
            | BackgroundSprite::Purple => yt[0],
            BackgroundSprite::PolygonDark
            | BackgroundSprite::BlueDark
            | BackgroundSprite::GreenDark
            | BackgroundSprite::RedDark
            | BackgroundSprite::OrangeDark
            | BackgroundSprite::PurpleDark => yt[1],
        };
        CopyTool::new(Rect::new(x, y, w, h), dst)
    }

    #[allow(dead_code)]
    pub fn props(sprite: PropsSprite, dst: Rect) -> CopyTool {
        let (w, h): (u32, u32) = (16, 16);
        let xt = vec![256, 288, 320, 352, 384];
        let yt = |i: i32| i * w as i32;
        let x = match sprite {
            PropsSprite::Cone1
            | PropsSprite::Cone2
            | PropsSprite::Cone3
            | PropsSprite::Cone4
            | PropsSprite::Cone5
            | PropsSprite::Cone6
            | PropsSprite::Cone7
            | PropsSprite::Cone8 => xt[0],
            PropsSprite::Pyramid1
            | PropsSprite::Pyramid2
            | PropsSprite::Pyramid3
            | PropsSprite::Pyramid4
            | PropsSprite::Pyramid5
            | PropsSprite::Pyramid6
            | PropsSprite::Pyramid7
            | PropsSprite::Pyramid8
            | PropsSprite::Pyramid9
            | PropsSprite::Pyramid10
            | PropsSprite::Pyramid11 => xt[1],
            PropsSprite::Molecule1
            | PropsSprite::Molecule2
            | PropsSprite::Molecule3
            | PropsSprite::Molecule4
            | PropsSprite::Molecule5
            | PropsSprite::Molecule6
            | PropsSprite::Molecule7
            | PropsSprite::Molecule8
            | PropsSprite::Molecule9
            | PropsSprite::Molecule10
            | PropsSprite::Molecule11
            | PropsSprite::Molecule12
            | PropsSprite::Molecule13
            | PropsSprite::Molecule14
            | PropsSprite::Molecule15
            | PropsSprite::Molecule16 => xt[2],
            PropsSprite::Molecule17
            | PropsSprite::Molecule18
            | PropsSprite::Molecule19
            | PropsSprite::Molecule20
            | PropsSprite::Molecule21
            | PropsSprite::Molecule22
            | PropsSprite::Molecule23
            | PropsSprite::Molecule24 => xt[3],
            PropsSprite::Explosion1
            | PropsSprite::Explosion2
            | PropsSprite::Explosion3
            | PropsSprite::Explosion4
            | PropsSprite::Explosion5
            | PropsSprite::Explosion6 => xt[4],
        };
        let y = match sprite {
            PropsSprite::Cone1
            | PropsSprite::Pyramid1
            | PropsSprite::Molecule1
            | PropsSprite::Molecule17
            | PropsSprite::Explosion1 => yt(0),
            PropsSprite::Cone2
            | PropsSprite::Pyramid2
            | PropsSprite::Molecule2
            | PropsSprite::Molecule18
            | PropsSprite::Explosion2 => yt(1),
            PropsSprite::Cone3
            | PropsSprite::Pyramid3
            | PropsSprite::Molecule3
            | PropsSprite::Molecule19
            | PropsSprite::Explosion3 => yt(2),
            PropsSprite::Cone4
            | PropsSprite::Pyramid4
            | PropsSprite::Molecule4
            | PropsSprite::Molecule20
            | PropsSprite::Explosion4 => yt(3),
            PropsSprite::Cone5
            | PropsSprite::Pyramid5
            | PropsSprite::Molecule5
            | PropsSprite::Molecule21
            | PropsSprite::Explosion5 => yt(4),
            PropsSprite::Cone6
            | PropsSprite::Pyramid6
            | PropsSprite::Molecule6
            | PropsSprite::Molecule22
            | PropsSprite::Explosion6 => yt(5),
            PropsSprite::Cone7
            | PropsSprite::Pyramid7
            | PropsSprite::Molecule7
            | PropsSprite::Molecule23 => yt(6),
            PropsSprite::Cone8
            | PropsSprite::Pyramid8
            | PropsSprite::Molecule8
            | PropsSprite::Molecule24 => yt(7),
            PropsSprite::Pyramid9 | PropsSprite::Molecule9 => yt(8),
            PropsSprite::Pyramid10 | PropsSprite::Molecule10 => yt(9),
            PropsSprite::Pyramid11 | PropsSprite::Molecule11 => yt(10),
            PropsSprite::Molecule12 => yt(11),
            PropsSprite::Molecule13 => yt(12),
            PropsSprite::Molecule14 => yt(13),
            PropsSprite::Molecule15 => yt(14),
            PropsSprite::Molecule16 => yt(15),
        };
        CopyTool::new(Rect::new(x, y, w, h), dst)
    }
    // PropsSprite
    // Cone1 -> 8       0, 256
    // Pyramid1 -> 11   0, 288
    // Molecule1 -> 16  0, 320
    // Molecule17 -> 24 0, 352
    // Explosion1 -> 6  0, 384

    pub fn vessel(sprite: VesselSprite, dst: Rect) -> CopyTool {
        let (x, h) = (384, 16);
        let (y, w) = match sprite {
            VesselSprite::Size1 => (128, 64),
            VesselSprite::Size2 => (144, 74),
            VesselSprite::Size3 => (160, 82),
            VesselSprite::Size4 => (176, 90),
            VesselSprite::Size5 => (192, 98),
            VesselSprite::Size6 => (208, 108),
            VesselSprite::Size7 => (224, 118),
            VesselSprite::Size8 => (240, 129),
        };
        CopyTool::new(Rect::new(x, y, w, h), dst)
    }

    #[allow(dead_code)]
    pub fn powerups(sprite: PowerUpSprite, put: PowerUpType, dst: Rect) -> CopyTool {
        let (w, h) = (16, 16);
        let x = match put {
            PowerUpType::S => 0,
            PowerUpType::C => 15,
            PowerUpType::L => 31,
            PowerUpType::E => 47,
            PowerUpType::D => 63,
            PowerUpType::B => 79,
            PowerUpType::P => 96,
        };
        let y = match sprite {
            PowerUpSprite::Rot1 => 256,
            PowerUpSprite::Rot2 => 288,
            PowerUpSprite::Rot3 => 320,
            PowerUpSprite::Rot4 => 352,
            PowerUpSprite::Rot5 => 384,
            PowerUpSprite::Rot6 => 416,
            PowerUpSprite::Rot7 => 448,
            PowerUpSprite::Rot8 => 480,
        };
        CopyTool::new(Rect::new(x, y, h, w), dst)
    }
}
