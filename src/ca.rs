#[macro_use]
extern crate serde_derive;

extern crate docopt;
use docopt::Docopt;

extern crate rand;
use rand::random;

extern crate bit_vec;
use bit_vec::BitVec;

const USAGE: &'static str = "
ca: Simulate an elementary one-dimensional cellular automaton.

Usage: ca [options]
Try 'ca --help' for more information.

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
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_rule: u8,
    flag_cells: usize,
    flag_start: String,
    flag_steps: usize,
    flag_dead: char,
    flag_live: char,
    flag_random: f32
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(env!("CARGO_PKG_VERSION").to_string()))
                  .help(true)
                  .deserialize())
        .unwrap_or_else(|e| e.exit());

    // set up the world and rules
    let rule = args.flag_rule;
    println!("using rule {}", rule);

    let (dead, live) = (args.flag_dead, args.flag_live);

    let cells = args.flag_cells;
    let generations = args.flag_steps;
    println!("simulating {} cells for {} generations", cells, generations);

    let world = &mut BitVec::from_elem(cells, false);
    let next = &mut BitVec::from_elem(cells, false);

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
        std::mem::swap(world, next);
    }
}

/// Takes a number representing three cells and returns whether, according
/// to this rule, the center cell should become live or dead at the next
/// generation.
fn evolve(rule: u8, state: u8) -> bool {
    let state = state & 7;  // we're only interested in the last 3 bits

    // state will be a number in the range 0b000 (0) to 0b111 (7) , we
    // want to pick the corresponding bit from the rule
    (rule & 1 << state) > 0
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
    bv.iter().map(|b| if b { live } else { dead }).collect::<String>()
}
