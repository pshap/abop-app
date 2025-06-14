//! Database-specific conversion utilities

use super::super::error::DomainCastError;
use log::warn;

/// Convert database count (i64) to collection size (usize) safely
///
/// This function handles negative values, overflow conditions, and
/// platform differences between 32-bit and 64-bit systems.
///
/// # Platform Safety
/// On 32-bit platforms, `usize` is 32 bits, so we need to be careful
/// with large i64 values that might come from the database.
#[must_use] pub fn safe_db_count_to_usize(count: i64) -> usize {
    if count < 0 {
        warn!("Negative database count: {count}, using 0");
        return 0;
    }

    if cfg!(target_pointer_width = "32") && count > (usize::MAX as i64) {
        warn!(
            "Database count {count} exceeds usize::MAX ({}), clamping",
            usize::MAX
        );
        return usize::MAX;
    }

    count as usize
}

/// Validate database count against usize limits with error handling
///
/// # Errors
/// Returns domain-specific database errors
pub fn validate_db_count(count: i64) -> Result<usize, DomainCastError> {
    use crate::utils::casting::error::domain::DatabaseCastError;

    if count < 0 {
        return Err(DatabaseCastError::CountOutOfRange(count).into());
    }

    if !can_fit_in_usize(count) {
        return Err(DatabaseCastError::ValueTooLarge(count).into());
    }

    Ok(count as usize)
}

/// Check if an i64 database count can safely fit in usize
#[inline]
#[must_use] pub const fn can_fit_in_usize(count: i64) -> bool {
    // Check if count is non-negative and within the safe range
    // Since i64::MAX is always <= usize::MAX on 64-bit systems,
    // and usize::MAX can overflow when cast to i64, we compare differently
    if count < 0 {
        return false;
    }

    // For positive values, if count fits in i64, it will fit in usize on 64-bit
    // For 32-bit systems, we need to check against usize::MAX
    if cfg!(target_pointer_width = "32") {
        (count as u64) <= (usize::MAX as u64)
    } else {
        // On 64-bit, any positive i64 fits in usize
        true
    }
}

/// Convert usize to i64 safely for database storage
///
/// # Errors
/// Returns domain-specific database errors
pub fn safe_usize_to_i64(value: usize) -> Result<i64, DomainCastError> {
    use crate::utils::casting::error::domain::DatabaseCastError;

    if value > (i64::MAX as usize) {
        return Err(DatabaseCastError::ValueTooLarge(i64::MAX).into());
    }

    Ok(value as i64)
}

/// Platform-aware maximum safe database count
#[inline]
#[must_use] pub const fn max_safe_db_count() -> i64 {
    if cfg!(target_pointer_width = "64") {
        i64::MAX
    } else {
        // On 32-bit platforms, we're limited by usize::MAX
        usize::MAX as i64
    }
}

/// Convenience wrapper for validate_db_count
///
/// # Errors
/// Returns domain-specific database errors for invalid counts
pub fn count_to_usize(count: i64) -> Result<usize, DomainCastError> {
    validate_db_count(count)
}

/// Convenience wrapper for safe_usize_to_i64
///
/// # Errors
/// Returns domain-specific database errors for invalid conversions
pub fn size_to_i64(size: usize) -> Result<i64, DomainCastError> {
    safe_usize_to_i64(size)
}

/// Convenience wrapper for safe_db_count_to_usize
#[must_use]
pub fn count_to_size(count: i64) -> usize {
    safe_db_count_to_usize(count)
}
