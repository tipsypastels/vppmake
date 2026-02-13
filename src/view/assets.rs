use std::sync::OnceLock;

// TODO: Replace with main once merged.
const BASE: &str =
    "https://raw.githubusercontent.com/tipsypastels/vppmake/refs/heads/rewrite/assets/";

pub fn egg_asset() -> &'static str {
    static EGG_ASSET: OnceLock<Box<str>> = OnceLock::new();
    EGG_ASSET.get_or_init(|| asset("pokemon/egg.png"))
}

pub fn pokemon_asset(key: &str, species_key: &str) -> Box<str> {
    asset(["pokemon/", key, "-", species_key, ".png"])
}

pub fn room_asset(room_key: &str) -> Box<str> {
    asset(["rooms/", room_key, ".png"])
}

pub fn type_icon_asset(type_key: &str) -> Box<str> {
    asset(["types/icons/", type_key, ".svg"])
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
