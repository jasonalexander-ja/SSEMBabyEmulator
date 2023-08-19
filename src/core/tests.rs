use super::*;


#[test]
fn test_new_with_program() {
    let model = BabyModel::new_with_program([5; MEMORY_WORDS]);
    assert_eq!(model.instruction, 5);
}

#[test]
fn test_new_example_program() {
    let model = BabyModel::new_example_program();
    assert_eq!(model.instruction, 16389)
}

#[test]
fn test_decode_instruction() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 0,
        instruction: 16383
    };
    let (value, instruction) = model.decode_instruction();
    assert_eq!(instruction, BabyInstruction::Subtract(31));
    assert_eq!(value, 31);
}

#[test]
fn test_run_loop() {
    let model = BabyModel::new_example_program();
    let (new_model, err) = model.run_loop(1);
    assert_eq!(new_model.accumulator, 5);
    match err {
        BabyErrors::IterationExceeded(err) => {
            assert_eq!(err.end_model.accumulator, 5);
            assert_eq!(err.end_model.instruction_address, 1);
            assert_eq!(err.end_model.instruction, model.main_store[1] as u16);
            assert_eq!(err.max_iter, 1);
        }
        _ => assert!(false)
    }
}

#[test]
fn test_jump_in_range() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel::new_with_program(main_store);
    let new_model = model.jump(5);
    assert_eq!(new_model.instruction, 5);
    assert_eq!(new_model.instruction_address, 5);
}

#[test]
fn test_jump_in_range_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel::new_with_program(main_store);
    let new_model = model.dispatch_instruction(BabyInstruction::Jump(0), 5).unwrap();
    assert_eq!(new_model.instruction, 5);
    assert_eq!(new_model.instruction_address, 5);
}

#[test]
fn test_jump_out_range() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel::new_with_program(main_store);
    let new_model = model.jump(32);
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_jump_out_range_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel::new_with_program(main_store);
    let new_model = model.dispatch_instruction(BabyInstruction::Jump(0), 32).unwrap();
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_relative_jump_in_range() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 2,
        instruction: 2
    };
    let new_model = model.relative_jump(5);
    assert_eq!(new_model.instruction, 7);
    assert_eq!(new_model.instruction_address, 7);
}

#[test]
fn test_relative_jump_in_range_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 2,
        instruction: 2
    };
    let new_model = model.dispatch_instruction(BabyInstruction::RelativeJump(0), 5).unwrap();
    assert_eq!(new_model.instruction, 7);
    assert_eq!(new_model.instruction_address, 7);
}


#[test]
fn test_relative_jump_out_range() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 2,
        instruction: 2
    };
    let new_model = model.relative_jump(30);
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_relative_jump_out_range_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 2,
        instruction: 2
    };
    let new_model = model.dispatch_instruction(BabyInstruction::RelativeJump(0), 30).unwrap();
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_negate() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.negate(5);
    assert_eq!(new_model.accumulator, -5);
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_negate_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 0,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.dispatch_instruction(BabyInstruction::Negate(0), 5).unwrap();
    assert_eq!(new_model.accumulator, -5);
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_store() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.store(32);
    assert_eq!(new_model.main_store[0], 5);
    assert_eq!(new_model.instruction, 5);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_store_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.dispatch_instruction(BabyInstruction::Store(0), 32).unwrap();
    assert_eq!(new_model.main_store[0], 5);
    assert_eq!(new_model.instruction, 5);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_subtract() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.subtract(5);
    assert_eq!(new_model.accumulator, 0);
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_subtract_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.dispatch_instruction(BabyInstruction::Subtract(0), 5).unwrap();
    assert_eq!(new_model.accumulator, 0);
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_test_negative() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: -5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.test();
    assert_eq!(new_model.instruction, 1);
    assert_eq!(new_model.instruction_address, 1);
}

#[test]
fn test_test_negative_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: -5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.dispatch_instruction(BabyInstruction::SkipNextIfNegative, 0).unwrap();
    assert_eq!(new_model.instruction, 1);
    assert_eq!(new_model.instruction_address, 1);
}

#[test]
fn test_test_positive() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.test();
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}

#[test]
fn test_test_positive_dispatch() {
    let main_store: [i32; MEMORY_WORDS] = core::array::from_fn(|i| i as i32);
    let model = BabyModel {
        main_store,
        accumulator: 5,
        instruction_address: 31,
        instruction: 31
    };
    let new_model = model.dispatch_instruction(BabyInstruction::SkipNextIfNegative, 0).unwrap();
    assert_eq!(new_model.instruction, 0);
    assert_eq!(new_model.instruction_address, 0);
}
