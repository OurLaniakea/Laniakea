use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use std::convert::TryInto;

// Program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("GNN Program Entrypoint");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    match instruction_data[0] {
        0 => {
            msg!("Instruction: Initialize Neural Network");
            process_initialize_network(accounts, &instruction_data[1..])?;
        }
        1 => {
            msg!("Instruction: Update Model Parameters");
            process_update_parameters(accounts, &instruction_data[1..])?;
        }
        2 => {
            msg!("Instruction: Execute Neural Computation");
            process_neural_computation(accounts, &instruction_data[1..])?;
        }
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}

fn process_initialize_network(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Processing Initialize Network");
    
    let accounts_iter = &mut accounts.iter();
    let network_account = next_account_info(accounts_iter)?;
    let authority_account = next_account_info(accounts_iter)?;

    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Initialize network parameters
    let mut network_data = NetworkState::unpack_from_slice(&network_account.data.borrow())?;
    network_data.is_initialized = true;
    network_data.authority = *authority_account.key;
    network_data.model_version = 1;
    network_data.last_update = solana_program::clock::Clock::get()?.unix_timestamp;

    NetworkState::pack_into_slice(&network_data, &mut network_account.data.borrow_mut());

    msg!("Neural Network initialized successfully");
    Ok(())
}

fn process_update_parameters(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Processing Update Parameters");
    
    let accounts_iter = &mut accounts.iter();
    let network_account = next_account_info(accounts_iter)?;
    let authority_account = next_account_info(accounts_iter)?;

    if !authority_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut network_data = NetworkState::unpack_from_slice(&network_account.data.borrow())?;
    
    // Update network parameters
    network_data.model_version += 1;
    network_data.last_update = solana_program::clock::Clock::get()?.unix_timestamp;

    NetworkState::pack_into_slice(&network_data, &mut network_account.data.borrow_mut());

    msg!("Parameters updated successfully");
    Ok(())
}

fn process_neural_computation(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Processing Neural Computation");
    
    let accounts_iter = &mut accounts.iter();
    let network_account = next_account_info(accounts_iter)?;
    let computation_account = next_account_info(accounts_iter)?;

    let network_data = NetworkState::unpack_from_slice(&network_account.data.borrow())?;
    
    // Perform neural computation
    // This is where the actual AI/ML computation would happen
    msg!("Neural computation executed successfully");
    Ok(())
}

#[derive(Debug)]
pub struct NetworkState {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub model_version: u64,
    pub last_update: i64,
}

impl NetworkState {
    pub fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 50];
        Ok(Self {
            is_initialized: src[0] != 0,
            authority: Pubkey::new_from_array(*array_ref![src, 1, 32]),
            model_version: u64::from_le_bytes(*array_ref![src, 33, 8]),
            last_update: i64::from_le_bytes(*array_ref![src, 41, 8]),
        })
    }

    pub fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 50];
        let (is_initialized_dst, authority_dst, model_version_dst, last_update_dst) =
            mut_array_refs![dst, 1, 32, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        model_version_dst.copy_from_slice(&self.model_version.to_le_bytes());
        last_update_dst.copy_from_slice(&self.last_update.to_le_bytes());
    }
} 