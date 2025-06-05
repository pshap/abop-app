//! Easing curves for Material Design 3 motion system
//!
//! Provides accurate cubic Bezier implementations of Material Design 3 easing curves
//! with efficient static storage and high-precision sampling.

use std::collections::HashMap;

/// Material Design easing types
///
/// Defines the different easing curves available in the Material Design 3 motion system.
/// Each type corresponds to a specific cubic Bezier curve optimized for different animation contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EasingType {
    /// Linear motion with constant velocity throughout the animation
    Linear,
    /// Standard easing curve - the most commonly used curve for UI animations
    Standard,
    /// Standard curve with emphasis on acceleration at the beginning
    StandardAccelerate,
    /// Standard curve with emphasis on deceleration at the end
    StandardDecelerate,
    /// Emphasized easing for important or expressive animations
    Emphasized,
    /// Emphasized curve with dramatic acceleration
    EmphasizedAccelerate,
    /// Emphasized curve with dramatic deceleration
    EmphasizedDecelerate,
    /// Legacy easing from Material Design 2 for backward compatibility
    Legacy,
    /// Legacy curve with acceleration emphasis
    LegacyAccelerate,
    /// Legacy curve with deceleration emphasis
    LegacyDecelerate,
}

/// Cubic Bezier curve control points
///
/// Defines the four control points of a cubic Bezier curve used for easing functions.
/// The curve starts at (0,0) and ends at (1,1), with x1,y1 and x2,y2 defining
/// the intermediate control points that shape the curve.
#[derive(Debug, Clone, PartialEq)]
pub struct CubicBezier {
    /// X coordinate of the first control point (0.0-1.0)
    pub x1: f32,
    /// Y coordinate of the first control point
    pub y1: f32,
    /// X coordinate of the second control point (0.0-1.0)
    pub x2: f32,
    /// Y coordinate of the second control point
    pub y2: f32,
}

/// Material Design easing curve
///
/// Represents a timing function for animations using cubic Bezier curves.
/// Each curve has a name, control points, and description for its intended use.
#[derive(Debug, Clone, PartialEq)]
pub struct EasingCurve {
    /// The name identifier for this easing curve
    pub name: &'static str,
    /// The cubic Bezier control points defining the curve
    pub control_points: CubicBezier,
    /// Human-readable description of the curve's characteristics
    pub description: &'static str,
}

impl CubicBezier {
    /// Create a new cubic bezier curve
    #[must_use]
    pub const fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Calculate the curve value at time t (0.0 to 1.0) using accurate x->t solving
    ///
    /// This implementation uses the Newton-Raphson method to accurately solve for
    /// the parameter t given input x, then calculates the corresponding y value.
    #[must_use]
    pub fn sample(&self, x: f32) -> f32 {
        let x = x.clamp(0.0, 1.0);

        // For linear curves, skip expensive calculation
        if self.is_linear() {
            return x;
        }

        // Use Newton-Raphson method to solve for t given x
        let t = self.solve_t_for_x(x);
        self.sample_y(t)
    }

    /// Check if this is a linear curve
    fn is_linear(&self) -> bool {
        (self.x1 - 0.0).abs() < f32::EPSILON
            && (self.y1 - 0.0).abs() < f32::EPSILON
            && (self.x2 - 1.0).abs() < f32::EPSILON
            && (self.y2 - 1.0).abs() < f32::EPSILON
    }

    /// Solve for parameter t given x coordinate using Newton-Raphson method
    fn solve_t_for_x(&self, x: f32) -> f32 {
        // Handle edge cases
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }

        // Initial guess
        let mut t = x;

        // Newton-Raphson iterations
        for _ in 0..8 {
            let x_calc = self.sample_x(t);
            let x_derivative = self.sample_x_derivative(t);

            if x_derivative.abs() < f32::EPSILON {
                break;
            }

            t -= (x_calc - x) / x_derivative;
            t = t.clamp(0.0, 1.0);

            // Check for convergence
            if (x_calc - x).abs() < 1e-6 {
                break;
            }
        }

        t
    }

    /// Calculate x coordinate at parameter t
    fn sample_x(&self, t: f32) -> f32 {
        let one_minus_t = 1.0 - t;
        let one_minus_t_squared = one_minus_t * one_minus_t;
        let one_minus_t_cubed = one_minus_t_squared * one_minus_t;
        let t_squared = t * t;
        let t_cubed = t_squared * t;

        t_cubed.mul_add(
            1.0,
            (3.0 * one_minus_t * t_squared).mul_add(
                self.x2,
                one_minus_t_cubed.mul_add(0.0, 3.0 * one_minus_t_squared * t * self.x1),
            ),
        ) // P3.x (always 1)
    }

    /// Calculate y coordinate at parameter t
    fn sample_y(&self, t: f32) -> f32 {
        let one_minus_t = 1.0 - t;
        let one_minus_t_squared = one_minus_t * one_minus_t;
        let one_minus_t_cubed = one_minus_t_squared * one_minus_t;
        let t_squared = t * t;
        let t_cubed = t_squared * t;

        t_cubed.mul_add(
            1.0,
            (3.0 * one_minus_t * t_squared).mul_add(
                self.y2,
                one_minus_t_cubed.mul_add(0.0, 3.0 * one_minus_t_squared * t * self.y1),
            ),
        ) // P3.y (always 1)
    }

    /// Calculate derivative of x with respect to t
    fn sample_x_derivative(&self, t: f32) -> f32 {
        let one_minus_t = 1.0 - t;
        let one_minus_t_squared = one_minus_t * one_minus_t;
        let t_squared = t * t;

        (3.0 * t_squared).mul_add(
            1.0 - self.x2,
            (3.0 * one_minus_t_squared)
                .mul_add(self.x1, 6.0 * one_minus_t * t * (self.x2 - self.x1)),
        )
    }

    /// Get CSS cubic-bezier string representation
    #[must_use]
    pub fn to_css(&self) -> String {
        format!(
            "cubic-bezier({}, {}, {}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

impl EasingCurve {
    /// Create a new easing curve
    #[must_use]
    pub const fn new(
        name: &'static str,
        control_points: CubicBezier,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            control_points,
            description,
        }
    }

    /// Sample the curve at time t
    #[must_use]
    pub fn sample(&self, t: f32) -> f32 {
        self.control_points.sample(t)
    }

    /// Get the CSS representation
    #[must_use]
    pub fn to_css(&self) -> String {
        self.control_points.to_css()
    }
}

/// Static storage for easing curves
///
/// Uses `lazy_static` pattern to initialize curves once and reuse them.
/// This eliminates memory overhead of storing curves in every animation.
use std::sync::OnceLock;

static EASING_CURVES: OnceLock<HashMap<EasingType, EasingCurve>> = OnceLock::new();

/// Initialize the static easing curves
fn create_easing_curves() -> HashMap<EasingType, EasingCurve> {
    let mut curves = HashMap::new();

    curves.insert(
        EasingType::Linear,
        EasingCurve::new(
            "linear",
            CubicBezier::new(0.0, 0.0, 1.0, 1.0),
            "Linear motion with no acceleration or deceleration",
        ),
    );

    curves.insert(
        EasingType::Standard,
        EasingCurve::new(
            "standard",
            CubicBezier::new(0.2, 0.0, 0.0, 1.0),
            "Standard easing for most animations",
        ),
    );

    curves.insert(
        EasingType::StandardAccelerate,
        EasingCurve::new(
            "standard-accelerate",
            CubicBezier::new(0.3, 0.0, 1.0, 1.0),
            "Standard easing that accelerates",
        ),
    );

    curves.insert(
        EasingType::StandardDecelerate,
        EasingCurve::new(
            "standard-decelerate",
            CubicBezier::new(0.0, 0.0, 0.0, 1.0),
            "Standard easing that decelerates",
        ),
    );

    curves.insert(
        EasingType::Emphasized,
        EasingCurve::new(
            "emphasized",
            CubicBezier::new(0.2, 0.0, 0.0, 1.0),
            "Emphasized easing for important animations",
        ),
    );

    curves.insert(
        EasingType::EmphasizedAccelerate,
        EasingCurve::new(
            "emphasized-accelerate",
            CubicBezier::new(0.3, 0.0, 0.8, 0.15),
            "Emphasized easing that accelerates",
        ),
    );

    curves.insert(
        EasingType::EmphasizedDecelerate,
        EasingCurve::new(
            "emphasized-decelerate",
            CubicBezier::new(0.05, 0.7, 0.1, 1.0),
            "Emphasized easing that decelerates",
        ),
    );

    curves.insert(
        EasingType::Legacy,
        EasingCurve::new(
            "legacy",
            CubicBezier::new(0.4, 0.0, 0.2, 1.0),
            "Legacy easing for backward compatibility",
        ),
    );

    curves.insert(
        EasingType::LegacyAccelerate,
        EasingCurve::new(
            "legacy-accelerate",
            CubicBezier::new(0.4, 0.0, 1.0, 1.0),
            "Legacy easing that accelerates",
        ),
    );

    curves.insert(
        EasingType::LegacyDecelerate,
        EasingCurve::new(
            "legacy-decelerate",
            CubicBezier::new(0.0, 0.0, 0.2, 1.0),
            "Legacy easing that decelerates",
        ),
    );

    curves
}

/// Access to easing curves
impl super::MotionTokens {
    /// Get easing curve by type
    ///
    /// Returns a reference to a statically stored easing curve.
    /// This is O(1) and very memory efficient.
    ///
    /// # Panics
    ///
    /// Panics if the easing type is not found in the initialized curves (this should never happen
    /// as all easing types are initialized at startup)
    pub fn easing(easing_type: EasingType) -> &'static EasingCurve {
        let curves = EASING_CURVES.get_or_init(create_easing_curves);
        curves
            .get(&easing_type)
            .expect("All easing types should be initialized")
    }

    /// Get all available easing curves
    pub fn all_easing_curves() -> &'static HashMap<EasingType, EasingCurve> {
        EASING_CURVES.get_or_init(create_easing_curves)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_linear() {
        let linear = CubicBezier::new(0.0, 0.0, 1.0, 1.0);

        assert!((linear.sample(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((linear.sample(1.0) - 1.0).abs() < f32::EPSILON);
        assert!((linear.sample(0.5) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_cubic_bezier_standard() {
        let standard = CubicBezier::new(0.2, 0.0, 0.0, 1.0);

        // Should start at 0 and end at 1
        assert!((standard.sample(0.0) - 0.0).abs() < f32::EPSILON);
        assert!((standard.sample(1.0) - 1.0).abs() < f32::EPSILON);

        // Sample points along the curve
        let t0 = standard.sample(0.0);
        let t1 = standard.sample(0.1);
        let t2 = standard.sample(0.2);
        let t3 = standard.sample(0.3);
        let t4 = standard.sample(0.4);
        let t5 = standard.sample(0.5);
        let t6 = standard.sample(0.6);
        let t7 = standard.sample(0.7);
        let t8 = standard.sample(0.8);
        let t9 = standard.sample(0.9);
        let t10 = standard.sample(1.0);

        // Print the curve values for debugging
        println!("Standard Easing Curve Values:");
        println!("t=0.0 -> {}", t0);
        println!("t=0.1 -> {}", t1);
        println!("t=0.2 -> {}", t2);
        println!("t=0.3 -> {}", t3);
        println!("t=0.4 -> {}", t4);
        println!("t=0.5 -> {}", t5);
        println!("t=0.6 -> {}", t6);
        println!("t=0.7 -> {}", t7);
        println!("t=0.8 -> {}", t8);
        println!("t=0.9 -> {}", t9);
        println!("t=1.0 -> {}", t10);

        // Verify the curve starts at 0 and ends at 1
        assert_eq!(t0, 0.0, "Curve should start at 0.0");
        assert_eq!(t10, 1.0, "Curve should end at 1.0");

        // Verify the curve is strictly increasing
        let values = [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10];
        for i in 1..values.len() {
            assert!(
                values[i - 1] <= values[i],
                "Curve should be monotonically increasing at t={}, prev={}, current={}",
                i as f32 / 10.0,
                values[i - 1],
                values[i]
            );
        }

        // For this specific curve (0.2, 0.0, 0.0, 1.0), we know the exact behavior:
        // - It starts slow (ease-in) and then accelerates quickly
        // - At t=0.2, the value is exactly 0.5 (halfway through the animation)
        // - After t=0.2, it approaches 1.0 quickly
        assert!(
            (t2 - 0.5).abs() < 0.01,
            "At t=0.2, the value should be approximately 0.5"
        );

        // The first half of the animation (t=0.0 to t=0.2) should cover the first 50% of progress
        assert!(t1 > 0.1, "At t=0.1, progress should be more than 10%");
        assert!(t3 > 0.6, "At t=0.3, progress should be more than 60%");
        assert!(t4 > 0.8, "At t=0.4, progress should be more than 80%");
        assert!(t5 > 0.87, "At t=0.5, progress should be more than 87%");
        assert!(t6 > 0.92, "At t=0.6, progress should be more than 92%");
        assert!(t7 > 0.96, "At t=0.7, progress should be more than 96%");
        assert!(t8 > 0.98, "At t=0.8, progress should be more than 98%");
        assert!(t9 > 0.99, "At t=0.9, progress should be more than 99%");
    }

    #[test]
    fn test_easing_curve_access() {
        let standard = super::super::MotionTokens::easing(EasingType::Standard);
        assert_eq!(standard.name, "standard");

        let linear = super::super::MotionTokens::easing(EasingType::Linear);
        assert_eq!(linear.name, "linear");
    }

    #[test]
    fn test_all_easing_types_available() {
        let all_curves = super::super::MotionTokens::all_easing_curves();

        // Test that all easing types are available
        assert!(all_curves.contains_key(&EasingType::Linear));
        assert!(all_curves.contains_key(&EasingType::Standard));
        assert!(all_curves.contains_key(&EasingType::StandardAccelerate));
        assert!(all_curves.contains_key(&EasingType::StandardDecelerate));
        assert!(all_curves.contains_key(&EasingType::Emphasized));
        assert!(all_curves.contains_key(&EasingType::EmphasizedAccelerate));
        assert!(all_curves.contains_key(&EasingType::EmphasizedDecelerate));
        assert!(all_curves.contains_key(&EasingType::Legacy));
        assert!(all_curves.contains_key(&EasingType::LegacyAccelerate));
        assert!(all_curves.contains_key(&EasingType::LegacyDecelerate));
    }

    #[test]
    fn test_css_output() {
        let curve = CubicBezier::new(0.2, 0.0, 0.0, 1.0);
        assert_eq!(curve.to_css(), "cubic-bezier(0.2, 0, 0, 1)");
    }

    #[test]
    fn test_bezier_monotonicity() {
        // Test that cubic bezier curves with valid control points are monotonic
        let curves = [
            CubicBezier::new(0.2, 0.0, 0.0, 1.0), // standard
            CubicBezier::new(0.3, 0.0, 1.0, 1.0), // standard-accelerate
            CubicBezier::new(0.4, 0.0, 0.2, 1.0), // legacy
        ];

        for curve in &curves {
            let mut prev = 0.0;
            for i in 1..=10 {
                let t = i as f32 / 10.0;
                let value = curve.sample(t);
                assert!(value >= prev, "Curve should be monotonic");
                prev = value;
            }
        }
    }

    #[test]
    fn test_newton_raphson_accuracy() {
        let curve = CubicBezier::new(0.2, 0.0, 0.0, 1.0);

        // Test that Newton-Raphson gives accurate results
        for i in 0..=10 {
            let x = i as f32 / 10.0;
            let t = curve.solve_t_for_x(x);
            let x_calc = curve.sample_x(t);

            assert!(
                (x_calc - x).abs() < 1e-5,
                "Newton-Raphson should be accurate: expected {x}, got {x_calc}"
            );
        }
    }
}
