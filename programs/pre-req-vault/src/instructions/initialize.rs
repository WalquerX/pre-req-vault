use crate::state::VaultState;
use anchor_lang::prelude::*;

// Signer, Account, SystemAccount, Program are checking conditions.
// Signer checks user signed the tx. Then we can use it as payer for paying for the space vault_state will need.
// Accounts checks the owner is the Program, and the account does not exist.
// SystemAccount checks the owner is the SystemProgram.
// Program checks it is an executable and that the key equals the System Program ID
//
// We have vault_state derived from user key and vault derived from vault_state key:
//      user key -> vault_state key -> vault key.
// Then, there is an instance of vault_state and vault_key for each user. We have vault state and vault balance per user.
// Vault key is derived from vault_state key so to link/chain the derivation and setting the relation between those accounts in its keys
// rather than only in code checks.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // Since vault_state holds data, it need to be initialized, then someone has to pay for its rent.
    #[account(
        init,
        payer = user,
        // seeds have type &[&[u8]], then we use as_ref() on user.key()
        seeds = [b"state", user.key().as_ref()],
        bump,
        // VaultState have InitSpace trait derived on it, then we can call INIT_SPACE. The macro is making the calculation for us.
        // In previous anchor versiones that was introduced manually.
        // The discriminator space if probably not included because that represent a different design space decision, not part of the current
        // program but from anchor (I think it should not be here neither, I do have the same feeling for 'info). 
        space = 8 + VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    // vault does not hold data, and vault is not created by this instruction, then it does not need a payer.
    // vault will only hold lamports. And will be born when receiving its first transference.
    // Notice: The address where the program is deployed can hold lamports but you will not be able to move it from it, since the owner of that account
    // is the system (BPF loader). 
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // We are creating account then we need to list also the system program.
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    // bumps are not passed by users, then we get them from the context when wrapping it at lib.rs.
    pub fn intialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Save data to state
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
