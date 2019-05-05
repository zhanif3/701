cargo new bwt --bin

So:
Build suffix array
Create BWT array using suffix array
Perform search using


For the class project, I am most interested to implement the Burrows-Wheeler transform in a high performance language (in this case, Rust). In implementing this, I would focus on steps taken to improve the speed of computation over large DNA strings, and specifically seek to write about optimizations both algorithmic and implementation based used to improve the speed of the program. In addition to this, I would also implement search capabilities over the compressed string via a network interface via a cloud provider. From there, I would like to explore methods of compressing and sharing that transformed data, thereby providing a means for other users to easily transfer their datasets in a pre-transformed format. This has obvious applications for users who wish to serve their data over the aforementioned network interface.

I will be working alone, and have attempted to scope the project against that. If there is a desire for an expansion of scope, or cautions that would lead to a reduction in scope, I am happy to discuss.

from:
/Users/bxa005/masters/701/Project/rust/bwt
cargo build --release

target/release/generator
target/release/client
target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna

RUST_BACKTRACE=1 cargo run --bin generator /Users/bxa005/masters/701/Project/rust/bwt/src/testing_dna

cargo build --bin generator --release
sudo dtrace -c 'target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna' -o out.stacks -n 'profile-997 /execname == "generator"/ { @[ustack(100)] = count(); }'


sudo dtrace -c 'target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/1_000_000_dna ttcccg' -o out.stacks -n 'profile-997 /execname == "generator"/ { @[ustack(100)] = count(); }'

../../FlameGraph/stackcollapse.pl out.stacks | ../../FlameGraph/flamegraph.pl > pretty-graph.svg



// Optimizations
- Add Hashbrown
- Increase Rayon usage on heavy loops



time target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/13_000_000_dna ttccccgg

READ THE FILE: 0
BUILT VECTOR OF SUFFIXES: 0
BUILT SORTED THE VECTOR: 108
BUILT THE SUFFIX ARRAY: 109
MADE F: 1
MADE L: 1
MADE INDEX: 2
BUILT THE LF STRUCTURE: 5
BUILT THE ORIGINAL STRING: 2
BUILT ZIPPED: 0
ABA SHOULD BE SOME(2, 4): Some((12459498, 12459719))
You searched for: "ttccccgg".
There are 222 matches.
SIZEOF LF: FC; 5, F; 117000009, L; 117000009, Index; 234000018
target/release/generator  ttccccgg  116.73s user 1.61s system 100% cpu 1:58.23 total


WITH RAYON SORT:

READ THE FILE: 0
BUILT VECTOR OF SUFFIXES: 0
BUILT SORTED THE VECTOR: 39
BUILT THE SUFFIX ARRAY: 40
MADE F: 1
MADE L: 1
MADE INDEX: 2
BUILT THE LF STRUCTURE: 6
BUILT THE ORIGINAL STRING: 2
BUILT ZIPPED: 0
ABA SHOULD BE SOME(2, 4): Some((12459498, 12459719))
You searched for: "ttccccgg".
There are 222 matches.
SIZEOF LF: FC; 5, F; 117000009, L; 117000009, Index; 234000018
target/release/generator  ttccccgg  270.50s user 2.56s system 549% cpu 49.706 total


----


➜  bwt git:(master) ✗ time target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/14_000_000_dna ttccccgg
READ THE FILE: 0
BUILT VECTOR OF SUFFIXES: 0
BUILT SORTED THE VECTOR: 87
BUILT THE SUFFIX ARRAY: 87
MADE F: 2
MADE L: 1
MADE INDEX: 2
BUILT THE LF STRUCTURE: 6
BUILT THE ORIGINAL STRING: 3
BUILT ZIPPED: 0
ABA SHOULD BE SOME(2, 4): Some((13418088, 13418325))
You searched for: "ttccccgg".
There are 238 matches.
SIZEOF LF: FC; 5, F; 126000009, L; 126000009, Index; 252000018
target/release/generator  ttccccgg  650.80s user 3.17s system 665% cpu 1:38.28 total

➜  bwt git:(master) ✗ time target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/13_000_000_dna ttccccgg
READ THE FILE: 0
BUILT VECTOR OF SUFFIXES: 0
BUILT SORTED THE VECTOR: 37
BUILT THE SUFFIX ARRAY: 37
MADE F: 1
MADE L: 1
MADE INDEX: 2
BUILT THE LF STRUCTURE: 5
BUILT THE ORIGINAL STRING: 2
BUILT ZIPPED: 0
ABA SHOULD BE SOME(2, 4): Some((12459498, 12459719))
You searched for: "ttccccgg".
There are 222 matches.
SIZEOF LF: FC; 5, F; 117000009, L; 117000009, Index; 234000018
target/release/generator  ttccccgg  260.86s user 2.27s system 563% cpu 46.667 total

➜  bwt git:(master) ✗ time target/release/generator /Users/bxa005/masters/701/Project/rust/bwt/src/20_000_000_dna ttccccgg
READ THE FILE: 0
BUILT VECTOR OF SUFFIXES: 0
BUILT SORTED THE VECTOR: 1696
BUILT THE SUFFIX ARRAY: 1696
MADE F: 3
MADE L: 2
MADE INDEX: 3
BUILT THE LF STRUCTURE: 9
BUILT THE ORIGINAL STRING: 3
BUILT ZIPPED: 0
ABA SHOULD BE SOME(2, 4): Some((19167454, 19167801))
You searched for: "ttccccgg".
There are 348 matches.
SIZEOF LF: FC; 5, F; 180000009, L; 180000009, Index; 360000018
target/release/generator  ttccccgg  11548.85s user 66.50s system 679% cpu 28:30.53 total