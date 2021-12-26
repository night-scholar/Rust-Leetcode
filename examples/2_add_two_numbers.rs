pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}


#[allow(dead_code)]
impl ListNode {
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
#[allow(dead_code)]
struct Solution{

}
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1,mut l2) = (l1,l2);
        let mut listHead = Some(Box::new(ListNode::new(0)));
        let mut listTail = &mut listHead;
        let (mut l1_end,mut l2_end,mut overflow) = (false,false,false);

        loop{
            let l1Val = match l1{
                None => {
                    l1_end = true;
                    0
                } 
                Some(node) =>{
                    l1 = node.next;
                    node.val
                } 
            };

            let l2Val = match l2{
                None => {
                    l2_end = true;
                    0
                }
                Some(node) =>{
                    l2 = node.next;
                    node.val
                } 
            };

            if l1_end && l2_end && !overflow{
                break listHead.unwrap().next;
            }

            let mut sum = l1Val+l2Val+if overflow {1}else {0};
            if sum >= 10{
                overflow = true;
                sum = sum -10;
            }else{
                overflow = false;
            }
            listTail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            listTail = &mut listTail.as_mut().unwrap().next;
        }
    }
}

fn main(){
    
}