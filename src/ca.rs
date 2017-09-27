#[macro_use]
extern crate serde_derive;

extern crate docopt;
use docopt::Docopt;

extern crate rand;
use rand::random;

extern crate bit_vec;
use bit_vec::BitVec;

/// The CA World
struct World {
    rule: u8,           // the rule that will be used to determine each new generation
    wrap: bool,         // whether to treat the first and last cells in the world as neighbors
    size: usize,        // the number of cells in the world
    state: BitVec,      // the current state of each cell in the world
    state_next: BitVec, // buffer used while calculating the next generation
}

impl World {
    /// Create a new World with the given rule and number of cells
    pub fn new(rule: u8, size: usize, wrap: bool) -> World {
        World {
            rule: rule,
            wrap: wrap,
            size: size,
            state: BitVec::from_elem(size, false),
            state_next: BitVec::from_elem(size, false),
        }
    }

    /// Returns the underlying BitVec with the current state of the world
    pub fn get_state(&self) -> &BitVec {
        return &self.state;
    }

    /// Sets the cell at the given index to the given state
    pub fn set(&mut self, index: usize, state: bool) {
        self.state.set(index, state);
    }

    /// Applies the rule to each cell
    pub fn step(&mut self) {
        for cell_index in 0..self.size {
            let neighbors = self.cell_neighbors(cell_index);
            // pick the bit for this cell state out of the rule
            self.state_next.set(
                cell_index,
                self.rule & (1 << neighbors) != 0,
            );
        }
        std::mem::swap(&mut self.state, &mut self.state_next);
    }

    // Private function to extract the three bits representing a cell and its
    // neighbors, returned as a u8
    fn cell_neighbors(&self, c_idx: usize) -> u8 {
        // get the state of each cell, with checks to ensure we stay in bounds
        // and wrap if appropriate
        let left = match (c_idx == 0, self.wrap) {
            (true, true) => self.state[self.size - 1], // first cell and wrapping
            (true, false) => false,                    // first cell and not wrapping
            (_, _) => self.state[c_idx - 1],           // any other case
        };

        let right = match (c_idx == self.size - 1, self.wrap) {
            (true, true) => self.state[0],   // last cell and wrapping
            (true, false) => false,          // last cell and not wrapping
            (_, _) => self.state[c_idx + 1], // any other case
        };

        let cell = self.state[c_idx];

        // combine the bits into a byte
        return (left as u8) << 2 | (cell as u8) << 1 | (right as u8);
    }
}

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
    -w, --wrap          If present, causes the world to that the first and last cells are
                        neighbors.
    --skip-to-end       Only output the state of the last generation

    -h, --help          Show this message.
    --version           Show the version number.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_rule: u8,
    flag_cells: usize,
    flag_wrap: bool,
    flag_start: String,
    flag_steps: usize,
    flag_dead: char,
    flag_live: char,
    flag_random: f32,
    flag_skip_to_end: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| {
            d.version(Some(env!("CARGO_PKG_VERSION").to_string()))
                .help(true)
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());

    // set up the world and rules
    let mut world = World::new(args.flag_rule, args.flag_cells, args.flag_wrap);
    println!("using rule {}", args.flag_rule);
    println!(
        "simulating {} cells for {} generations",
        args.flag_cells,
        args.flag_steps
    );

    // parse the start flag,
    // empty string means we default to 1 live cell in the center,
    // RANDOM means we use the --random option to scatter live cells,
    // anything else we try to parse with the given live/dead characters
    if args.flag_start == "" {
        world.set(args.flag_cells / 2, true);
    } else if args.flag_start == "RANDOM" {
        for i in 0..args.flag_cells {
            world.set(i, random::<f32>() < args.flag_random);
        }
    } else {
        for (i, c) in args.flag_start.chars().enumerate() {
            world.set(i,
                      if c == args.flag_live {
                          true
                      } else if c == args.flag_dead {
                          false
                      } else {
                          panic!("Don't know how to handle '{}' char in --start flag", c);
                      },
            );
        }
    }

    // keep and reuse a String buffer for the formatted version of the world
    let mut buffer = String::new();

    if !args.flag_skip_to_end {
        format_world(&mut buffer, &world, args.flag_dead, args.flag_live);
        println!("{}", buffer);
    }

    // run the simulation
    for _ in 0..args.flag_steps {
        world.step();

        if !args.flag_skip_to_end {
            format_world(&mut buffer, &world, args.flag_dead, args.flag_live);
            println!("{}", buffer);
        }
    }

    if args.flag_skip_to_end {
        format_world(&mut buffer, &world, args.flag_dead, args.flag_live);
        println!("final state:\n{}", buffer);
    }
}

/// Format a World into a String buffer
fn format_world(buffer: &mut String, world: &World, dead: char, live: char) {
    buffer.clear();
    for cell in world.get_state().iter() {
        if cell {
            buffer.push(live);
        } else {
            buffer.push(dead);
        }
    }
}
