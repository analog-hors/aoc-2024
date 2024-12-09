use std::collections::{VecDeque, BTreeMap};

type DenseDisk = VecDeque<(Option<u64>, u64)>;
type SparseDisk = BTreeMap<u64, (Option<u64>, u64)>;

fn parse_disk_entries(input: &str) -> impl Iterator<Item=(Option<u64>, u64)> + '_ {
    input.chars()
        .enumerate()
        .map(|(i, c)| ((i % 2 == 0).then_some(i as u64 / 2), c.to_digit(10).unwrap() as u64))
}

fn parse_dense_disk(input: &str) -> DenseDisk {
    parse_disk_entries(input).collect()
}

fn parse_sparse_disk(input: &str) -> SparseDisk {
    let mut disk = SparseDisk::new();
    let mut index = 0;
    for (kind, size) in parse_disk_entries(input) {
        disk.insert(index, (kind, size));
        index += size;
    }
    disk
}

fn pop_last_used_from_dense(disk: &mut DenseDisk) -> Option<(u64, u64)> {
    while let Some((kind, size)) = disk.pop_back() {
        if let Some(id) = kind {
            return Some((id, size));
        }
    }
    None
}

fn compact_dense_by_block(mut disk: DenseDisk) -> DenseDisk {
    let mut compacted = DenseDisk::new();
    while let Some((kind, size)) = disk.pop_front() {
        if let Some(id) = kind {
            compacted.push_back((Some(id), size));
        } else if let Some((id, used_size)) = pop_last_used_from_dense(&mut disk) {
            if used_size < size {
                disk.push_front((None, size - used_size));
                compacted.push_back((Some(id), used_size));
            } else {
                disk.push_back((Some(id), used_size - size));
                compacted.push_back((Some(id), size));
            }
        }
    }
    compacted
} 

fn find_unused_entry_in_sparse(disk: &mut SparseDisk, min_size: u64) -> Option<(u64, u64)> {
    disk.iter()
        .find(|(_, &(kind, size))| kind.is_none() && size >= min_size)
        .map(|(&index, &(_, size))| (index, size))
}

fn compact_sparse_by_entry(mut disk: SparseDisk) -> SparseDisk {
    let mut compacted = SparseDisk::new();
    while let Some((index, (kind, size))) = disk.pop_last() {
        if let Some(id) = kind {
            if let Some((unused_index, unused_size)) = find_unused_entry_in_sparse(&mut disk, size) {
                disk.remove(&unused_index);
                compacted.insert(unused_index, (Some(id), size));
                if size < unused_size {
                    disk.insert(unused_index + size, (None, unused_size - size));
                }
            } else {
                compacted.insert(index, (kind, size));
            }
        }
    }
    compacted
}

fn dense_disk_checksum(disk: &DenseDisk) -> u64 {
    let mut index = 0;
    let mut checksum = 0;
    for &(kind, size) in disk {
        if let Some(id) = kind {
            checksum += (index..index + size).sum::<u64>() * id;
        }
        index += size;
    }
    checksum
}

fn sparse_disk_checksum(disk: &SparseDisk) -> u64 {
    let mut checksum = 0;
    for (&index, &(kind, size)) in disk {
        if let Some(id) = kind {
            checksum += (index..index + size).sum::<u64>() * id;
        }
    }
    checksum
}

fn part_1(input: String) -> u64 {
    let disk = parse_dense_disk(&input);
    let compacted = compact_dense_by_block(disk);
    dense_disk_checksum(&compacted)
}

fn part_2(input: String) -> u64 {
    let disk = parse_sparse_disk(&input);
    let compacted = compact_sparse_by_entry(disk);
    sparse_disk_checksum(&compacted)
}

aoc::main!();
