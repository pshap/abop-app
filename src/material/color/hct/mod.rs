//! HCT (Hue-Chroma-Tone) color space implementation
//!
//! This module provides an implementation of the HCT color space used internally by
//! Material Design 3 for color manipulation and theming.
//!
//! The HCT color space is a perceptually accurate color space that separates color
//! into three components:
//! - Hue: The color's position on the color wheel (0-360 degrees)
//! - Chroma: The color's intensity or saturation (0-100+)
//! - Tone: The color's lightness (0-100)

use std::f64::consts::*;
use std::ops::{Add, Mul, Sub};

use super::Srgb;

/// A color in the HCT color space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hct {
    /// Hue in degrees (0-360)
    pub hue: f64,
    /// Chroma (0-100+)
    pub chroma: f64,
    /// Tone (0-100)
    pub tone: f64,
}

impl Hct {
    /// Create a new HCT color
    pub fn new(hue: f64, chroma: f64, tone: f64) -> Self {
        Self {
            hue: hue.rem_euclid(360.0),
            chroma: chroma.max(0.0),
            tone: tone.clamp(0.0, 100.0),
        }
    }
    
    /// Create an HCT color from sRGB values
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        // Convert to linear RGB
        let r = srgb_to_linear(r);
        let g = srgb_to_linear(g);
        let b = srgb_to_linear(b);
        
        // Convert to XYZ
        let x = 0.41233895 * r + 0.35762064 * g + 0.18051042 * b;
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let z = 0.01932141 * r + 0.11916382 * g + 0.95034478 * b;
        
        // Convert to CAM16
        let cam = Cam16::from_xyz(x, y, z);
        
        // Convert to HCT
        Self {
            hue: cam.h,
            chroma: cam.c,
            tone: y_to_lstar(y * 100.0) as f64,
        }
    }
    
    /// Convert to sRGB
    pub fn to_srgb(&self) -> Srgb {
        // Convert HCT to CAM16
        let cam = Cam16 {
            h: self.hue,
            c: self.chroma,
            j: self.tone,
            q: 0.0, // Will be calculated
            m: 0.0, // Will be calculated
            s: 0.0, // Will be calculated
        };
        
        // Convert CAM16 to XYZ
        let (x, y, z) = cam.to_xyz();
        
        // Convert XYZ to linear RGB
        let r = 3.2406 * x - 1.5372 * y - 0.4986 * z;
        let g = -0.9689 * x + 1.8758 * y + 0.0415 * z;
        let b = 0.0557 * x - 0.2040 * y + 1.0570 * z;
        
        // Convert to sRGB
        Srgb::new(
            linear_to_srgb(r) as f32,
            linear_to_srgb(g) as f32,
            linear_to_srgb(b) as f32,
        )
    }
    
    /// Get a color with the same hue and chroma but a different tone
    pub fn with_tone(&self, tone: f64) -> Self {
        Self {
            tone: tone.clamp(0.0, 100.0),
            ..*self
        }
    }
    
    /// Get a color with the same hue and tone but a different chroma
    pub fn with_chroma(&self, chroma: f64) -> Self {
        Self {
            chroma: chroma.max(0.0),
            ..*self
        }
    }
    
    /// Get a color with the same chroma and tone but a different hue
    pub fn with_hue(&self, hue: f64) -> Self {
        Self {
            hue: hue.rem_euclid(360.0),
            ..*self
        }
    }
}

impl From<Srgb> for Hct {
    fn from(srgb: Srgb) -> Self {
        Self::from_rgb(
            srgb.r as f64,
            srgb.g as f64,
            srgb.b as f64,
        )
    }
}

impl From<Hct> for Srgb {
    fn from(hct: Hct) -> Self {
        hct.to_srgb()
    }
}

// CAM16 color appearance model
#[derive(Debug, Clone, Copy)]
struct Cam16 {
    // Hue
    h: f64,
    // Chroma
    c: f64,
    // Lightness
    j: f64,
    // Brightness
    q: f64,
    // Colorfulness
    m: f64,
    // Saturation
    s: f64,
}

impl Cam16 {
    // Convert from XYZ to CAM16
    fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        // Simplified implementation - full CAM16 is more complex
        let (l, a, b) = xyz_to_lab(x, y, z);
        let c = (a * a + b * b).sqrt();
        let h = b.atan2(a).to_degrees();
        
        // Approximate CAM16 from Lab
        Self {
            h: h.rem_euclid(360.0),
            c,
            j: l,
            q: 0.0,
            m: 0.0,
            s: 0.0,
        }
    }
    
    // Convert from CAM16 to XYZ
    fn to_xyz(&self) -> (f64, f64, f64) {
        // Simplified implementation - full CAM16 is more complex
        let h_rad = self.h.to_radians();
        let a = self.c * h_rad.cos();
        let b = self.c * h_rad.sin();
        
        // Approximate Lab from CAM16
        let l = self.j;
        
        // Convert Lab to XYZ
        lab_to_xyz(l, a, b)
    }
}

// Helper functions for color space conversions

fn srgb_to_linear(srgb: f64) -> f64 {
    if srgb <= 0.04045 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(linear: f64) -> f64 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

fn xyz_to_lab(x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    // D65 white point
    const REF_X: f64 = 0.95047;
    const REF_Y: f64 = 1.0;
    const REF_Z: f64 = 1.08883;
    
    let x = x / REF_X;
    let y = y / REF_Y;
    let z = z / REF_Z;
    
    let f = |t: f64| -> f64 {
        const EPSILON: f64 = 216.0 / 24389.0;
        const KAPPA: f64 = 24389.0 / 27.0;
        
        if t > EPSILON {
            t.cbrt()
        } else {
            (KAPPA * t + 16.0) / 116.0
        }
    };
    
    let fx = f(x);
    let fy = f(y);
    let fz = f(z);
    
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    
    (l, a, b)
}

fn lab_to_xyz(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    // D65 white point
    const REF_X: f64 = 0.95047;
    const REF_Y: f64 = 1.0;
    const REF_Z: f64 = 1.08883;
    
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;
    
    let xr = if fx > 0.206896552 {
        fx.powi(3)
    } else {
        (fx - 16.0 / 116.0) / 7.787037
    } * REF_X;
    
    let yr = if l > 7.999625 {
        ((l + 16.0) / 116.0).powi(3) * REF_Y
    } else {
        l * REF_Y / 903.3
    };
    
    let zr = if fz > 0.206896552 {
        fz.powi(3)
    } else {
        (fz - 16.0 / 116.0) / 7.787037
    } * REF_Z;
    
    (xr, yr, zr)
}

// Convert from Y in XYZ to L* in L*a*b*
fn y_to_lstar(y: f64) -> f64 {
    const EPSILON: f64 = 216.0 / 24389.0;
    const KAPPA: f64 = 24389.0 / 27.0;
    
    if y <= EPSILON * 100.0 {
        y * KAPPA / 100.0
    } else {
        116.0 * (y / 100.0).cbrt() - 16.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;
    
    #[test]
    fn test_hct_creation() {
        let hct = Hct::new(120.0, 50.0, 50.0);
        
        assert!(approx_eq!(f64, hct.hue, 120.0, epsilon = 0.001));
        assert!(approx_eq!(f64, hct.chroma, 50.0, epsilon = 0.001));
        assert!(approx_eq!(f64, hct.tone, 50.0, epsilon = 0.001));
        
        // Test hue wrapping
        let hct = Hct::new(480.0, 50.0, 50.0);
        assert!(approx_eq!(f64, hct.hue, 120.0, epsilon = 0.001));
        
        // Test chroma clamping
        let hct = Hct::new(0.0, -10.0, 50.0);
        assert!(hct.chroma >= 0.0);
        
        // Test tone clamping
        let hct = Hct::new(0.0, 50.0, 150.0);
        assert!(hct.tone <= 100.0);
    }
    
    #[test]
    fn test_hct_to_srgb() {
        // Test black
        let black = Hct::new(0.0, 0.0, 0.0).to_srgb();
        assert!(black.r == 0.0 && black.g == 0.0 && black.b == 0.0);
        
        // Test white
        let white = Hct::new(0.0, 0.0, 100.0).to_srgb();
        assert!(white.r >= 0.99 && white.g >= 0.99 && white.b >= 0.99);
        
        // Test red
        let red = Hct::new(0.0, 100.0, 50.0).to_srgb();
        assert!(red.r > red.g && red.r > red.b);
    }
    
    #[test]
    fn test_srgb_to_hct() {
        // Test black
        let black = Hct::from(Srgb::new(0.0, 0.0, 0.0));
        assert!(black.tone < 0.1);
        
        // Test white
        let white = Hct::from(Srgb::new(1.0, 1.0, 1.0));
        assert!(white.tone > 99.9);
        
        // Test red
        let red = Hct::from(Srgb::new(1.0, 0.0, 0.0));
        assert!((red.hue < 30.0) || (red.hue > 330.0)); // Red is at 0°
        assert!(red.chroma > 50.0);
    }
}
