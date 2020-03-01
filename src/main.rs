use std::collections::BTreeSet;
use itertools::Itertools;
use std::cmp::Reverse;

fn main() {
    let free = [
        "A1",
        "A2",
        "A3",
        "A4",
        "A8",
        "A9",
        "A10",
        "A15",
        "B0",
        "B1",
        "B2",
        "B4",
        "B5",
        "B8",
        "B9",
        "B10",
        "B11",
        "B12",
        "B13",
        "B14",
        "B15",
        "C0",
        "C1",
        "C2",
        "C3",
        "C6",
        "C7",
        "C8",
        "C9",
        "C10",
        "C11",
        "C12",
        "C13",
        "D0",
        "D1",
        "D2",
        "D3",
        "D4",
        "D5",
        "D6",
        "D7",
        "D8",
        "D9",
        "D10",
        "D11",
        "D12",
        "D13",
        "D14",
        "D15",
        "E6",
        "E7",
        "F2",
        "F4",
        "F6",
        "F9",
        "F10",
    ];
    let free: BTreeSet<_> = free.iter().copied().collect();

    let row1 = [
        "GND",
        "C1",
        "C3",
        "A1",
        "A3",
        "F4",
        "A5",
        "A7",
        "C5",
        "B1",
        "E7",
        "E9",
        "E11",
        "E13",
        "E15",
        "B11",
        "B13",
        "B15",
        "D9",
        "D11",
        "D13",
        "D15",
        "C6"
    ];
    let row2 = [
        "NRST",
        "C0",
        "C2",
        "F2",
        "A0",
        "A2",
        "A4",
        "A6",
        "C4",
        "B0",
        "B2",
        "E8",
        "E10",
        "E12",
        "E14",
        "B10",
        "B12",
        "B14",
        "D8",
        "D10",
        "D12",
        "D14",
        "C7",
    ];
    let row3 = [
        "F9",
        "F0",
        "C14",
        "E6",
        "E4",
        "E2",
        "E0",
        "B8",
        "BOOT0",
        "B6",
        "B4",
        "D7",
        "D5",
        "D3",
        "D1",
        "C12",
        "C10",
        "A14",
        "F6",
        "A12",
        "A10",
        "A8",
        "C8",
    ];
    let row4 = [
        "F10",
        "F1",
        "C15",
        "C13",
        "E5",
        "E3",
        "E1",
        "B9",
        "VDD",
        "B7",
        "B5",
        "B3",
        "D6",
        "D4",
        "D2",
        "D0",
        "C11",
        "A15",
        "A13",
        "A11",
        "A9",
        "C9",
        "NC",
    ];

    let rows = [
        row1,
        row2,
        row3,
        row4,
    ];
    
    let runs = rows
        .iter()
        .flat_map(|row| row
            .iter()
            .copied()
            .group_by(|pin| free.contains(pin))
            .into_iter()
            .filter(|(is_free, _)| *is_free)
            .map(|(_, run)| run.collect_vec())
            .collect_vec()
        )
        .sorted_by_key(|run| Reverse(run.len()));

    for run in runs {
        println!("[{}] {:?}", run.len(), run);
    }
}