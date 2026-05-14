// ## Day 10 - `VecDeque` and `BinaryHeap`
//
// - [ ] Focus: queues and priority queues.
//   - Detail: Queues and priority queues solve problems that plain vectors can solve awkwardly but not cleanly. The focus is recognizing when insertion/removal patterns matter enough to pick a specialized container.
// - [ ] Read/inspect:
//   - What to look for: Look for operations at the front/back of `VecDeque` and push/pop behavior in `BinaryHeap`. Pay attention to whether the heap is max-oriented and how that affects top-N problems.
//   - `std::collections::VecDeque`
//   - `std::collections::BinaryHeap`
// - [ ] Exercise:
//   - Goal: The point is to model data that changes at the ends or needs priority access. Comparing heap versus sorting helps you decide when specialized data structures are worth the extra concept.
//   - Implement a fixed-size recent-events buffer using `VecDeque`.
//   - Implement top-N largest numbers using `BinaryHeap` or sorting; compare the approaches.
// - [ ] Done when:
//   - You know where to reach for queue-like behavior without abusing `Vec`.

fn main() {}
