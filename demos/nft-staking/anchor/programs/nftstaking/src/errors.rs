use anchor_lang::error_code;

#[error_code]
pub enum StakeProgramError {
    #[msg("Freeze period not passed")]
    FreezePeriodNotPassed,
    #[msg("Max stake reached")]
    MaxStakeReached,
}