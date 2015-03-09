#![feature(core, collections, plugin)]
#![plugin(docopt_macros)]
extern crate core;
extern crate docopt;
extern crate rand;

#[macro_use]
extern crate bitflags;

extern crate "rustc-serialize" as rustc_serialize;


use std::mem;
use std::collections::BitVec;
use core::num::wrapping::OverflowingOps;
use core::iter::FromIterator;
use docopt::Docopt;
use rand::random;

const RULE_TTT: u8  = 0b10000000;
const RULE_TTF: u8  = 0b01000000;
const RULE_TFT: u8  = 0b00100000;
const RULE_TFF: u8  = 0b00010000;
const RULE_FTT: u8  = 0b00001000;
const RULE_FTF: u8  = 0b00000100;
const RULE_FFT: u8  = 0b00000010;
const RULE_FFF: u8  = 0b00000001;

/// Takes a number representing three cells and returns whether, according
/// to this rule, the center cell should become live or dead at the next
/// generation.
fn evolve(rule: u8, state: u8) -> bool {
    let state = state & 7;  // we're only interested in the last 3 bits
    match state {
        7 => rule & RULE_TTT > 0,
        6 => rule & RULE_TTF > 0,
        5 => rule & RULE_TFT > 0,
        4 => rule & RULE_TFF > 0,
        3 => rule & RULE_FTT > 0,
        2 => rule & RULE_FTF > 0,
        1 => rule & RULE_FFT > 0,
        0 => rule & RULE_FFF > 0,
        _ => { panic!("if this happens, bitwise math is broken!?") }
    }
}

/// Get the three bits representing a single cell and its neighbors
/// If wrap is true, the first and last cells are considered neighbors, otherwise
/// any neighbors "out of bounds" are always considered to be dead.
fn cell_neighbors(cell_idx: usize, world: &BitVec, wrap: bool) -> u8 {
    if let Some(cell) = world.get(cell_idx) {
        let right = world.get(cell_idx.overflowing_add(1).0)
            .or_else(|| if wrap { Some(world[0]) } else { Some(false) }).unwrap();
        let left = world.get(cell_idx.overflowing_sub(1).0)
            .or_else(|| if wrap { Some(world[world.len()-1]) } else { Some(false) }).unwrap();
        
        (left as u8) << 2 | (cell as u8) << 1 | (right as u8)
    } else {
        0
    }
}

/// Simple function to print out a BitVec as '.' and '#' characters
fn format_bitvec(bv: &BitVec, dead: char, live: char) -> String {
    String::from_iter(bv.iter().map(|b| if b { live } else { dead }))
}

// Docopt usage string
docopt!(Args derive Debug, "
Simulate an elementary one-dimensional cellular automaton.
Usage: ca [options]

Options:
    -r, --rule=RULE     Use the given rule [default: 90].
    -c, --cells=CELLS   Simulate the given number of cells [default: 80].
    -n, --steps=STEPS   Run the simulation for a given number of generations [default: 24].
    -d, --dead=CHAR     Use the given character to display \"dead\" cells [default: .].
    -l, --live=CHAR     Use the given character to display \"live\" cells [default: #].
    --start=STRING      Initialize the world with the given values.  STRING should contain
                        only '.' or '#' characters, or the characters specified by the
                        d and l options.  Can be RANDOM to enable the --random flag.
    --random=PERCENT    Used with the --start flag; initialize the world randomly, giving
                        each cell a PERCENT chance to start live.  [default: 0.5].

    -h, --help          Show this message.
    --version           Show the version number.
",
        flag_rule: u8,
        flag_cells: usize,
        flag_steps: usize,
        flag_dead: char,
        flag_live: char,
        flag_random: f32
);

fn main() {
    let args: Args = Args::docopt()
        .version(Some(env!("CARGO_PKG_VERSION").to_string()))
        .decode().unwrap_or_else(|e| e.exit());

    // set up the world and rules
    let rule = args.flag_rule;
    println!("using rule {}", rule);

    let (dead, live) = (args.flag_dead, args.flag_live);

    let cells = args.flag_cells;
    let generations = args.flag_steps;
    println!("simulating {} cells for {} generations", cells, generations);

    let mut world = &mut BitVec::from_elem(cells, false);
    let mut next = &mut BitVec::from_elem(cells, false);

    // parse the start flag,
    // empty string means we default to 1 live cell in the center,
    // RANDOM means we use the --random option to scatter live cells,
    // anything else we try to parse with the given live/dead characters
    if args.flag_start == "" {
        world.set(cells / 2, true);
    } else if args.flag_start == "RANDOM" {
            for i in 0..cells {
                world.set(i, random::<f32>() < args.flag_random);
            }
    } else {
        for (i, c) in args.flag_start.chars().enumerate() {
            world.set(i,
                      if c == args.flag_live { true }
                      else if c == args.flag_dead { false }
                      else { panic!("Don't know how to handle '{}' char in --start flag", c); });
        }
    }

    // run the simulation
    for _ in 0..generations {
        println!("{}", format_bitvec(world, dead, live));
        for i in 0..cells {
            let state = cell_neighbors(i, &world, false);
            next.set(i, evolve(rule, state));
        }
        mem::swap(world, next);
    }
    
}
