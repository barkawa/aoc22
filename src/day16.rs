use super::graph::Graph;
use regex::Regex;

fn parse_input(input: &[String]) -> Graph<String, u8, ()> {
    let mut graph: Graph<String, u8, ()> = Graph::new();

    let regex = Regex::new(
        r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? ((?:(?:\w\w)(?:, |$))+)",
    )
    .unwrap();

    for line in input.iter() {
        let captures = regex.captures(line).unwrap();

        let valve_id = &captures[1];
        let flow_rate = captures[2].parse().unwrap();
        let adjacent_valves = captures[3].split(", ");

        graph.push_vertex(valve_id.to_string(), flow_rate);

        for adjacent_valve_id in adjacent_valves {
            graph.push_edge(valve_id.to_string(), adjacent_valve_id.to_string(), ());
        }
    }

    graph
}

pub fn day16() {
    let input = super::get_input();
    let graph = parse_input(&input);
    const TIME: u8 = 30;

    
}
