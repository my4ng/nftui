use nftnl::{Hook, ProtoFamily};
use ratatui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HookFamily(Hook, ProtoFamily, u16, u16);

impl std::fmt::Display for HookFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hook, proto_family) = self.to_hook_family();
        fn hook_to_str(hook: Hook) -> &'static str {
            use Hook::*;
            match hook {
                In => "input",
                Out => "output",
                Forward => "forward",
                PreRouting => "prerouting",
                PostRouting => "postrouting",
            }
        }

        fn proto_family_to_str(proto_family: ProtoFamily) -> &'static str {
            use ProtoFamily::*;
            match proto_family {
                Arp => "arp",
                Bridge => "bridge",
                Inet => "inet",
                Ipv4 => "ipv4",
                Ipv6 => "ipv6",
                NetDev => "netdev",
                Unspec | DecNet => unimplemented!(),
            }
        }

        write!(f, "HOOK: {:6} | {:11} ", proto_family_to_str(proto_family), hook_to_str(hook))
    }
}

impl Default for HookFamily {
    fn default() -> Self {
        HOOK_FAMILIES[0]
    }
}

impl HookFamily {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn move_by_direction(self, direction: Direction) -> Self {
        let (sx, sy) = self.to_xy();
        let move_scores = HOOK_FAMILIES.map(|hf| {
            let (x, y) = hf.to_xy();
            let dx = x as i16 - sx as i16;
            let dy = y as i16 - sy as i16;
            (hf, Self::move_score((dx, dy), direction))
        });

        move_scores
            .iter()
            .copied()
            .min_by_key(|(_, score)| *score)
            .unwrap()
            .0
    }

    #[inline]
    fn move_score((dx, dy): (i16, i16), direction: Direction) -> u16 {
        use self::Direction::*;
        let (major, minor) = match direction {
            Up => (-dy, dx),
            Down => (dy, dx),
            Left => (-dx, dy),
            Right => (dx, dy),
        };

        if major < 0 || major == 0 && minor != 0 {
            u16::MAX
        } else if major == 0 {
            u16::MAX - 1
        } else {
            (major + minor.abs() * 3) as u16
        }
    }

    fn to_xy(self) -> (u16, u16) {
        let HookFamily(_, _, x, y) = self;
        (x, y)
    }

    fn to_hook_family(self) -> (Hook, ProtoFamily) {
        let HookFamily(hook, proto_family, _, _) = self;
        (hook, proto_family)
    }
}

// 23x10
const DIAGRAM_STRS: &[&str] = &[
    "         ┌┄┄┄┐",
    "         █   └─█",
    "         │     │",
    "     ┌─█─┴─█───█─┬─┐",
    "     │ │         │ │",
    "     │ └─█     █─┘ │",
    "     │   │     │   │",
    "┄┄┄█─┼─█─┴─█───█───█┄┄┄",
    "     │             │",
    "     └───█┄┄┄┄┄█───┘",
];

static HOOK_FAMILIES: [HookFamily; 12] = {
    use Hook::*;
    use ProtoFamily::*;
    [
        HookFamily(In, Inet, 9, 1),
        HookFamily(Out, Inet, 15, 1),
        HookFamily(PreRouting, Inet, 7, 3),
        HookFamily(Forward, Inet, 11, 3),
        HookFamily(PostRouting, Inet, 15, 3),
        HookFamily(In, Bridge, 9, 5),
        HookFamily(Out, Bridge, 15, 5),
        HookFamily(PreRouting, Bridge, 7, 7),
        HookFamily(Forward, Bridge, 11, 7),
        HookFamily(PostRouting, Bridge, 15, 7),
        HookFamily(In, Arp, 9, 9),
        HookFamily(Out, Arp, 15, 9),
    ]
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Diagram {
    selected: HookFamily,
}

impl Diagram {
    pub fn new(selected: HookFamily) -> Self {
        Self { selected }
    }
}

impl Widget for Diagram {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        debug_assert!(area.width == 23 && area.height == 10);

        for (y, line) in DIAGRAM_STRS.iter().enumerate() {
            buf.set_string(area.x, area.y + y as u16, line, Style::default());
        }

        let (x, y) = self.selected.to_xy();
        buf.get_mut(area.x + x, area.y + y).set_fg(Color::Red);
    }
}
