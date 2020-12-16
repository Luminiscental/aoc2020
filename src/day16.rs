use crate::day::Day;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Input<'a> {
    fields: HashMap<&'a str, Field>,
    your_ticket: ObfuscatedTicket,
    nearby_tickets: Vec<ObfuscatedTicket>,
}

#[derive(Debug)]
struct Field {
    ranges: Vec<(usize, usize)>,
}

impl Field {
    fn parse(string: &str) -> Self {
        Field {
            ranges: string
                .split(" or ")
                .map(|range| {
                    let mut parts = range.split('-');
                    let min = parts.next().unwrap().parse::<usize>().unwrap();
                    let max = parts.next().unwrap().parse::<usize>().unwrap();
                    (min, max)
                })
                .collect(),
        }
    }

    fn is_valid(&self, value: usize) -> bool {
        for (min, max) in self.ranges.iter() {
            if value >= *min && value <= *max {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
struct ObfuscatedTicket {
    field_values: Vec<usize>,
}

impl ObfuscatedTicket {
    fn parse(string: &str) -> Self {
        ObfuscatedTicket {
            field_values: string
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

pub struct Day16 {}

impl<'a> Day<'a> for Day16 {
    type Input1 = Input<'a>;
    type Input2 = Input<'a>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 16;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().collect();
        let mut chunks = lines.split(|line| line.is_empty());
        let fields = chunks.next().unwrap();
        let your_ticket = chunks.next().unwrap();
        let nearby_tickets = chunks.next().unwrap();
        Input {
            fields: fields
                .iter()
                .map(|line| {
                    let mut parts = line.split(": ");
                    let name = parts.next().unwrap();
                    let field = Field::parse(parts.next().unwrap());
                    (name, field)
                })
                .collect(),
            your_ticket: ObfuscatedTicket::parse(your_ticket[1]),
            nearby_tickets: nearby_tickets[1..]
                .iter()
                .map(|ticket| ObfuscatedTicket::parse(ticket))
                .collect(),
        }
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let invalid_values: Vec<usize> = input
            .nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.field_values.iter())
            .filter(|&&value| !input.fields.values().any(|field| field.is_valid(value)))
            .copied()
            .collect();
        let valid_tickets: Vec<ObfuscatedTicket> = input
            .nearby_tickets
            .iter()
            .filter(|ticket| {
                !ticket
                    .field_values
                    .iter()
                    .any(|value| invalid_values.contains(value))
            })
            .cloned()
            .collect();
        let error_rate = invalid_values.iter().sum();
        (
            Input {
                nearby_tickets: valid_tickets,
                ..input
            },
            error_rate,
        )
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let mut field_possible_positions = HashMap::new();
        for (field_name, field) in input.fields.iter() {
            let valid_positions: Vec<_> = (0..input.fields.len())
                .filter(|&idx| {
                    input
                        .nearby_tickets
                        .iter()
                        .all(|ticket| field.is_valid(ticket.field_values[idx]))
                })
                .collect();
            field_possible_positions.insert(field_name, valid_positions);
        }
        let mut field_positions = HashMap::new();
        while field_positions.len() < input.fields.len() {
            for (field_name, possible_positions) in field_possible_positions.iter() {
                let possibilities: Vec<_> = possible_positions
                    .iter()
                    .filter(|idx| field_positions.values().find(|i| i == idx).is_none())
                    .copied()
                    .collect();
                if possibilities.len() == 1 {
                    field_positions.insert(field_name, possibilities[0]);
                    break;
                }
            }
        }
        field_positions
            .into_iter()
            .filter(|name_pos| name_pos.0.starts_with("departure"))
            .map(|name_pos| input.your_ticket.field_values[name_pos.1])
            .product()
    }
}
