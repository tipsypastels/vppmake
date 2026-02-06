use super::map::Map;
use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Room {
    key: KString,
    name: KString,
    pokemon: Map<RoomMember>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoomMember {
    key: KString,
    #[serde(default)]
    flipped: bool,
    position: RoomMemberPosition,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum RoomMemberPosition {
    TopLeft { top: u16, left: u16 },
    TopRight { top: u16, right: u16 },
    BottomLeft { bottom: u16, left: u16 },
    BottomRight { bottom: u16, right: u16 },
}

impl RoomMemberPosition {
    pub fn unpack(self) -> [(&'static str, u16); 2] {
        macro_rules! unpack {
            ($($pat:ident($a:ident, $b:ident)),*$(,)?) => {
                match self {
                    $(Self::$pat { $a, $b } => [(stringify!($a), $a), (stringify!($b), $b)],)*
                }
            };
        }
        unpack! {
            TopLeft(top, left),
            TopRight(top, right),
            BottomLeft(bottom, left),
            BottomRight(bottom, right),
        }
    }
}
