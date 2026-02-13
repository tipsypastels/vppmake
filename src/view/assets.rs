const BASE: &str = "https://github.com/tipsypastels/vppmake/blob/main/";

pub fn room_asset(room_key: &str) -> Box<str> {
    asset(["rooms/", room_key, ".png"])
}

fn asset(path: impl AsAssetPath) -> Box<str> {
    Box::from(path.append_to(BASE.to_string()))
}

trait AsAssetPath {
    fn append_to(self, base: String) -> String;
}

impl AsAssetPath for &str {
    fn append_to(self, base: String) -> String {
        base + self
    }
}

impl<const N: usize> AsAssetPath for [&str; N] {
    fn append_to(self, mut base: String) -> String {
        base.extend(self);
        base
    }
}
