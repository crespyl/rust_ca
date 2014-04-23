use std::io::println;
use std::io::print;

struct Rules {
    ttt: bool,
    ttf: bool,
    tft: bool,
    tff: bool,
    ftt: bool,
    ftf: bool,
    fft: bool,
    fff: bool
}

// This function simply takes a ruleset and three neighboring cells (a,b, and c) and
// matches the cells against the resulting output state according to this ruleset
fn computeSingle(rules: Rules, a:bool, b:bool, c:bool) -> bool {
    // here we group a b and c into a triple just to simplify the match statement
    match (a, b, c) {
        (true, true, true)    => rules.ttt,
        (true, true, false)   => rules.ttf,
        (true, false, true)   => rules.tft,
        (true, false, false)  => rules.tff,
        (false, true, true)   => rules.ftt,
        (false, true, false)  => rules.ftf,
        (false, false, true)  => rules.fft,
        (false, false, false) => rules.fff
    }
}

// This function takes a ruleset and a world (an array of cells), and returns a new
// world representing one step of the automaton, placing the computation for each
// cell in a separate task.
// The overhead of spawning so many tasks for such a simple calculation usually
// means that the single-threaded version is usually much faster, but this way is
// more fun...
fn computeWorldParallel(rules: Rules, world: &[bool]) -> ~[bool] {
    // declare an array to put our resulting state in
    let mut new_world = ~[];
    new_world.push_all(world);

    // create a channel pair to communicate with our sub-tasks
    let (tx, rx): (Sender<(uint, bool)>, Receiver<(uint, bool)>) = channel();

    // iterate over every cell in the world
    for i in range(0, new_world.len()) {
        // extract cells a, b, and c, where b is the current cell,
        // a is the preceeding cell, and c is the following cell.
        // if a or c is out of bounds, assume that their value is false

        let a = if i > 0 { world[i-1] } else { false };
        let b = world[i];
        let c = if i < (new_world.len()-1) { world[i+1] } else { false };

        // clone tx (the Sender part of the channel pair) before handing it off
        // to our sub-task (after which we won't have access to it any longer)
        let child_tx = tx.clone();
        spawn(proc() {
            // wrap the cell index (i) and the computed state for this cell in
            // a tuple, then send the result back to the parent task via the
            // channel
            let result = (i, computeSingle(rules, a, b, c));
            child_tx.send(result);
        });
    }

    // once for each cell, collect a (cell_index, new_state) tuple from our end
    // of the channel, and apply the new state, then return our new array
    for _ in range(0, new_world.len()) {
        // (we need the cell index from the child task since the order of completion
        // is not guaranteed)
        let (index, result) = rx.recv();
        new_world[index] = result;
    }

    return new_world;
}

fn computeWorld(rules: Rules, world: &[bool]) -> ~[bool]{
    let mut new_world = ~[];
    new_world.push_all(world);

    for i in range(0, new_world.len()) {
        let a = if i > 0 { world[i-1] } else { false };
        let b = world[i];
        let c = if i < (new_world.len()-1) { world[i+1] } else { false };

        new_world[i] = computeSingle(rules, a, b, c);
    }

    return new_world;
}

// Simply iterate over each cell in the array, printing either a blank or a # character
fn printWorld(world: &[bool]) {
    print("|");
    for i in world.iter() {
        if *i  {
            print("#");
        } else {
            print(" ");
        }
    }
    println("|");
}

fn main() {

    let rule30 = Rules {
        ttt: false,
        ttf: false,
        tft: false,
        tff: true,
        ftt: true,
        ftf: true,
        fft: true,
        fff: false
    };

    // let rule110 = Rules {
    //     ttt: false,
    //     ttf: true,
    //     tft: true,
    //     tff: false,
    //     ftt: true,
    //     ftf: true,
    //     fft: true,
    //     fff: false
    // };


    let mut world = ~[];
    for i in range(0, 200) {
        world.push(false);
    }
    world.push(true);

    printWorld(world);

    for _ in range(0, 200) {
        //world = computeWorldParallel(rule30, world);
        world = computeWorld(rule30, world);
        printWorld(world);
    }
}
