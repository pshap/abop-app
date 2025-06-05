//! Serialization support for Material Design 3 elevation system

use super::{ElevationStyle, MaterialElevation};
use iced::{Color, Shadow, Vector};
use serde::{
    Deserialize, Serialize, Serializer,
    de::{Deserializer, MapAccess, Visitor},
    ser::SerializeStruct,
};
use std::fmt;

// Serialization for ElevationStyle
impl Serialize for ElevationStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ElevationStyle", 4)?;
        state.serialize_field("level", &self.level)?;
        state.serialize_field("dp", &self.dp)?;
        state.serialize_field("tint_opacity", &self.tint_opacity)?;

        // Serialize shadow as tuple: (color_rgba, offset_xy, blur_radius)
        let shadow_data = (
            [
                self.shadow.color.r,
                self.shadow.color.g,
                self.shadow.color.b,
                self.shadow.color.a,
            ],
            [self.shadow.offset.x, self.shadow.offset.y],
            self.shadow.blur_radius,
        );
        state.serialize_field("shadow", &shadow_data)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ElevationStyle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElevationStyleVisitor;

        impl<'de> Visitor<'de> for ElevationStyleVisitor {
            type Value = ElevationStyle;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ElevationStyle")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ElevationStyle, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut level = None;
                let mut dp = None;
                let mut tint_opacity = None;
                let mut shadow = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "level" => {
                            if level.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level = Some(map.next_value()?);
                        }
                        "dp" => {
                            if dp.is_some() {
                                return Err(serde::de::Error::duplicate_field("dp"));
                            }
                            dp = Some(map.next_value()?);
                        }
                        "tint_opacity" => {
                            if tint_opacity.is_some() {
                                return Err(serde::de::Error::duplicate_field("tint_opacity"));
                            }
                            tint_opacity = Some(map.next_value()?);
                        }
                        "shadow" => {
                            if shadow.is_some() {
                                return Err(serde::de::Error::duplicate_field("shadow"));
                            }
                            let shadow_data: ([f32; 4], [f32; 2], f32) = map.next_value()?;
                            shadow = Some(Shadow {
                                color: Color {
                                    r: shadow_data.0[0],
                                    g: shadow_data.0[1],
                                    b: shadow_data.0[2],
                                    a: shadow_data.0[3],
                                },
                                offset: Vector::new(shadow_data.1[0], shadow_data.1[1]),
                                blur_radius: shadow_data.2,
                            });
                        }
                        _ => {
                            // Ignore unknown fields
                        }
                    }
                }

                let level = level.ok_or_else(|| serde::de::Error::missing_field("level"))?;
                let dp = dp.ok_or_else(|| serde::de::Error::missing_field("dp"))?;
                let tint_opacity =
                    tint_opacity.ok_or_else(|| serde::de::Error::missing_field("tint_opacity"))?;
                let shadow = shadow.ok_or_else(|| serde::de::Error::missing_field("shadow"))?;

                Ok(ElevationStyle {
                    level,
                    dp,
                    shadow,
                    tint_opacity,
                })
            }
        }

        const FIELDS: &[&str] = &["level", "dp", "tint_opacity", "shadow"];
        deserializer.deserialize_struct("ElevationStyle", FIELDS, ElevationStyleVisitor)
    }
}

// Serialization for MaterialElevation
impl Serialize for MaterialElevation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MaterialElevation", 6)?;
        state.serialize_field("level0", &self.level0)?;
        state.serialize_field("level1", &self.level1)?;
        state.serialize_field("level2", &self.level2)?;
        state.serialize_field("level3", &self.level3)?;
        state.serialize_field("level4", &self.level4)?;
        state.serialize_field("level5", &self.level5)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for MaterialElevation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MaterialElevationVisitor;

        impl<'de> Visitor<'de> for MaterialElevationVisitor {
            type Value = MaterialElevation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MaterialElevation")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MaterialElevation, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut level0 = None;
                let mut level1 = None;
                let mut level2 = None;
                let mut level3 = None;
                let mut level4 = None;
                let mut level5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "level0" => {
                            if level0.is_some() {
                                return Err(serde::de::Error::duplicate_field("level0"));
                            }
                            level0 = Some(map.next_value()?);
                        }
                        "level1" => {
                            if level1.is_some() {
                                return Err(serde::de::Error::duplicate_field("level1"));
                            }
                            level1 = Some(map.next_value()?);
                        }
                        "level2" => {
                            if level2.is_some() {
                                return Err(serde::de::Error::duplicate_field("level2"));
                            }
                            level2 = Some(map.next_value()?);
                        }
                        "level3" => {
                            if level3.is_some() {
                                return Err(serde::de::Error::duplicate_field("level3"));
                            }
                            level3 = Some(map.next_value()?);
                        }
                        "level4" => {
                            if level4.is_some() {
                                return Err(serde::de::Error::duplicate_field("level4"));
                            }
                            level4 = Some(map.next_value()?);
                        }
                        "level5" => {
                            if level5.is_some() {
                                return Err(serde::de::Error::duplicate_field("level5"));
                            }
                            level5 = Some(map.next_value()?);
                        }
                        _ => {
                            // Ignore unknown fields
                        }
                    }
                }

                let level0 = level0.ok_or_else(|| serde::de::Error::missing_field("level0"))?;
                let level1 = level1.ok_or_else(|| serde::de::Error::missing_field("level1"))?;
                let level2 = level2.ok_or_else(|| serde::de::Error::missing_field("level2"))?;
                let level3 = level3.ok_or_else(|| serde::de::Error::missing_field("level3"))?;
                let level4 = level4.ok_or_else(|| serde::de::Error::missing_field("level4"))?;
                let level5 = level5.ok_or_else(|| serde::de::Error::missing_field("level5"))?;

                Ok(MaterialElevation {
                    level0,
                    level1,
                    level2,
                    level3,
                    level4,
                    level5,
                })
            }
        }

        const FIELDS: &[&str] = &["level0", "level1", "level2", "level3", "level4", "level5"];
        deserializer.deserialize_struct("MaterialElevation", FIELDS, MaterialElevationVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::elevation::{ElevationLevel, MaterialElevation};
    use iced::Color;

    #[test]
    fn test_elevation_style_serialization() {
        let style = ElevationStyle::new(ElevationLevel::Level2, Color::BLACK, Color::WHITE);

        // Test serialization
        let serialized = serde_json::to_string(&style).unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized: ElevationStyle = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.level, style.level);
        assert_eq!(deserialized.dp, style.dp);
        assert_eq!(deserialized.tint_opacity, style.tint_opacity);

        // Check shadow properties
        assert_eq!(deserialized.shadow.offset.y, style.shadow.offset.y);
        assert_eq!(deserialized.shadow.blur_radius, style.shadow.blur_radius);
        assert_eq!(deserialized.shadow.color.a, style.shadow.color.a);
    }

    #[test]
    fn test_material_elevation_serialization() {
        let elevation = MaterialElevation::default();

        // Test serialization
        let serialized = serde_json::to_string(&elevation).unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized: MaterialElevation = serde_json::from_str(&serialized).unwrap();

        // Check that all levels match
        for level in ElevationLevel::all() {
            let original = elevation.get_level(*level);
            let deserialized_style = deserialized.get_level(*level);
            assert_eq!(original.level, deserialized_style.level);
            assert_eq!(original.dp, deserialized_style.dp);
            assert_eq!(original.tint_opacity, deserialized_style.tint_opacity);
        }
    }
}
