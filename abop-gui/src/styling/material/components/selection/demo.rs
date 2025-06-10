//! Demo module for selection components
//! 
//! This module demonstrates how to use various selection components
//! including chips, checkboxes, switches, and radio groups.

use iced::Element;
use iced::widget::{Column, Container, Text};

use crate::styling::material::components::selection::{
    chip::{
        ChipBuilder,
        ChipState,
        ChipVariant,
        ChipCollectionBuilder,
        ChipSelectionMode,
    },
    checkbox::{
        CheckboxBuilder,
        CheckboxState,
    },
    switch::{
        SwitchBuilder,
        SwitchState,
    },
    radio::RadioGroupState,
    ComponentSize,
};

/// Demonstrates the usage of checkboxes
pub fn demo_checkbox() -> Element<'static, ()> {
    let mut col = Column::new()
        .spacing(10)
        .push(Text::new("Checkbox Examples"));

    // Add checkboxes with different states
    let checkboxes = [
        ("Unchecked", CheckboxState::Unchecked),
        ("Checked", CheckboxState::Checked),
        ("Indeterminate", CheckboxState::Indeterminate),
    ];

    for (label, state) in checkboxes {
        let checkbox = CheckboxBuilder::new()
            .with_label(label)
            .with_state(state)
            .build()
            .expect("Failed to build checkbox");
        
        col = col.push(checkbox.view());
    }

    Container::new(col).into()
}

/// Demonstrates the usage of switches
pub fn demo_switch() -> Element<'static, ()> {
    let mut col = Column::new()
        .spacing(10)
        .push(Text::new("Switch Examples"));

    // Add switches with different states
    let switches = [
        ("Off", SwitchState::Off),
        ("On", SwitchState::On),
    ];

    for (label, state) in switches {
        let switch = SwitchBuilder::new()
            .with_label(label)
            .with_state(state)
            .build()
            .expect("Failed to build switch");
        
        col = col.push(switch.view());
    }

    Container::new(col).into()
}

/// Demonstrates the usage of chips
pub fn demo_chip() -> Element<'static, ()> {
    let mut col = Column::new()
        .spacing(10)
        .push(Text::new("Chip Examples"));

    // Add different chip variants
    let variants = [
        ("Assist", ChipVariant::Assist),
        ("Filter", ChipVariant::Filter),
        ("Input", ChipVariant::Input),
        ("Suggestion", ChipVariant::Suggestion),
    ];

    for (label, variant) in variants {
        let chip = ChipBuilder::new()
            .with_label(label)
            .with_variant(variant)
            .with_state(if label == "Filter" { 
                ChipState::Selected 
            } else { 
                ChipState::Unselected 
            })
            .build()
            .expect("Failed to build chip");
        
        col = col.push(chip.view(None, &Default::default()));
    }

    Container::new(col).into()
}

/// Demonstrates the usage of chip collections
pub fn demo_chip_collection() -> Element<'static, ()> {
    let mut col = Column::new()
        .spacing(20)
        .push(Text::new("Chip Collection Examples"));

    // Create a single-selection chip collection
    let single_select = {
        let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Single);
        
        // Add some chips
        for i in 0..3 {
            let chip = ChipBuilder::new()
                .with_label(format!("Option {}", i + 1))
                .build()
                .expect("Failed to build chip");
            
            builder = builder.add_chip(chip);
        }
        
        // Select the first chip
        builder = builder.with_selected_index(0);
        
        builder.build().expect("Failed to build chip collection")
    };

    col = col.push(Text::new("Single Selection"));
    col = col.push(single_select.view(&Default::default()));

    // Create a multi-selection chip collection
    let multi_select = {
        let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Multiple);
        
        // Add some chips
        for i in 0..5 {
            let chip = ChipBuilder::new()
                .with_label(format!("Item {}", (i as u8 + b'A') as char))
                .with_variant(ChipVariant::Filter)
                .build()
                .expect("Failed to build chip");
            
            builder = builder.add_chip(chip);
        }
        
        // Select first and third chips
        builder = builder.with_selected_indices(vec![0, 2]);
        
        builder.build().expect("Failed to build chip collection")
    };

    col = col.push(Text::new("Multiple Selection"));
    col = col.push(multi_select.view(&Default::default()));

    Container::new(col).into()
}

/// Demonstrates the usage of radio groups
pub fn demo_radio_group() -> Element<'static, ()> {
    let mut col = Column::new()
        .spacing(10)
        .push(Text::new("Radio Group Examples"));

    // Create a radio group
    let mut radio_group = RadioGroupState::new();
    
    // Add options
    radio_group.add_option("option1", "Option 1");
    radio_group.add_option("option2", "Option 2");
    radio_group.add_option("option3", "Option 3");
    
    // Set the selected option
    radio_group.set_selected("option1");
    
    // Create radio buttons for each option
    for (value, label) in radio_group.options() {
        let is_selected = radio_group.selected_value() == Some(value);
        let radio = iced::widget::radio(label, value, radio_group.selected_value(), |_| ());
        col = col.push(radio);
    }

    Container::new(col).into()
}

/// Main demo function that combines all demos
pub fn demo() -> Element<'static, ()> {
    let content = Column::new()
        .spacing(20)
        .push(Text::new("Selection Components Demo").size(24))
        .push(demo_checkbox())
        .push(demo_switch())
        .push(demo_chip())
        .push(demo_chip_collection())
        .push(demo_radio_group());

    Container::new(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .padding(20)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_functions() {
        // Just verify that all demo functions run without panicking
        let _ = demo_checkbox();
        let _ = demo_switch();
        let _ = demo_chip();
        let _ = demo_chip_collection();
        let _ = demo_radio_group();
        let _ = demo();
    }
}
