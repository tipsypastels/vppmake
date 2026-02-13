use super::Map;
use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Room {
    pub key: KString,
    pub name: KString,
    pub pokemon: Map<RoomMember>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoomMember {
    pub key: KString,
    #[serde(default)]
    pub flipped: bool,
    pub position: RoomMemberPosition,
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
