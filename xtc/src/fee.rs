/// Compute the fee that should be taken for a transaction based on the total amount
/// of the transaction.
/// Any implementation of this method should guarantee that for an amount A > B, the
/// `compute_fee(A) >= compute_fee(B)` holds true. Unless the code across the project
/// is also changed to take that into account.
#[inline(always)]
pub fn compute_fee(_: u64) -> u64 {
    2_000_000_000
}