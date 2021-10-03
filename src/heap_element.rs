use std::cmp::Ordering;
use std::cmp;
use crate::fraction::Fraction;
use crate::network::EdgeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapElement{
    pub priority : Fraction,
    pub edge_id : EdgeId,
    pub queue_id : usize,
}

impl cmp::PartialOrd for HeapElement{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert_ne!(self.edge_id, other.edge_id, "Error: edge ids should never be equal");
        assert_ne!(self.queue_id, other.queue_id, "Error: queue ids should never be equal");
        if self.priority != other.priority{
            // higher priority means less, i.e. first
            match self.priority > other.priority {
                true => Some(Ordering::Less),
                false => Some(Ordering::Greater),
            }
        }
        else{
            match self.edge_id < other.edge_id{
                true => Some(Ordering::Less),
                false => Some(Ordering::Greater), 
            }
        }
    }
}

impl Ord for HeapElement{
    fn cmp(&self, other : &HeapElement) -> Ordering{
        let ord = self.partial_cmp(other).unwrap();
        match ord {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => ord
        }
    }
}