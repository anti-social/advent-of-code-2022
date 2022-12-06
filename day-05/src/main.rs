use std::fs::read_to_string;

use anyhow::Result as AnyResult;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, line_ending, satisfy, u32};
use nom::combinator::map;
use nom::combinator::value;
use nom::multi::{many1, separated_list1, separated_list0};
use nom::sequence::{delimited, terminated, tuple, preceded};

fn main() -> AnyResult<()> {
    let input = read_to_string("day-05/input.txt")?;

    println!("{}", solve1(&input)?);
    println!("{}", solve2(&input)?);

    Ok(())
}

fn solve1(input: &str) -> AnyResult<String> {
    let (actions_input, mut stacks) = parse_crate_stacks(input).unwrap();
    let (rest, moves) = parse_moves(actions_input).unwrap();
    assert_eq!(rest.trim(), "");

    for mv in moves.iter() {
        for _ in 0..mv.num_crates {
            let c = stacks.get_mut(mv.src as usize - 1).unwrap().pop().unwrap();
            stacks.get_mut(mv.dst as usize - 1).unwrap().push(c);
        }
    }

    let top = stacks.iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();

    Ok(top)
}

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(
            delimited(tag("["), anychar, tag("]")),
            Some
        ),
        value(None, tag("   "))
    ))(input)
}

fn parse_layer(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_crate)(input)
}

fn parse_layer_nums(input: &str) -> IResult<&str, ()> {
    value(
        (),
        separated_list1(
            tag(" "),
            delimited(
                tag(" "),
                satisfy(|c| c.is_digit(10)),
                tag(" ")
            )
        )
    )(input)
}

fn parse_layers(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    terminated(
        many1(
            terminated(parse_layer, line_ending)
        ),
        tuple((parse_layer_nums, line_ending, line_ending))
    )(input)
}

fn parse_crate_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(
        parse_layers,
        |layers| {
            let mut stacks: Vec<Vec<char>> = vec!();
            for layer in layers.iter().rev() {
                for (stack_ix, c) in layer.iter().enumerate() {
                    if let Some(c) = c {
                        match stacks.get_mut(stack_ix) {
                            Some(stack) => stack.push(*c),
                            None => stacks.push(vec!(*c)),
                        }
                    }
                }
            }
            stacks
        }
    )(input)
}



fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            delimited(
                tuple((tag("move"), tag(" "))),
                u32,
                tag(" ")
            ),
            delimited(
                tuple((tag("from"), tag(" "))),
                u32,
                tag(" ")
            ),
            preceded(
                tuple((tag("to"), tag(" "))),
                u32,
            ),
        )),
        |(num_crates, src, dst)| Move { num_crates, src, dst }
    )(input)
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list0(line_ending, parse_move)(input)
}

#[derive(Debug)]
struct Move {
    num_crates: u32,
    src: u32,
    dst: u32,
}

fn solve2(input: &str) -> AnyResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{solve1, solve2};

    const INPUT: &str = indoc!(r#"
        [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "#);

    #[test]
    fn test1() {
        println!("{}", INPUT);
        assert_eq!(
            solve1(INPUT).unwrap(),
            "CMZ"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve2(INPUT).unwrap(),
            8u32
        );
    }
}
