advent_of_code::solution!(9);

#[derive(Copy, Clone, Debug)]
struct Block {
    id: usize,
    pos: usize,
    size: usize,
}

#[derive(Clone, Debug)]
struct DiskMap {
    file_blocks: Vec<Block>,
    empty_blocks: Vec<Block>,
}

impl DiskMap {
    fn compact(&self) -> Self {
        let mut copy = self.clone();

        let mut file_blocks = Vec::new();

        let mut pos = 0;
        for file in self.file_blocks.iter() {
            let ceil_id = copy.file_blocks.last().unwrap().id;
            if file.id >= ceil_id {
                if file.id == ceil_id {
                    file_blocks.push(copy.file_blocks[file.id]);
                }
                break;
            }

            file_blocks.push(*file);
            pos += file.size;

            let empty = &mut copy.empty_blocks[file.id];
            while empty.size > 0 {
                let last = copy.file_blocks.last_mut().unwrap();
                let move_size = empty.size.min(last.size);
                file_blocks.push(Block {
                    id: last.id,
                    pos,
                    size: move_size,
                });
                empty.size -= move_size;
                last.size -= move_size;
                pos += move_size;
                if last.size == 0 {
                    copy.file_blocks.pop();
                }
            }
        }

        Self {
            file_blocks,
            empty_blocks: Vec::new(),
        }
    }

    fn compact2(&self) -> Self {
        let mut file_blocks = self.file_blocks.clone();
        let mut empty_blocks = self.empty_blocks.clone();

        for file in file_blocks.iter_mut().rev() {
            for empty in empty_blocks.iter_mut() {
                if empty.size >= file.size && file.pos > empty.pos {
                    file.pos = empty.pos;
                    empty.size -= file.size;
                    empty.pos += file.size;
                    break;
                }
            }
        }

        Self {
            file_blocks,
            empty_blocks,
        }
    }

    fn checksum(&self) -> usize {
        self.file_blocks
            .iter()
            .map(|&Block { id, pos, size }| (pos..pos + size).sum::<usize>() * id)
            .sum()
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let mut file_blocks = Vec::new();
        let mut empty_blocks = Vec::new();
        let mut pos = 0;
        for (i, c) in value.as_bytes().iter().enumerate() {
            let id = i / 2;
            let size = (c - b'0') as usize;
            if i % 2 == 0 {
                file_blocks.push(Block { id, pos, size });
            } else {
                empty_blocks.push(Block { id, pos, size });
            }
            pos += size;
        }

        Self {
            file_blocks,
            empty_blocks,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk: DiskMap = input.into();
    let ans = disk.compact().checksum();
    Some(ans)
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk: DiskMap = input.into();
    let ans = disk.compact2().checksum();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
