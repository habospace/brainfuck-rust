use std::collections::LinkedList;
use std::collections::HashMap;

enum BfCommand {
    IncrPointer,
    DecrPointer,
    Incr,
    Decr,
    Print,
    BlockStart,
    BlockEnd
}

// TODO: rewrite this with a type that supports runtime sizing
// Array only support compile time static sizing
type BfCode = [BfCommand;106];

struct BfMemory<'a> {
    right:  &'a mut LinkedList<i32>,
    middle: &'a mut i32,
    left:   &'a mut LinkedList<i32>,
}

struct BfProgram<'a> {
    code: Box<BfCode>,
    memory: &'a mut BfMemory<'a>,
}

struct BfProgramExecution<'a> {
    print_chars: &'a mut LinkedList<String>, 
    program:     &'a mut BfProgram<'a>
}

fn move_cursor_right(memory: &mut BfMemory) {
    let new_middle = memory.right.pop_front();
    match new_middle {
        None          => (),
        Some(command) => {
            memory.left.push_back(*memory.middle);
            *memory.middle = command;
        }

    }
}

fn move_cursor_left(memory: &mut BfMemory) {
    let new_middle = memory.left.pop_back();
    match new_middle {
        None          => (),
        Some(command) => {
            memory.right.push_front(*memory.middle);
            *memory.middle = command;
        }
    }
}

fn incr_val(memory: &mut BfMemory) {
    *memory.middle += 1;
}

fn decr_val(memory: &mut BfMemory) {
    *memory.middle -= 1;
}

fn to_ascii(i: &i32) -> String {
    match *i {
        x@0..=127 => format!("{:?}", x as u8 as char),
        _ => "".into(),
    }
}

fn extract_blocks(code: &Box<BfCode>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut block_start_end_map: HashMap<usize, Option<usize>> = HashMap::new();
    let mut block_starts: LinkedList<usize> = LinkedList::new();
    for (i, command) in code.iter().enumerate() {
        match command {
            BfCommand::BlockStart => {
                block_start_end_map.insert(i, None);
                block_starts.push_back(i);

            }
            BfCommand::BlockEnd   => {
                match block_starts.pop_back() {
                    None              => {panic!("Unmatched block end (']') at {i}.");}
                    Some(block_start) => {block_start_end_map.insert(block_start, Some(i));}
                }
            }
            _                     => ()
        }
    }
    let mut block_end_start_map: HashMap<usize, usize> = HashMap::new();
    let mut block_start_end_map_: HashMap<usize, usize> = HashMap::new();
    for (block_start, maybe_block_end) in &block_start_end_map {
        match maybe_block_end {
            None            => {panic!("Unmatched block start ('[') at {block_start}.");}
            Some(block_end) => {
                block_end_start_map.insert(*block_end, *block_start);
                block_start_end_map_.insert(*block_start, *block_end);
            }
        }
    }
    (block_end_start_map, block_start_end_map_)
}

fn run_bf_program(execution: &mut BfProgramExecution) {
    let mut i: usize = 0;
    let (block_ends_starts, block_starts_ends_map) = extract_blocks(&execution.program.code);
    while i < execution.program.code.len() {
        let command = &execution.program.code[i];
        match command {
            BfCommand::IncrPointer => {move_cursor_right(execution.program.memory); i+=1;}
            BfCommand::DecrPointer => {move_cursor_left(execution.program.memory); i+=1;}
            BfCommand::Incr        => {incr_val(execution.program.memory); i+=1;}
            BfCommand::Decr        => {decr_val(execution.program.memory); i+=1;}
            BfCommand::Print       => {
                let print_char: String = to_ascii(execution.program.memory.middle);
                execution.print_chars.push_back(print_char);
                i+=1;
            }
            BfCommand::BlockStart => {
                if *execution.program.memory.middle == 0 {
                    match block_starts_ends_map.get(&i) {
                        None => (),
                        Some(block_end) => {i = block_end + 1;}
                    }
                } else {
                    i+=1;
                }
            }
            BfCommand::BlockEnd   => {
                if *execution.program.memory.middle == 0 {
                    i+=1;
                } else {
                    match block_ends_starts.get(&i) {
                        None    => (),
                        Some(x) => {i = *x + 1;}
                    }
                }
            }
        }
    }
}

fn main() {
    let hello_world_bf_code: BfCode = [
        BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, 
        BfCommand::Incr, BfCommand::Incr, BfCommand::BlockStart, BfCommand::IncrPointer, BfCommand::Incr, BfCommand::Incr, 
        BfCommand::Incr, BfCommand::Incr, BfCommand::BlockStart, BfCommand::IncrPointer, BfCommand::Incr, BfCommand::Incr, 
        BfCommand::IncrPointer, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, BfCommand::IncrPointer, BfCommand::Incr, 
        BfCommand::Incr, BfCommand::Incr, BfCommand::IncrPointer, BfCommand::Incr, BfCommand::DecrPointer, BfCommand::DecrPointer, 
        BfCommand::DecrPointer, BfCommand::DecrPointer, BfCommand::Decr, BfCommand::BlockEnd, BfCommand::IncrPointer, BfCommand::Incr, 
        BfCommand::IncrPointer, BfCommand::Incr, BfCommand::IncrPointer, BfCommand::Decr, BfCommand::IncrPointer, BfCommand::IncrPointer, 
        BfCommand::Incr, BfCommand::BlockStart, BfCommand::DecrPointer, BfCommand::BlockEnd, BfCommand::DecrPointer, BfCommand::Decr, 
        BfCommand::BlockEnd, BfCommand::IncrPointer, BfCommand::IncrPointer, BfCommand::Print, BfCommand::IncrPointer, BfCommand::Decr, 
        BfCommand::Decr, BfCommand::Decr, BfCommand::Print, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, 
        BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, BfCommand::Print, BfCommand::Print, BfCommand::Incr, BfCommand::Incr, 
        BfCommand::Incr, BfCommand::Print, BfCommand::IncrPointer, BfCommand::IncrPointer, BfCommand::Print, BfCommand::DecrPointer, 
        BfCommand::Decr, BfCommand::Print, BfCommand::DecrPointer, BfCommand::Print, BfCommand::Incr, BfCommand::Incr, BfCommand::Incr, 
        BfCommand::Print, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, 
        BfCommand::Print, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, BfCommand::Decr, 
        BfCommand::Decr, BfCommand::Decr, BfCommand::Print, BfCommand::IncrPointer, BfCommand::IncrPointer, BfCommand::Incr, BfCommand::Print,
        BfCommand::IncrPointer, BfCommand::Incr, BfCommand::Incr, BfCommand::Print
        
    ];

    let mut print_chars = LinkedList::new();
    let mut left = LinkedList::from([0; 1000]);
    let mut middle = 0;
    let mut right = LinkedList::from([0; 1000]);
    let mut bf_memory = BfMemory {
        left: &mut left, middle: &mut middle, right: &mut right
    };
    let mut bf_program = BfProgram {
        memory: &mut bf_memory,
        code: Box::new(hello_world_bf_code)
    };
    let mut execution = BfProgramExecution {
        print_chars: &mut print_chars,
        program: &mut bf_program
    };

    run_bf_program(&mut execution);
    println!("{:?}", print_chars);
}