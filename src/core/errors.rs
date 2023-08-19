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
use crate::core::BabyModel;

/// Defines standard behaviour for any thrown errors. 
pub trait BabyError: Clone {
    /// Gets a string describing the error. 
    fn get_descriptor(&self) -> String;
    /// Gets the instruction being executed when the error was thrown. 
    fn get_instruction(&self) -> BabyInstruction;
    /// Gets the memory address of the instruction being exected when the error was thrown. 
    fn at(&self) -> u16;
}

/// An enum containing potential errors allowing them to be handled. 
#[derive(Clone)]
pub enum BabyErrors {
    /// The emulator has encountered a stop instruction.  
    Stop(Stop),
    /// The emulator has hit the maximum number of iterations. 
    IterationExceeded(IterationsExceeded)
}

impl BabyError for BabyErrors {

    fn get_descriptor(&self) -> String {
        match self {
            BabyErrors::Stop(s) => s.get_descriptor(),
            BabyErrors::IterationExceeded(s) => s.get_descriptor()
        }
    }

    fn get_instruction(&self) -> BabyInstruction {
        match self {
            BabyErrors::Stop(s) => s.get_instruction(),
            BabyErrors::IterationExceeded(s) => s.get_instruction()
        }
    }

    fn at(&self) -> u16 {
        match self {
            BabyErrors::Stop(s) => s.at(),
            BabyErrors::IterationExceeded(s) => s.at()
        }
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
/// use baby_emulator::core::errors::BabyError;
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

/// An error thrown when the emulator execution iteration loop exceeds
/// the maximum specified number of iterations. 
/// 
/// Contains the state of the model when the iteration was exceeded. 
/// 
/// # Example 
/// ```
/// use baby_emulator::core::BabyModel;
/// use baby_emulator::core::errors::BabyErrors;
/// use baby_emulator::core::errors::BabyError;
/// 
/// let model = BabyModel::new_example_program();
/// match model.run_loop(100) {
///     (model, BabyErrors::Stop(_)) => println!("{}", model.core_dump()),
///     (_, BabyErrors::IterationExceeded(err)) => 
///         println!("Iterations exceeded {}", err.get_descriptor()),
///     (_, err) => println!("{}", err.get_descriptor()),
/// }
/// ```
/// 
#[derive(Clone)]
pub struct IterationsExceeded {
    /// The maximum number of specified iterations. 
    pub max_iter: usize,
    /// The state of the model when iterations were exceeded. 
    pub end_model: BabyModel,
}

impl IterationsExceeded {
    pub fn new(max_iter: usize, end_model: BabyModel) -> IterationsExceeded {
        IterationsExceeded { max_iter, end_model }
    }
}

impl BabyError for IterationsExceeded {
    fn get_descriptor(&self) -> String {
        format!("Emulation execution iterations hit limmit of {}. \n", self.max_iter)
    }
    
    fn get_instruction(&self) -> BabyInstruction {
        BabyInstruction::from_number(self.end_model.instruction)
    }

    fn at(&self) -> u16 {
        self.end_model.instruction_address
    }
}
