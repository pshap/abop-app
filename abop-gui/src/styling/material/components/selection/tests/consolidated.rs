//! Consolidated tests for Material Design selection components
//! 
//! This module contains unified tests for checkbox, chip, switch, and related
//! selection component functionality.

use crate::styling::material::components::selection::*;
use crate::styling::material::components::selection::builder::*;

#[cfg(test)]
mod checkbox_tests {
    use super::*;

    #[test]
    fn test_checkbox_builder_creation() {
        let checkbox = CheckboxBuilder::new(CheckboxState::Checked)
            .label("Test checkbox")
            .build()
            .expect("Should create valid checkbox");
        
        assert_eq!(checkbox.state, CheckboxState::Checked);
        assert_eq!(checkbox.props.label, Some("Test checkbox".to_string()));
    }

    #[test]
    fn test_checkbox_convenience_methods() {
        let checked_checkbox = CheckboxBuilder::checked()
            .build()
            .expect("Should create checked checkbox");
        assert_eq!(checked_checkbox.state, CheckboxState::Checked);

        let unchecked_checkbox = CheckboxBuilder::unchecked()
            .build()
            .expect("Should create unchecked checkbox");
        assert_eq!(unchecked_checkbox.state, CheckboxState::Unchecked);

        let bool_checkbox = CheckboxBuilder::from_bool(true)
            .build()
            .expect("Should create checkbox from bool");
        assert_eq!(bool_checkbox.state, CheckboxState::Checked);
    }

    #[test]
    fn test_checkbox_indeterminate_state() {
        let checkbox = CheckboxBuilder::indeterminate()
            .label("Indeterminate")
            .build()
            .expect("Should create indeterminate checkbox");
        
        assert_eq!(checkbox.state, CheckboxState::Indeterminate);
        assert_eq!(checkbox.props.label, Some("Indeterminate".to_string()));
    }
}

#[cfg(test)]
mod chip_tests {
    use super::*;

    #[test]
    fn test_chip_builder_creation() {
        let chip = ChipBuilder::new("Test Chip", ChipVariant::Filter)
            .build()
            .expect("Should create valid chip");
        
        assert_eq!(chip.label, "Test Chip");
        assert_eq!(chip.variant, ChipVariant::Filter);
        assert_eq!(chip.state, ChipState::Unselected);
    }

    #[test]
    fn test_chip_convenience_methods() {
        let filter_chip = ChipBuilder::filter("Filter")
            .build()
            .expect("Should create filter chip");
        assert_eq!(filter_chip.variant, ChipVariant::Filter);

        let assist_chip = ChipBuilder::assist("Assist")
            .build()
            .expect("Should create assist chip");
        assert_eq!(assist_chip.variant, ChipVariant::Assist);

        let input_chip = ChipBuilder::input("Input")
            .build()
            .expect("Should create input chip");
        assert_eq!(input_chip.variant, ChipVariant::Input);
    }

    #[test]
    fn test_chip_selection_state() {
        let chip = ChipBuilder::filter("Toggle Chip")
            .selected(true)
            .build()
            .expect("Should create selected chip");
        
        assert_eq!(chip.state, ChipState::Selected);
        assert_eq!(chip.label, "Toggle Chip");
    }
}

#[cfg(test)]
mod switch_tests {
    use super::*;

    #[test]
    fn test_switch_builder_creation() {
        let switch = SwitchBuilder::new(SwitchState::On)
            .label("Test switch")
            .build()
            .expect("Should create valid switch");
        
        assert_eq!(switch.state, SwitchState::On);
        assert_eq!(switch.props.label, Some("Test switch".to_string()));
    }

    #[test]
    fn test_switch_convenience_methods() {
        let on_switch = SwitchBuilder::on()
            .build()
            .expect("Should create on switch");
        assert_eq!(on_switch.state, SwitchState::On);

        let off_switch = SwitchBuilder::off()
            .build()
            .expect("Should create off switch");
        assert_eq!(off_switch.state, SwitchState::Off);

        let bool_switch = SwitchBuilder::from_bool(true)
            .build()
            .expect("Should create switch from bool");
        assert_eq!(bool_switch.state, SwitchState::On);
    }

    #[test]
    fn test_switch_labeling() {
        let switch = SwitchBuilder::off()
            .label("Enable feature")
            .build()
            .expect("Should create labeled switch");
        
        assert_eq!(switch.props.label, Some("Enable feature".to_string()));
        assert_eq!(switch.state, SwitchState::Off);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_selection_component_sizing() {
        let checkbox = CheckboxBuilder::checked()
            .size(ComponentSize::Large)
            .build()
            .expect("Should create large checkbox");
        
        let chip = ChipBuilder::filter("Test")
            .size(ComponentSize::Large)
            .build()
            .expect("Should create large chip");
        
        let switch = SwitchBuilder::on()
            .size(ComponentSize::Large)
            .build()
            .expect("Should create large switch");
        
        assert_eq!(checkbox.props.size, ComponentSize::Large);
        assert_eq!(chip.props.size, ComponentSize::Large);
        assert_eq!(switch.props.size, ComponentSize::Large);
    }

    #[test]
    fn test_selection_component_labeling() {
        let checkbox = CheckboxBuilder::unchecked()
            .label("Checkbox Label")
            .build()
            .expect("Should create labeled checkbox");
        
        let switch = SwitchBuilder::off()
            .label("Switch Label")
            .build()
            .expect("Should create labeled switch");
        
        assert_eq!(checkbox.props.label, Some("Checkbox Label".to_string()));
        assert_eq!(switch.props.label, Some("Switch Label".to_string()));
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_chip_empty_label_validation() {
        let result = ChipBuilder::filter("")
            .build();
        
        // Empty labels should cause validation errors
        assert!(result.is_err());
    }

    #[test]
    fn test_chip_valid_label_validation() {
        let result = ChipBuilder::filter("Valid Label")
            .build();
        
        assert!(result.is_ok());
    }

    #[test] 
    fn test_component_builder_patterns() {
        // Test that all builders follow the same patterns
        let checkbox = CheckboxBuilder::from_bool(true)
            .label("Pattern Test")
            .build()
            .expect("Should build checkbox");
        
        let chip = ChipBuilder::suggestion("Pattern Test")
            .build()
            .expect("Should build chip");
        
        // Verify consistent behavior
        assert_eq!(checkbox.props.label, Some("Pattern Test".to_string()));
        assert_eq!(chip.label, "Pattern Test");
    }

    #[test]
    fn test_builder_chaining() {
        let chip = ChipBuilder::filter("Chain Test")
            .selected(true)
            .size(ComponentSize::Medium)
            .disabled(false)
            .build()
            .expect("Should build with chained methods");
        
        assert_eq!(chip.label, "Chain Test");
        assert_eq!(chip.state, ChipState::Selected);
        assert_eq!(chip.props.size, ComponentSize::Medium);
        assert!(!chip.props.disabled);
    }
}