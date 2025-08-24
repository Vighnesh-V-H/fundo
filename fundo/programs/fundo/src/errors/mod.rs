use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("The program has already been initialied.")]
    ProgramAlreadyInitialized,
    #[msg("Title exceeds the maximum length of 64 characters.")]
    MaxTitleLengthExceeded,
    #[msg("Description exceeds the maximum length of 512 characters.")]
    MaxDescriptionLengthExceeded,
    #[msg("Image URL exceeds the maximum length of 200 characters.")]
    MaxImageUrlLengthExceeded,
    #[msg("Invalid goal amount. Goal must be greater than zero.")]
    InvalidGoalAmount,
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Campaign not found. Please check the id once!")]
    CampaignNotFound,
    #[msg("Campaign is inactive.")]
    InactiveCampaign,
    #[msg("Donation amount must be at least 1 SOL.")]
    InvalidDonationAmount,
    #[msg("Campaign goal reached.")]
    CampaignGoalActualized,
    #[msg("Withdrawal amount must be at least 1 SOL.")]
    InvalidWithdrawalAmount,
    #[msg("Insufficient funds in the campaign.")]
    InsufficientFund,
    #[msg("The provided platform address is invalid.")]
    InvalidPlatformAddress,
    #[msg("Invalid platform fee percentage.")]
    InvalidPlatformFee,
}