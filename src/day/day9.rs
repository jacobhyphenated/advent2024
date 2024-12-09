use super::Day;
use std::fs;

/// Day 9: Disk Fragmenter
/// 
/// The puzzle input is a list of integers such as:
/// ```
/// 12345
/// ```
/// 
/// The first value indicates the size of memory take up. The next value is how many empty blocks of memory,
/// followed by another block of used memory, etc. Each used block of memory has an id based on the order
/// it appears in the puzzle input. So if we write out each block with its id using `.` for empty:
/// ```
/// 0..111....22222
/// ```
/// 
/// Part 1: Move file blocks one at a time from the end of the memory list to the leftmost free memorty space.
/// Using the previous example, the end result would look like:
/// ```
/// 022111222......
/// ```
/// Calculate the file checksum by taking each memory location and multiplying the file id by the index in
/// the final memory array (empty spaces have a file id of 0)
/// 
/// Part 2: Instead of splitting files over different parts of the memory, keep the file id memory locations
/// contiguous. Starting with the highest file id and then decreasing, find the leftmost open memory space
/// that will accomodate the entire memory file. If there is none, then do not move that file.
/// Return the file checksum from the resulting memory array.
pub struct Day9;

/// Represents a block of memory.
/// `id` is the index value in the memory list.
/// None indicates an empty block of memory.
/// `space` represents how much memory this block takes up
#[derive(Debug, Copy, Clone)]
pub struct Mem {
    id: Option<i32>,
    space: i32,
}

impl Day<Vec<Mem>> for Day9 {
    fn read_input() -> Vec<Mem> {
        let input = fs::read_to_string("resources/day9.txt").expect("file day9.txt not found");
        parse_input(&input)
    }

    /// Go from left to right, and fill in all empty memory spaces from the end of the mem list.
    /// Truncate the mem list of trailing empty memory blocks as we go.
    fn part1(input: &Vec<Mem>) -> impl std::fmt::Display {
        let mut memory = input.clone();
        let mut search_idx = 1;

        while search_idx < memory.len() {
            if memory.last().unwrap().id.is_none() {
                memory.remove(memory.len() - 1);
                continue;
            }

            // search idx should only point at None (empty memory) values
            if memory[search_idx].id.is_some() {
                search_idx += 1;
                continue;
            }

            let end_memory = *memory.last().unwrap();
            let current = memory[search_idx];
            if current.space > end_memory.space {
                let moved = end_memory;
                let remaining = Mem {
                    id: None,
                    space: current.space - end_memory.space
                };
                memory.remove(search_idx);
                memory.insert(search_idx, remaining);
                memory.insert(search_idx, moved);
                memory.remove(memory.len() - 1);
            } else {
                let moved = Mem { id: end_memory.id, space: current.space };
                memory.remove(search_idx);
                memory.insert(search_idx, moved);
                memory.remove(memory.len() - 1);
                if current.space != end_memory.space {
                    memory.push(Mem {
                        id: end_memory.id,
                        space: end_memory.space - current.space
                    });
                }
            }
            search_idx += 1;
        }

        let mut idx: i64 = 0;
        let mut sum: i64 = 0;
        for mem in memory {
            let Some(mem_idx) = mem.id else {
                panic!("Invalid Empty Memory block in final array");
            };
            for _ in 0 .. mem.space {
                sum += idx * mem_idx as i64;
                idx += 1;
            }
        }
        sum
        
    }

    // Go from right to left, no truncation, so there will be empty memory blocks in the final result
    fn part2(input: &Vec<Mem>) -> impl std::fmt::Display {
        let mut memory = input.clone();
        let mut end_ptr = memory.len() - 1;

        while end_ptr > 0{
            if memory[end_ptr].id.is_none() {
                end_ptr -= 1;
                continue;
            }
            let empty = &memory[.. end_ptr].iter()
                .enumerate()
                .find(|(_, mem)| mem.id.is_none() && mem.space >= memory[end_ptr].space);
            if let Some((empty_idx, empty)) = empty {
                // copy to avoid memory borrow
                let empty = **empty;
                let empty_ptr = *empty_idx;

                memory[empty_ptr] = memory[end_ptr];
                memory[end_ptr].id = None;
                if empty.space > memory[end_ptr].space {
                    memory.insert(empty_ptr + 1, Mem {
                        id: None,
                        space: empty.space - memory[end_ptr].space,
                    });
                }
            }
            end_ptr -= 1;
        }

        let mut idx: i64 = 0;
        let mut sum: i64 = 0;
        for mem in memory {
            if let Some(mem_idx) = mem.id {
                for _ in 0 .. mem.space {
                    sum += idx * mem_idx as i64;
                    idx += 1;
                }
            } else {
                idx += mem.space as i64;
            }
        }
        sum
    }

}


fn parse_input(input: &str) -> Vec<Mem> {
    let ints = input.chars().map(|c| 
        c.to_digit(10)
            .unwrap()
            .try_into().unwrap()
        )
        .collect::<Vec<i32>>();
    let mut idx = 0;
    let mut empty = false;
    let mut memory = Vec::new();
    for i in ints {
        memory.push(Mem {
            id: if empty { None } else { Some(idx) },
            space: i,
        });
        if !empty {
            idx += 1;
        }
        empty = !empty;
    }
    memory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input("2333133121414131402");
        let result =  Day9::part1(&input);
        assert_eq!("1928", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input("2333133121414131402");
        let result =  Day9::part2(&input);
        assert_eq!("2858", result.to_string())
    }

}
