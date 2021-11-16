use std::cmp::Ordering;
use std::cmp;
use crate::fraction::Fraction;
use crate::network::EdgeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxHeapElement{
    // small priority means the packet is first
    pub priority : Fraction,
    pub edge_id : EdgeId,
    pub queue_id : usize,
}

impl cmp::PartialOrd for MaxHeapElement{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert_ne!(self.edge_id, other.edge_id, "Error: edge ids should never be equal");
        assert_ne!(self.queue_id, other.queue_id, "Error: queue ids should never be equal");
        if self.priority != other.priority{
            // higher priority means less, i.e. first
            // TODO: check, test
            match self.priority < other.priority {
                true => Some(Ordering::Greater),
                false => Some(Ordering::Less),
            }
        }
        else{
            match self.edge_id < other.edge_id{
                true => Some(Ordering::Greater),
                false => Some(Ordering::Less), 
            }
        }
    }
}

impl Ord for MaxHeapElement{
    fn cmp(&self, other : &MaxHeapElement) -> Ordering{
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord
        }
    }
}