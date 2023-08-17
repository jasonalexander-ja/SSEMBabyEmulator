//! # Errors
//! 
//! This module includes all the possible errors that can be thrown 
//! during emulation execution, all error types implement [LinkerError][crate::core::errors::BabyError].
//! 
//! The main type in this module is [BabyErrors][crate::core::errors::BabyErrors]
//! whcih is an enum that has a branch for each possible error that can be thrown 
//! with each branch wrapping the corresponding error type that contains metadata 
//! on that error. 
//! 
//! Any new error types and object should be added to this enum. 
//! 


use crate::core::instructions::BabyInstruction;

/// Defines standard behaviour for any thrown errors. 
pub trait BabyError: Clone + Copy {
    /// Gets a string describing the error. 
    fn get_descriptor(&self) -> String;
    /// Gets the instruction being executed when the error was thrown. 
    fn get_instruction(&self) -> BabyInstruction;
    /// Gets the memory address of the instruction being exected when the error was thrown. 
    fn at(&self) -> u16;
}

/// An enum containing potential errors allowing them to be handled. 
#[derive(Clone, Copy)]
pub enum BabyErrors {
    /// The emulator has encountered a stop instruction.  
    Stop(Stop)
}

impl BabyErrors {

    /// Gets the inner error. 
    /// 
    /// This method gets the inner [BabyError] derived error allowing for 
    /// immediate access to the root errors methods detailing the errors 
    /// description and metadata. 
    /// 
    pub fn get_baby_error(&self) -> &impl BabyError {
        match self {
            BabyErrors::Stop(s) => s
        }
    }
}

impl BabyError for BabyErrors {

    fn get_descriptor(&self) -> String {
        self.get_baby_error().get_descriptor()
    }

    fn get_instruction(&self) -> BabyInstruction {
        self.get_baby_error().get_instruction()
    }

    fn at(&self) -> u16 {
        self.get_baby_error().at()
    }
}

/// An error thrown when the emulator encounters a stop instruction. 
/// 
/// Contains the position in memory where it was encountered, 
/// this is used as an error so that when the emulator is ran in a loop 
/// any errors can break out of that loop and then checked programatically 
/// by the calling program, and appropriate action taken.
/// 
/// # Example 
/// ```
/// use baby_emulator::core::BabyModel;
/// use baby_emulator::errors::BabyError;
/// 
/// let mut model = BabyModel::new_example_program();
/// loop {
///     model = match model.execute() {
///         Ok(m) => m,
///         Err(e) => {
///             println!("{}", e.get_descriptor());
///             break
///         }
///     }
/// }
/// println!("{}", model.core_dump());
/// ```
#[derive(Clone, Copy)]
pub struct Stop {
    /// The memory address where the stop was encountered. 
    pub at: u16
}

impl BabyError for Stop {
    fn get_descriptor(&self) -> String {
        format!("Program stop instruction encountered at {:#06x}; \n", self.at)
    }
    
    fn get_instruction(&self) -> BabyInstruction {
        BabyInstruction::Stop
    }

    fn at(&self) -> u16 {
        self.at
    }
}
