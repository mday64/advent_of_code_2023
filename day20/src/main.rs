use std::collections::{VecDeque, HashMap};
use num::Integer;

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 841763884);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 246006621493687);
}

fn part1(input: &str) -> u32 {
    // Flip-flop state: map name to on/off
    let mut flip_flops = HashMap::<&str, bool>::new();
    // Conjunction memory: map name to (sender -> last pulse)
    let mut conjunction_memory = HashMap::<&str, HashMap<&str, bool>>::new();
    // Module destinations
    let mut destinations = HashMap::<&str, Vec<&str>>::new();
    // Module kinds
    let mut module_kind = HashMap::<&str, ModuleKind>::new();

    // Parse the input
    for mut line in input.lines() {
        let kind = if line.starts_with('%') {
            line = line.strip_prefix('%').unwrap();
            ModuleKind::FlipFlop
        } else if line.starts_with('&') {
            line = line.strip_prefix('&').unwrap();
            ModuleKind::Conjunction
        } else {
            ModuleKind::Generic
        };
        
        let (name, rest) = line.split_once(" -> ").unwrap();
        let dest_names = rest.split(", ").collect::<Vec<_>>();

        module_kind.insert(name, kind);
        destinations.insert(name, dest_names);
        match kind {
            ModuleKind::Generic => {}
            ModuleKind::FlipFlop => { flip_flops.insert(name, false); }
            ModuleKind::Conjunction => { conjunction_memory.insert(name, HashMap::new()); }
        }

        // Set up the conjunction_memory.  We need to go through all of the
        // modules again, and for every destination that is a conjunction,
        // add that sender->false to the memory.
        for (sender, receivers) in destinations.iter() {
            for receiver in receivers {
                if let Some(memory) = conjunction_memory.get_mut(receiver) {
                    memory.insert(sender, false);
                }
            }
        }
    }

    let mut queue = VecDeque::<Pulse>::new();
    let mut num_low = 0;
    let mut num_high = 0;

    for _ in 0..1000 {
        // Button sends a low pulse to "broadcaster"
        queue.push_back(Pulse{sender: "button", receiver: "broadcaster", high: false});

        // Process all of the pulses that result from the button push
        while let Some(Pulse{sender, receiver, high}) = queue.pop_front() {
            // Process this pulse

            // Count high and low pulses sent
            if high {
                num_high += 1;
            } else {
                num_low += 1;
            }

            match module_kind.get(receiver) {
                Some(ModuleKind::FlipFlop) => {
                    if !high {
                        // Invert the flip-flop's current state
                        let state = flip_flops.get_mut(receiver).unwrap();
                        *state = !*state;
    
                        // Send out a pulse equal to the current state
                        for destination in destinations.get(receiver).unwrap() {
                            queue.push_back(Pulse{sender: receiver, receiver: destination, high: *state});
                        }
                    }
                }
                Some(ModuleKind::Conjunction) => {
                    // Update memory from sender
                    let memory = conjunction_memory.get_mut(receiver).unwrap();
                    memory.insert(sender, high);

                    // If all of its senders last sent it high pulses,
                    //      then send a low pulse
                    //      else send a high pulse
                    let send_high = !memory.values().all(|&v| v);
                    for destination in destinations.get(receiver).unwrap() {
                        queue.push_back(Pulse{sender: receiver, receiver: destination, high: send_high});
                    }
                }
                Some(ModuleKind::Generic) => {
                    for destination in destinations.get(receiver).unwrap() {
                        queue.push_back(Pulse{sender: receiver, receiver: destination, high});
                    }    
                }
                None => {
                    /* A receiver that is never mentioned on the left hand side */
                }
            }
        }
    }

    // Return # low pulses * # high pulses
    num_high * num_low
}

//
// Sigh.  We're going to have to analyze the logic in input.txt to find
// the pattern of states that will eventually result in a low pulse being
// sent to "rx".  It's probably some annoying counters plus multiplication
// (multiple counters have to get to their magic value at the same time).
//
// So, that final node, "rx" has one input: a conjunction (NAND) node.
// That conjunction node has multiple inputs (4 in my input).  It will
// send the desired low pulse to "rx" when all of its inputs are
// simultaneously high.  I'm assuming that those inputs go high every
// "N" button presses (with different "N" for each input; those values
// are probably relatively prime).  The solution is to find the number
// of button pushes to make each of those inputs go high, and then
// calculate the lowest common multiple for the answer.
//
fn part2(input: &str) -> u64 {
    // Flip-flop state: map name to on/off
    let mut flip_flops = HashMap::<&str, bool>::new();
    // Conjunction memory: map name to (sender -> last pulse)
    let mut conjunction_memory = HashMap::<&str, HashMap<&str, bool>>::new();
    // Module destinations
    let mut destinations = HashMap::<&str, Vec<&str>>::new();
    // Module kinds
    let mut module_kind = HashMap::<&str, ModuleKind>::new();

    // Parse the input
    for mut line in input.lines() {
        let kind = if line.starts_with('%') {
            line = line.strip_prefix('%').unwrap();
            ModuleKind::FlipFlop
        } else if line.starts_with('&') {
            line = line.strip_prefix('&').unwrap();
            ModuleKind::Conjunction
        } else {
            ModuleKind::Generic
        };
        
        let (name, rest) = line.split_once(" -> ").unwrap();
        let dest_names = rest.split(", ").collect::<Vec<_>>();

        module_kind.insert(name, kind);
        destinations.insert(name, dest_names);
        match kind {
            ModuleKind::Generic => {}
            ModuleKind::FlipFlop => { flip_flops.insert(name, false); }
            ModuleKind::Conjunction => { conjunction_memory.insert(name, HashMap::new()); }
        }

        // Set up the conjunction_memory.  We need to go through all of the
        // modules again, and for every destination that is a conjunction,
        // add that sender->false to the memory.
        for (sender, receivers) in destinations.iter() {
            for receiver in receivers {
                if let Some(memory) = conjunction_memory.get_mut(receiver) {
                    memory.insert(sender, false);
                }
            }
        }
    }

    // Figure out the name of the conjunction node that sends to "rx"
    let mut conjunction_name = None;
    for (sender, receivers) in destinations.iter() {
        if receivers.first() == Some(&"rx") {
            conjunction_name = Some(sender);
            break;
        }
    }
    let conjunction_name = *conjunction_name.unwrap();

    // Figure out the names of the nodes that send to the conjunction node.
    // Create a HashMap to keep track of the cycle lengths.
    // Note: I'm assuming that all of the cycles start with the very first
    // button press, and there are no initial states prior to the first cycle.
    let mut cycle_lengths: HashMap<&str, Option<u64>> = HashMap::new();
    for (sender, receivers) in destinations.iter() {
        if receivers.contains(&conjunction_name) {
            cycle_lengths.insert(sender, None);
        }
    }

    // Start pressing the button until we have determined all of the cycle lengths.
    let mut queue = VecDeque::<Pulse>::new();
    let mut presses = 0;

    loop {
        presses += 1;

        // Button sends a low pulse to "broadcaster"
        queue.push_back(Pulse{sender: "button", receiver: "broadcaster", high: false});

        // Process all of the pulses that result from the button push
        while let Some(Pulse{sender, receiver, high}) = queue.pop_front() {
            // Process this pulse
            match module_kind.get(receiver) {
                Some(ModuleKind::FlipFlop) => {
                    if !high {
                        // Invert the flip-flop's current state
                        let state = flip_flops.get_mut(receiver).unwrap();
                        *state = !*state;
    
                        // Send out a pulse equal to the current state
                        for destination in destinations.get(receiver).unwrap() {
                            queue.push_back(Pulse{sender: receiver, receiver: destination, high: *state});
                        }
                    }
                }
                Some(ModuleKind::Conjunction) => {
                    // Update memory from sender
                    let memory = conjunction_memory.get_mut(receiver).unwrap();
                    memory.insert(sender, high);

                    // If all of its senders last sent it high pulses,
                    //      then send a low pulse
                    //      else send a high pulse
                    let send_high = !memory.values().all(|&v| v);
                    for destination in destinations.get(receiver).unwrap() {
                        queue.push_back(Pulse{sender: receiver, receiver: destination, high: send_high});
                    }

                    // If the receiver was the last conjunction node, and the
                    // pulse was high, then set this sender's cycle length
                    // (if it was not yet set).
                    if receiver == conjunction_name && high {
                        cycle_lengths.insert(sender, Some(presses));
                    }
                }
                Some(ModuleKind::Generic) => {
                    for destination in destinations.get(receiver).unwrap() {
                        queue.push_back(Pulse{sender: receiver, receiver: destination, high});
                    }    
                }
                None => {
                    /* A receiver that is never mentioned on the left hand side */
                }
            }
        }

        // Have we determined all of the cycle lengths yet?
        if cycle_lengths.values().all(Option::is_some) {
            break;
        }
    }

    // Calculate LCM of the various cycle lengths
    let mut result = 1;
    for v in cycle_lengths.values() {
        result = result.lcm(&v.unwrap());
    }
    result
}

#[derive(Debug, Clone, Copy)]
enum ModuleKind {
    FlipFlop,
    Conjunction,
    Generic
}

struct Pulse<'a> {
    sender: &'a str,
    receiver: &'a str,
    high: bool
}

#[cfg(test)]
static EXAMPLE1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

#[cfg(test)]
static EXAMPLE2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

#[test]
fn test_part1_ex1() {
    assert_eq!(part1(EXAMPLE1), 32000000);
}

#[test]
fn test_part1_ex2() {
    assert_eq!(part1(EXAMPLE2), 11687500);
}
