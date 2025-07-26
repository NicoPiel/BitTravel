use bevy::prelude::Color;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Represents a color range with a start and end RGB color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRange {
    pub start: Color,
    pub end: Color,
}

impl From<((u8, u8, u8), (u8, u8, u8))> for ColorRange {
    fn from(colors: ((u8, u8, u8), (u8, u8, u8))) -> Self {
        ColorRange {
            start: Color::srgb_u8(colors.0.0, colors.0.1, colors.0.2),
            end: Color::srgb_u8(colors.1.0, colors.1.1, colors.1.2),
        }
    }
}

// DIAGNOSTIC: Using LazyLock because Color::srgb_u8() is not const
// LazyLock provides thread-safe lazy initialization - perfect for static data needing runtime init
// This solves both issues:
// 1. No OnceLock + phf::Map mismatch
// 2. Allows non-const ColorRange::from() calls
pub static BIOME_COLORS: LazyLock<HashMap<u32, ColorRange>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Biome 0-255
    map.insert(0u32, ColorRange::from(((255, 0, 0), (255, 0, 0)))); // RED
    map.insert(1u32, ColorRange::from(((178, 199, 145), (195, 222, 165)))); // CalmForest
    map.insert(2u32, ColorRange::from(((118, 123, 108), (139, 147, 123)))); // PineWood
    map.insert(3u32, ColorRange::from(((165, 165, 165), (255, 255, 255)))); // Mountain
    map.insert(4u32, ColorRange::from(((218, 236, 189), (218, 236, 189)))); // BreezyPlains
    map.insert(5u32, ColorRange::from(((182, 145, 92), (207, 173, 101)))); // AutumnForest
    map.insert(6u32, ColorRange::from(((232, 166, 223), (147, 50, 105)))); // Tundra
    map.insert(7u32, ColorRange::from(((222, 217, 200), (241, 236, 218)))); // Desert
    map.insert(8u32, ColorRange::from(((143, 188, 143), (143, 188, 143)))); // Swamp
    map.insert(9u32, ColorRange::from(((156, 150, 160), (182, 175, 188)))); // Canyon
    map.insert(10u32, ColorRange::from(((147, 153, 190), (147, 153, 190)))); // Ocean
    map.insert(11u32, ColorRange::from(((250, 236, 189), (250, 236, 189)))); // SafeMeadows
    map.insert(12u32, ColorRange::from(((0, 255, 0), (0, 255, 0)))); // Cave
    map.insert(13u32, ColorRange::from(((0, 0, 255), (0, 0, 255)))); // Jungle
    map.insert(14u32, ColorRange::from(((255, 0, 0), (255, 0, 0)))); // Sapwoods

    // Biome Border (CalmForest Border)
    map.insert(257u32, ColorRange::from(((178, 199, 145), (189, 214, 157))));
    map.insert(258u32, ColorRange::from(((118, 123, 108), (139, 147, 123))));
    map.insert(260u32, ColorRange::from(((218, 236, 189), (218, 236, 189))));
    map.insert(261u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(262u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(263u32, ColorRange::from(((222, 217, 200), (241, 236, 218))));
    map.insert(264u32, ColorRange::from(((143, 188, 143), (143, 188, 143))));
    map.insert(265u32, ColorRange::from(((156, 150, 160), (182, 175, 188))));
    map.insert(266u32, ColorRange::from(((147, 153, 190), (147, 153, 190))));
    map.insert(267u32, ColorRange::from(((250, 236, 189), (250, 236, 189))));
    map.insert(268u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(269u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(270u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // PineWood Border
    map.insert(513u32, ColorRange::from(((178, 199, 145), (189, 214, 157))));
    map.insert(514u32, ColorRange::from(((118, 123, 108), (139, 147, 123))));
    map.insert(515u32, ColorRange::from(((165, 165, 165), (255, 255, 255))));
    map.insert(516u32, ColorRange::from(((218, 236, 189), (218, 236, 189))));
    map.insert(517u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(518u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(519u32, ColorRange::from(((222, 217, 200), (241, 236, 218))));
    map.insert(520u32, ColorRange::from(((143, 188, 143), (143, 188, 143))));
    map.insert(521u32, ColorRange::from(((156, 150, 160), (182, 175, 188))));
    map.insert(522u32, ColorRange::from(((147, 153, 190), (147, 153, 190))));
    map.insert(523u32, ColorRange::from(((250, 236, 189), (250, 236, 189))));
    map.insert(524u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(525u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(526u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Mountain Border
    map.insert(769u32, ColorRange::from(((178, 199, 145), (189, 214, 157))));
    map.insert(770u32, ColorRange::from(((118, 123, 108), (139, 147, 123))));
    map.insert(771u32, ColorRange::from(((165, 165, 165), (255, 255, 255))));
    map.insert(772u32, ColorRange::from(((218, 236, 189), (218, 236, 189))));
    map.insert(773u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(774u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(775u32, ColorRange::from(((222, 217, 200), (241, 236, 218))));
    map.insert(776u32, ColorRange::from(((143, 188, 143), (143, 188, 143))));
    map.insert(777u32, ColorRange::from(((156, 150, 160), (182, 175, 188))));
    map.insert(778u32, ColorRange::from(((147, 153, 190), (147, 153, 190))));
    map.insert(779u32, ColorRange::from(((250, 236, 189), (250, 236, 189))));
    map.insert(780u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(781u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(782u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // BreezyPlains Border
    map.insert(
        1025u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        1026u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        1028u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(1029u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(1030u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        1031u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        1032u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        1033u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        1034u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        1035u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(1036u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(1037u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(1038u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // AutumnForest Border
    map.insert(
        1281u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        1282u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        1284u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(1285u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(1286u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        1287u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        1288u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        1289u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        1290u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        1291u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(1292u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(1293u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(1294u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Tundra Border
    map.insert(
        1537u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        1538u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        1540u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(1541u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(1542u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        1543u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        1544u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        1545u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        1546u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        1547u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(1548u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(1549u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(1550u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Canyon Border
    map.insert(
        2305u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        2306u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        2308u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(2309u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(2310u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        2311u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        2312u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        2313u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        2314u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        2315u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(2316u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(2317u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(2318u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Ocean Border
    map.insert(
        2561u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        2562u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        2564u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(2565u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(2566u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        2567u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        2568u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        2569u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        2570u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        2571u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(2572u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(2573u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(2574u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // SafeMeadows Border
    map.insert(
        2817u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        2818u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        2820u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(2821u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(2822u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        2823u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        2824u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        2825u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        2826u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        2827u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(2828u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(2829u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(2830u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Cave Border
    map.insert(
        3073u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        3074u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        3076u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(3077u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(3078u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        3079u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        3080u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        3081u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        3082u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        3083u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(3084u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(3085u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(3086u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Jungle Border
    map.insert(
        3329u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        3330u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        3332u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(3333u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(3334u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        3335u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        3336u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        3337u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        3338u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        3339u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(3340u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(3341u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(3342u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // Sapwoods Border
    map.insert(
        3585u32,
        ColorRange::from(((178, 199, 145), (189, 214, 157))),
    );
    map.insert(
        3586u32,
        ColorRange::from(((118, 123, 108), (139, 147, 123))),
    );
    map.insert(
        3588u32,
        ColorRange::from(((218, 236, 189), (218, 236, 189))),
    );
    map.insert(3589u32, ColorRange::from(((182, 145, 92), (207, 173, 101))));
    map.insert(3590u32, ColorRange::from(((232, 166, 223), (147, 50, 105))));
    map.insert(
        3591u32,
        ColorRange::from(((222, 217, 200), (241, 236, 218))),
    );
    map.insert(
        3592u32,
        ColorRange::from(((143, 188, 143), (143, 188, 143))),
    );
    map.insert(
        3593u32,
        ColorRange::from(((156, 150, 160), (182, 175, 188))),
    );
    map.insert(
        3594u32,
        ColorRange::from(((147, 153, 190), (147, 153, 190))),
    );
    map.insert(
        3595u32,
        ColorRange::from(((250, 236, 189), (250, 236, 189))),
    );
    map.insert(3596u32, ColorRange::from(((0, 255, 0), (0, 255, 0))));
    map.insert(3597u32, ColorRange::from(((0, 0, 255), (0, 0, 255))));
    map.insert(3598u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    // ??? (3 biomes)
    map.insert(
        66053u32,
        ColorRange::from(((184, 184, 184), (184, 184, 184))),
    );
    map.insert(
        66058u32,
        ColorRange::from(((192, 192, 192), (192, 192, 192))),
    );
    map.insert(
        66562u32,
        ColorRange::from(((200, 200, 200), (200, 200, 200))),
    );
    map.insert(
        66569u32,
        ColorRange::from(((200, 200, 200), (200, 200, 200))),
    );
    map.insert(
        66570u32,
        ColorRange::from(((200, 200, 200), (200, 200, 200))),
    );
    map.insert(
        66573u32,
        ColorRange::from(((200, 200, 200), (200, 200, 200))),
    );
    map.insert(
        66818u32,
        ColorRange::from(((208, 208, 208), (208, 208, 208))),
    );
    map.insert(
        66826u32,
        ColorRange::from(((208, 208, 208), (208, 208, 208))),
    );
    map.insert(
        67850u32,
        ColorRange::from(((216, 216, 216), (216, 216, 216))),
    );
    map.insert(
        131332u32,
        ColorRange::from(((224, 224, 224), (224, 224, 224))),
    );
    map.insert(
        131333u32,
        ColorRange::from(((224, 224, 224), (224, 224, 224))),
    );
    map.insert(
        131338u32,
        ColorRange::from(((224, 224, 224), (224, 224, 224))),
    );
    map.insert(
        132097u32,
        ColorRange::from(((232, 232, 232), (232, 232, 232))),
    );
    map.insert(
        132353u32,
        ColorRange::from(((232, 232, 232), (232, 232, 232))),
    );
    map.insert(
        132618u32,
        ColorRange::from(((232, 232, 232), (232, 232, 232))),
    );
    map.insert(
        262402u32,
        ColorRange::from(((240, 240, 240), (240, 240, 240))),
    );
    map.insert(
        262409u32,
        ColorRange::from(((240, 240, 240), (240, 240, 240))),
    );
    map.insert(
        262410u32,
        ColorRange::from(((240, 240, 240), (240, 240, 240))),
    );
    map.insert(
        262657u32,
        ColorRange::from(((240, 240, 240), (240, 240, 240))),
    );
    map.insert(
        264449u32,
        ColorRange::from(((240, 240, 240), (240, 240, 240))),
    );
    map.insert(
        327938u32,
        ColorRange::from(((248, 248, 248), (248, 248, 248))),
    );
    map.insert(
        327946u32,
        ColorRange::from(((248, 248, 248), (248, 248, 248))),
    );
    map.insert(
        328193u32,
        ColorRange::from(((248, 248, 248), (248, 248, 248))),
    );
    map.insert(
        393738u32,
        ColorRange::from(((255, 255, 255), (255, 255, 255))),
    );
    map.insert(590090u32, ColorRange::from(((255, 0, 0), (255, 0, 0))));

    map
});
