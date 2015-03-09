# ca.rs: Simulate an elementary one-dimensional cellular automaton.


```
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
```
