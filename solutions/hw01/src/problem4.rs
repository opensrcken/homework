/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let num_moves: u32 = (2 ^ num_discs) - 1;
    let mut ret: Vec<Move> = Vec::new();

    for i in 1..num_moves {
        if i % 3 == 1 {
            ret.push((src, dst));
        } else if i % 3 == 2 {
            ret.push((src, aux));
        } else {
            ret.push((aux, dst));
        }
    }

    ret
}