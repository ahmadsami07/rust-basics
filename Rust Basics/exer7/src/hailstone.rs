pub fn hailstone(n: u64) -> u64 {
    if n % 2 == 0 {
        // println!("{}", n / 2);
        return n / 2;
    } else {
        // println!("{}", 3 * n + 1);
        return 3 * n + 1;
    }
}

// Relative speed of hailstone_sequence_prealloc is higher.
// In case of n=7 test,
// hailstone_sequence_append mean speed was 278.11 ns per iteration,
// while hailstone_sequence_prealloc mean speed was 96.543 ns per iteration.
// Therefore relative speed for hailstone_sequence_prealloc to hailstone_sequence_append is 96.543-278.11=-181.567 ns per iteration.
// Similar trend is followed in n=162964 and n=686901248.

pub fn hailstone_sequence_append(n: u64) -> Vec<u64> {
    let mut resVec: Vec<u64> = Vec::new();
    let mut hailstoneSeqNumber: u64 = n;

    while hailstoneSeqNumber != 1 {
        resVec.push(hailstoneSeqNumber);
        let newHailstoneSeq: u64 = hailstone(hailstoneSeqNumber);
        hailstoneSeqNumber = newHailstoneSeq;
    }

    resVec.push(1);

    return resVec;
}

pub fn hailstone_sequence_prealloc(n: u64) -> Vec<u64> {
    let mut allocationLength: usize = 0;
    let mut hailstoneSeqNumber: u64 = n;

    while hailstoneSeqNumber != 1 {
        allocationLength += 1;
        hailstoneSeqNumber = hailstone(hailstoneSeqNumber);
    }
    allocationLength += 1;

    let mut resVec: Vec<u64> = Vec::with_capacity(allocationLength);

    hailstoneSeqNumber = n;
    while hailstoneSeqNumber != 1 {
        resVec.push(hailstoneSeqNumber);
        let newHailstoneSeq: u64 = hailstone(hailstoneSeqNumber);

        hailstoneSeqNumber = newHailstoneSeq;
    }
    resVec.push(1);

    return resVec;
}
