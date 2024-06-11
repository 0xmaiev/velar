/// Statuses and status effects are going to be very context specific..
///
/// How to properly organize and represent them?
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub enum Status {
    #[default]
    Default,
}

/// An object in the world.
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub enum Object {
    #[default]
    Default,

    /// A resource may yield a material or an item.
    ///
    /// ### Resources
    ///
    /// - Ores
    /// - Trees
    /// - Fishing Schools
    /// - Chests
    ///
    Resource,

    /// A material can be combined with other materials or items in order
    /// to craft new materials and/or items.
    ///
    /// ### Materials
    ///
    /// - Wood
    /// - Copper
    /// - Tin
    /// - Iron
    /// - Steel
    /// - Titanium
    ///
    Material,

    /// An item can be consumed for potentially beneficial effects, used for
    /// crafting recipes and/or worn by characters.
    ///
    /// ### Items
    ///
    /// -
    Item,
}

/// A character skill.
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Skill {
    /// The name of the skill.
    ///
    /// Encoded as zero-padded UTF-8 string.
    pub name: [u8; 16], // 16
    /// The maximum level of the skill.
    pub max_level: u16, // 18
    // Padding.
    pub(crate) padding: [u8; 6], // 24
    /// The maximum experience attainable in the skill.
    pub max_experience: u128, // 40
}

/// A character ability.
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Ability {
    /// The name of the ability.
    ///
    /// Encoded as zero-padded UTF-8 string.
    pub name: [u8; 16], // 16
}

/// A type alias for a bidimensional grid of [T].
pub type Grid2d<T> = Vec<Vec<T>>;

/// A type alias for a tridimensional grid of [T]
pub type Grid3d<T> = Vec<Vec<Vec<T>>>;

/// A tile in the world.
#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Tile {}

/// A type alias for a 2d world.
pub type World2d = Grid2d<Tile>;

/// A type alias for a 3d world.
pub type World3d = Grid3d<Tile>;

pub fn generate_tile() -> Tile {
    Tile {}
}

pub fn generate_world_2d() -> World2d {
    World2d::new()
}

pub fn generate_world_3d() -> World3d {
    World3d::new()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ability() {
        let ability_name = "Rain of Thunder";
        let ability_name_bytes = ability_name.bytes();
        let len = ability_name_bytes.len();
        println!("{:?}", len);
        assert!(16 > len);
    }

    #[test]
    fn test_experience_per_hour() {
        let resource_tier_experiences = [5, 15, 25, 40, 80, 240, 360, 480, 600, 1000, 1500, 2000];
        let resource_tier_durations = [10, 15, 20, 25, 30, 60, 70, 80, 90, 120, 150, 180];

        for i in 0..12 {
            let max_per_hour = 3600 / resource_tier_durations[i];
            let xp_per_hour = max_per_hour * resource_tier_experiences[i];

            println!(
                "Tier {} - {} max harvests ph - {} max xp ph",
                i + 1,
                max_per_hour,
                xp_per_hour
            );
        }
    }
}
