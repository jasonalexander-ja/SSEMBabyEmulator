//! # Core emulation utilities 
//! 
//! This module contains the emulator itself, this is in the form of
//! [BabyModel][crate::core::BabyModel] has fields corresponding to each 
//! register and memory location as what was on the original Manc Baby,
//! this also has several methods for running a debugging the model. 
//! 
//! ## Instantiating  
//! The baby model has a new class that instantiates a completely blank 
//! model with all of its fields set to zero, if this was run, it would 
//! continuously perform jump instructions back to memory address 0. 
//! 
//! ```
//! use baby_emulator::core::BabyModel;
//! let model = BabyModel::new();
//! ```
//! 
//! There are 2 ways to make a real runnable instances of the model,
//! the baby model upon creation will load the first instruction as 
//! from the supplied memory (or the main store) and will continue 
//! fetching instructions from here, so you can instiantiate a model 
//! with a memory loaded with your own program code. 
//! 
//! You can use [BabyInstruction][crate::core::instructions::BabyInstruction] and
//! [BabyInstruction::to_numbers][crate::core::instructions::BabyInstruction::to_numbers] 
//! to more easily generate a `[u32, 32]` program stack. 
//! 
//! ```
//! use baby_emulator::core::BabyModel;
//! use baby_emulator::core::instructions::BabyInstruction;
//! 
//! let instrs = vec![
//!     (BabyInstruction::Negate, 5),
//!     (BabyInstruction::Subtract, 5),
//!     (BabyInstruction::Store, 6),
//!     (BabyInstruction::Negate, 6),
//!     (BabyInstruction::Stop, 0),
//!     (BabyInstruction::AbsoluteValue(5), 0),
//! ];
//! let main_store = BabyInstruction::to_numbers(instrs);
//! let model = BabyModel::new_with_program(main_store);
//! ``` 
//! 
//! The other way is for quick demonstrations and that is to use 
//! [BabyModel::new_example_program][crate::core::BabyModel::new_example_program]. 
//! 
//! ## Running 
//! There are 2 methods used to run a model, each will execute one 
//! instruction at a time, calculate how the instruction will modify 
//! the models fields (including fetching the next instruction to 
//! the instruction register), and use this to generate and return a new model.
//! 
//! You can manually dispatch an instruction to the model by using one of 
//! the following methods, this is useful to see what each command does to the model:
//! 
//! * [BabyModel::jump][crate::core::BabyModel::jump]
//! * [BabyModel::relative_jump][crate::core::BabyModel::relative_jump]
//! * [BabyModel::negate][crate::core::BabyModel::negate]
//! * [BabyModel::store][crate::core::BabyModel::store]
//! * [BabyModel::subtract][crate::core::BabyModel::subtract]
//! * [BabyModel::test][crate::core::BabyModel::test]
//! 
//! You can also use [BabyModel::execute][crate::core::BabyModel::execute]
//! this will execute the next commands loaded from the memory, returning 
//! [InstrResult][crate::core::InstrResult] that will either be the 
//! new model, or a [BabyErrors][crate::core::errors::BabyErrors] detailing the 
//! error encountered (this can be simply encountering a stop command). 
//! 
//! ```
//! use baby_emulator::core::BabyModel;
//! use baby_emulator::errors::BabyError;
//! 
//! let model = BabyModel::new_example_program();
//! match model.execute() {
//!     Ok(m) => println!("{}", m.core_dump()),
//!     Err(e) => println!("Error {}", e.get_descriptor())
//! }
//! ```
//! 
//! To run a model continuously until an error is encountered, you can 
//! use a for let loop. 
//! ```
//! use baby_emulator::core::BabyModel;
//! use baby_emulator::errors::BabyError;
//! 
//! let mut model = BabyModel::new_example_program();
//! loop {
//!     model = match model.execute() {
//!         Ok(m) => m,
//!         Err(_) => break
//!     }
//! }
//! println!("{}", model.core_dump());
//! ```
//! 

use std::ops::Neg;
use errors::Stop;
use errors::BabyErrors;
use instructions::BabyInstruction;


/// Contains potential errors thrown during emulation. 
pub mod errors;
/// Contains models and functionality for decoding instructions. 
pub mod instructions;

/// The number of words in the memory used globally.  
pub const MEMORY_WORDS: usize = 32;

/// A result from [BabyModel] executing an instruction. 
/// 
/// Just a [Result] type, which is either a [BabyModel] of the updated model
/// post executing an instruction or a [BabyErrors] containing details of an error. 
/// 
/// # Example
/// ```
/// use baby_emulator::core::BabyModel;
/// use baby_emulator::errors::BabyError;
/// 
/// let model = BabyModel::new();
/// match model.execute() {
///     Ok(model) => println!("{}", model.core_dump()),
///     Err(error) => println!("{}", error.get_descriptor())
/// }
/// ```
pub type InstrResult = Result<BabyModel, BabyErrors>;

/// The model containing the data in all the registers and memory to be operated upon. 
#[derive(Clone)]
pub struct BabyModel {
    /// The memory (RAM), this is just 32 words of 32 bits, 
    /// originally famously stored on a Williams Tube.  
    pub main_store: [i32; MEMORY_WORDS],
    /// The register where all mathematical results 
    /// are stored (negations and subtractions). 
    pub accumulator: i32,
    /// The memory address of the instruction currently 
    /// being executed (program counter). 
    pub instruction_address: u16,
    /// The 16 bit instruction being executed (instruction register). 
    pub instruction: u16,
}

impl BabyModel {

    /// Creates a new model with all zeros. 
    pub fn new() -> BabyModel {
        BabyModel {
            main_store: [0; MEMORY_WORDS],
            accumulator: 0,
            instruction_address: 0,
            instruction: 0,
        }
    }

    /// Creates a new model with a specified memory. 
    /// 
    /// Initialised as to start executing at the start of the memory, 
    /// specified memory can contain program code to be executed.  
    /// 
    /// # Parameters 
    /// 
    /// * `main_store` - The custom memory to be initialised with. 
    /// 
    pub fn new_with_program(main_store: [i32; MEMORY_WORDS]) -> BabyModel {
        BabyModel { 
            main_store,
            accumulator: 0,
            instruction_address: 0,
            instruction: main_store[0] as u16
        }
    }

    /// Creates a new model with an example program loaded into memory. 
    /// 
    /// This program will add 5 to 5, storing the result in the 
    /// accumulator and end. 
    /// 
    /// # Example 
    /// ```
    /// use baby_emulator::core::BabyModel;
    /// 
    /// let mut model = BabyModel::new_example_program();
    /// loop {
    ///     model = match model.execute() {
    ///         Ok(m) => m,
    ///         Err(_) => break
    ///     }
    /// }
    /// println!("{}", model.core_dump());
    /// ```
    /// 
    pub fn new_example_program() -> BabyModel {
        let instrs = vec![
            BabyInstruction::Negate(5),
            BabyInstruction::Subtract(5),
            BabyInstruction::Store(6),
            BabyInstruction::Negate(6),
            BabyInstruction::Stop,
            BabyInstruction::AbsoluteValue(5),
        ];
        let main_store = BabyInstruction::to_numbers(instrs);

        BabyModel {
            main_store,
            accumulator: 0,
            instruction_address: 0,
            instruction: main_store[0] as u16,
        }
    }

    /// Executes the instruction in the instruction register. 
    /// 
    /// Decodes the instruction value in the instruction register and performs 
    /// the relevant operation on the data within the model, will return all the
    /// updated data in a new [Ok(BabyModel)] assuming no errors encountered.  
    /// 
    /// # Returns 
    /// - `Ok(InstrResult)`: A new model instance with all data updated as per 
    ///     the instruction, loaded with the next instruction. 
    /// - `Err(BabyErrors)`: An enum detailing errors encountered when 
    ///     executing the instruction. 
    /// 
    /// # Example 
    /// ```
    /// use baby_emulator::core::BabyModel;
    /// use baby_emulator::errors::{BabyError, BabyErrors};
    /// 
    /// fn run_model(model: BabyModel) {
    ///     let mut result = model.execute();
    ///     while let Ok(new_model) = result {
    ///         result = new_model.execute();
    ///     }
    ///     match result {
    ///         Err(BabyErrors::Stop(_)) => println!("Sucess! "),
    ///         _ => println!("Something went wrong. ")
    ///     }
    /// }
    /// ```
    /// 
    pub fn execute(&self) -> InstrResult {
        let instruction = BabyInstruction::from_number(self.instruction);
        let operand = instruction.get_operand();
        let operand_value = self.main_store[operand];

        self.dispatch_instruction(instruction, operand_value)
    }

    fn dispatch_instruction(&self, instruction: BabyInstruction, operand_value: i32) -> InstrResult {
        let res = match instruction {
            BabyInstruction::Jump(_) => self.jump(operand_value),
            BabyInstruction::RelativeJump(_) => self.relative_jump(operand_value),
            BabyInstruction::Negate(_) => self.negate(operand_value),
            BabyInstruction::Store(_) => self.store(operand_value),
            BabyInstruction::Subtract(_) => self.subtract(operand_value),
            BabyInstruction::SkipNextIfNegative => self.test(),
            BabyInstruction::Stop => return Err(BabyErrors::Stop(Stop {
                at: self.instruction_address,
            })),
            _ => self.clone()
        };
        return Ok(res);
    }

    /// Carries out a jump to a specified address. 
    /// 
    /// Will update the [BabyModel].`instruction_address` least significant 5 bits 
    /// to the last significant 5 bits of `address`, means jumping cannot be indexed outside
    /// of the memory, program execution will then proceed from this address. 
    /// 
    /// # Arguments
    /// 
    /// * `address` - The memory address to jump to. 
    /// 
    pub fn jump(&self, address: i32) -> BabyModel {
        let main_store = self.main_store.clone();
        let instruction_address = address as u16 & 0x1F;
        let instruction = main_store[instruction_address as usize] as u16;
        BabyModel { 
            main_store,
            accumulator: self.accumulator,
            instruction_address,
            instruction
        }
    }

    /// Carries out a jump to the instruction address plus an offset. 
    /// 
    /// This will add the [BabyModel].`instruction_address` to the offset, then set 
    /// the [BabyModel].`instruction_address` equal to the least significant 5 bits 
    /// of the result, this allows the jump to "loop" back to the start 
    /// of the memory, program execution will then proceed from this address. 
    /// 
    /// # Arguments 
    /// 
    /// * `offset` - The value to offset the [BabyModel].`instruction_address` to. 
    /// 
    pub fn relative_jump(&self, offset: i32) -> BabyModel {
        let main_store = self.main_store.clone();
        let instruction_address = (self.instruction_address + offset as u16) & 0x1F;
        let instruction = main_store[instruction_address as usize] as u16;
        BabyModel { 
            main_store,
            accumulator: self.accumulator,
            instruction_address,
            instruction
        }
    }

    /// Negates a value and stores it into the accumulator. 
    /// 
    /// Negates (adds or removes the "-") the specified value and 
    /// stores it in the accumulator, returning the updated model. 
    /// 
    /// Adds 1 to the [BabyModel].`instruction_address` and keeps only
    /// the least significant 5 bits as to only index within the 
    /// allocated memory. 
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to negate. 
    /// 
    pub fn negate(&self, value: i32) -> BabyModel {
        let main_store = self.main_store.clone();
        let instruction_address = (self.instruction_address + 1) & 0x1F;
        let instruction = main_store[instruction_address as usize] as u16;
        BabyModel { 
            main_store,
            accumulator: value.neg(),
            instruction_address,
            instruction
        }
    }

    /// Stores the accumulator at a specified address in memory. 
    /// 
    /// Takes the least significant 5 bits of `address` uses this to 
    /// index into the memory, as to not index outside of the memory 
    /// and stores the value in [BabyModel].`accumulator`. 
    /// 
    /// Adds 1 to the [BabyModel].`instruction_address` and keeps only
    /// the least significant 5 bits as to only index within the 
    /// allocated memory. 
    /// 
    /// # Arguments
    /// 
    /// * `address` - The address to store the accumulator to. 
    /// 
    pub fn store(&self, address: i32) -> BabyModel {
        let mut main_store = self.main_store.clone();
        main_store[address as usize] = self.accumulator;
        let instruction_address = (self.instruction_address + 1) & 0x1F;
        let instruction = main_store[instruction_address as usize] as u16;
        BabyModel { 
            main_store,
            accumulator: self.accumulator,
            instruction_address,
            instruction
        }
    }

    /// Subtracts the specified value from the accumulator. 
    /// 
    /// Subtracts the specified value from the accumulator, storing 
    /// the result back to the accumulator.  
    /// 
    /// Adds 1 to the [BabyModel].`instruction_address` and keeps only
    /// the least significant 5 bits as to only index within the allocated 
    /// memory, using this to get the next instruction from the memory and 
    /// storing it in [BabyModel].`instruction` register. 
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to subtract from the accumulator. 
    /// 
    pub fn subtract(&self, value: i32) -> BabyModel {
        let main_store = self.main_store.clone();
        let instruction_address = (self.instruction_address + 1) & 0x1F;
        let instruction = main_store[instruction_address as usize] as u16;
        BabyModel { 
            main_store,
            accumulator: self.accumulator - value,
            instruction_address,
            instruction
        }
    }


    /// Skips the next instruction address if the accumulator is negative. 
    /// 
    /// Adds 1 to the [BabyModel].`instruction_address` if the [BabyModel].`accumulator` 
    /// is not negative and 2 if it is and keeps only the least significant 5 bits 
    /// as to only index within the allocated memory, using this to get the next 
    /// instruction from the memory and storing it in [BabyModel].`instruction` register. 
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to subtract from the accumulator. 
    /// 
    pub fn test(&self) -> BabyModel {
        let instruction_address = if self.accumulator.is_negative() { self.instruction_address + 2 }
        else { self.instruction_address + 1 } & 0x1F;
        let main_store = self.main_store.clone();
        let instruction = main_store[instruction_address as usize] as u16;
        BabyModel { 
            main_store,
            accumulator: self.accumulator,
            instruction_address,
            instruction
        }
    }

    /// Generates a string representation of current state of the model. 
    /// 
    /// Generates a formatted string representation of all the registers 
    /// and memory addresses in the model, able to be printed to the console. 
    /// 
    /// # Example 
    /// ```
    /// use baby_emulator::core::BabyModel;
    /// use baby_emulator::errors::{BabyError, BabyErrors};
    /// 
    /// fn run_model(model: BabyModel) {
    ///     let mut result = model.execute();
    ///     let mut last_model = BabyModel::new();
    ///     while let Ok(new_model) = result {
    ///         last_model = new_model.clone();
    ///         result = new_model.execute();
    ///     }
    ///     match result {
    ///         // Shows the state of the model when it ends execution. 
    ///         Err(BabyErrors::Stop(_)) => println!("Sucess! \n{}", last_model.core_dump()),
    ///         _ => println!("Something went wrong. \n{}", last_model.core_dump())
    ///     }
    /// }
    /// ```
    pub fn core_dump(&self) -> String {
        let mut res = format!("Accumulator: {:#010x}; Instruction Register: {:#06x};\n", self.accumulator, self.instruction);
        res += &format!("Instruction Address: {:#06x}; Main Store: \n", self.instruction_address);
        
        for i in 0..(MEMORY_WORDS / 4) {
            let offset = i * 4;
            for i2 in 0..4 {
                let addr = i2 + offset;
                res += &format!("{:#04x}: {:#010x}; ", addr, self.main_store[addr]);
            }
            res += "\n";
        }
        res += "\n";
        return res;
    }
}
