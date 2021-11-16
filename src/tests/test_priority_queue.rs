#[cfg(test)]
use {
    std::collections::BinaryHeap,
    crate::fraction::Fraction,
    crate::heap_element::MaxHeapElement,
};

#[test]
fn test_priority_queue(){
    let mut priority_queue = BinaryHeap::<MaxHeapElement>::new();
    let frac_half = Fraction{numerator : 1, denominator : 2};
    let frac_full = Fraction{numerator : 1, denominator : 1};
    let comm_1_half = MaxHeapElement{priority : frac_half, edge_id : 0, queue_id : 0};
    let comm_2_half = MaxHeapElement{priority : frac_half, edge_id : 1, queue_id : 1};
    priority_queue.push(comm_1_half.clone());
    priority_queue.push(comm_2_half.clone());
    let mut top = priority_queue.pop().unwrap();
    assert_eq!(top, comm_1_half);
    let comm_1_full = MaxHeapElement{priority : frac_full, edge_id : 0, queue_id : 0};
    priority_queue.push(comm_1_full.clone());
    top = priority_queue.pop().unwrap();
    assert_eq!(top, comm_2_half);
    let comm_2_full = MaxHeapElement{priority : frac_full, edge_id : 1, queue_id : 1};
    priority_queue.push(comm_2_full.clone());
    top = priority_queue.pop().unwrap();
    assert_eq!(top, comm_1_full);
    top = priority_queue.pop().unwrap();
    assert_eq!(top, comm_2_full);
}