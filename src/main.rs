use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
enum BfCommand {
    IncrPointer,
    DecrPointer,
    Incr,
    Decr,
    Print,
    BlockStart,
    BlockEnd,
    Comment
}

type Index = usize;
type BlockStartIndex = usize;
type BlockEndIndex = usize;

type BfCode = BTreeMap<Index, BfCommand>;

struct BfMemory<'a> {
    right:  &'a mut LinkedList<i32>,
    middle: &'a mut i32,
    left:   &'a mut LinkedList<i32>,
}

struct BfProgram<'a> {
    code: BfCode,
    memory: &'a mut BfMemory<'a>,
}

struct BfProgramExecution<'a> {
    print_chars: &'a mut LinkedList<char>,
    program:     &'a mut BfProgram<'a>
}

fn char_to_bf_command(c: char) -> BfCommand {
    let command: BfCommand = match c {
        '>' => BfCommand::IncrPointer,
        '<' => BfCommand::DecrPointer,
        '+' => BfCommand::Incr,
        '-' => BfCommand::Decr,
        '.' => BfCommand::Print,
        '[' => BfCommand::BlockStart,
        ']' => BfCommand::BlockEnd,
        _   => BfCommand::Comment
    };
    command
}

fn translate_string_to_bf_code(str_code: &String) -> BfCode {
    let mut bfcode: BfCode = BTreeMap::new();
    for (i, char_command) in str_code.chars().enumerate() {
        bfcode.insert(i, char_to_bf_command(char_command));
    }
    bfcode
}

fn move_cursor_right(memory: &mut BfMemory) {
    let new_middle = memory.right.pop_front();
    memory.left.push_back(*memory.middle);
    match new_middle {
        None    => {*memory.middle = 0;},
        Some(x) => {*memory.middle = x;}
    }
}

fn move_cursor_left(memory: &mut BfMemory) {
    let new_middle = memory.left.pop_back();
    memory.right.push_front(*memory.middle);
    match new_middle {
        None    => {*memory.middle = 0;},
        Some(x) => {*memory.middle = x;}
    }
}

fn incr_val(memory: &mut BfMemory) {
    *memory.middle += 1;
}

fn decr_val(memory: &mut BfMemory) {
    *memory.middle -= 1;
}

fn to_ascii(i: &i32) -> char {
    match *i {
        x@0..=127 => x as u8 as char,
        _         => {panic!("'{i}' can't be translated to char.");},
    }
}

fn extract_blocks(code: &BfCode) ->
    (HashMap<BlockEndIndex, BlockStartIndex>, HashMap<BlockStartIndex, BlockEndIndex>) {

    let mut block_start_end_map: HashMap<BlockStartIndex, Option<BlockEndIndex>> = HashMap::new();
    let mut block_starts: LinkedList<BlockStartIndex> = LinkedList::new();
    for (i, command) in code {
        match command {
            BfCommand::BlockStart => {
                block_start_end_map.insert(*i, None);
                block_starts.push_back(*i);

            }
            BfCommand::BlockEnd   => {
                match block_starts.pop_back() {
                    None              => {panic!("Unmatched block end (']') at {i}.");}
                    Some(block_start) => {block_start_end_map.insert(block_start, Some(*i));}
                }
            }
            _                     => ()
        }
    }
    let mut block_end_start_map: HashMap<BlockEndIndex, BlockStartIndex> = HashMap::new();
    let mut block_start_end_map_: HashMap<BlockStartIndex, BlockEndIndex> = HashMap::new();
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
        let command = match execution.program.code.get(&i) {
            None    => BfCommand::Comment,
            Some(c) => *c
        };
        match command {
            BfCommand::IncrPointer => {move_cursor_right(execution.program.memory); i+=1;}
            BfCommand::DecrPointer => {move_cursor_left(execution.program.memory); i+=1;}
            BfCommand::Incr        => {incr_val(execution.program.memory); i+=1;}
            BfCommand::Decr        => {decr_val(execution.program.memory); i+=1;}
            BfCommand::Print       => {
                let print_char: char = to_ascii(execution.program.memory.middle);
                execution.print_chars.push_back(print_char);
                i+=1;
            }
            BfCommand::BlockStart  => {
                if *execution.program.memory.middle == 0 {
                    match block_starts_ends_map.get(&i) {
                        None => (),
                        Some(block_end) => {i = block_end + 1;}
                    }
                } else {
                    i+=1;
                }
            }
            BfCommand::BlockEnd    => {
                if *execution.program.memory.middle == 0 {
                    i+=1;
                } else {
                    match block_ends_starts.get(&i) {
                        None    => (),
                        Some(x) => {i = *x + 1;}
                    }
                }
            }
            _                      => {i+=1;}
        }
    }
}

fn main() {
    let hello_world_bf_code_str: String = String::from(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]
        <-]>>.>---.+++++++..+++.>>.<-.<.+++.------.----
        ----.>>+.>++."
    );
    let hello_world_bf_code: BfCode = translate_string_to_bf_code(&hello_world_bf_code_str);
    let mut print_chars = LinkedList::new();
    let mut left = LinkedList::from([0; 1000]);
    let mut middle = 0;
    let mut right = LinkedList::from([0; 1000]);
    let mut bf_memory = BfMemory {
        left: &mut left, middle: &mut middle, right: &mut right
    };
    let mut bf_program = BfProgram {
        memory: &mut bf_memory,
        code: hello_world_bf_code
    };
    let mut execution = BfProgramExecution {
        print_chars: &mut print_chars,
        program: &mut bf_program
    };

    run_bf_program(&mut execution);

    let mut print_string = String::new();
    for c in print_chars {
        print_string.push(c);
    }
    println!("{:?}", print_string);
}
