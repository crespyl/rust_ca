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

fn computeSingle(rules: Rules, a:bool, b:bool, c:bool) -> bool {
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

fn computeWorldParallel(rules: Rules, world: &[bool]) -> ~[bool] {
    let mut new_world = ~[];
    new_world.push_all(world);

    let (tx, rx): (Sender<(uint, bool)>, Receiver<(uint, bool)>) = channel();

    for i in range(0, new_world.len()) {
        let a = if i > 0 {
            world[i-1]
        } else {
            false
        };
        let b = world[i];
        let c = if i < (new_world.len()-1) {
            world[i+1]
        } else {
            false
        };

        let child_tx = tx.clone();
        spawn(proc() {
            let result = (i, computeSingle(rules, a, b, c));
            child_tx.send(result);
        });
    }

    for _ in range(0, new_world.len()) {
        let (index, result) = rx.recv();
        new_world[index] = result;
    }

    return new_world;
}

fn computeWorld(rules: Rules, world: &[bool]) -> ~[bool]{
    let mut new_world = ~[];
    new_world.push_all(world);

    for i in range(0, new_world.len()) {
        let a = if i > 0 {
            world[i-1]
        } else {
            false
        };
        let b = world[i];
        let c = if i < (new_world.len()-1) {
            world[i+1]
        } else {
            false
        };

        new_world[i] = computeSingle(rules, a, b, c);
    }

    return new_world;
}

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

   let rule110 = Rules {
       ttt: false,
       ttf: true,
       tft: true,
       tff: false,
       ftt: true,
       ftf: true,
       fft: true,
       fff: false
   };


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
