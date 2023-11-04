use std::ptr::NonNull;

struct Order {
  id: u64,
  is_buy: bool, // true for buy, false for sell
  shares: u64,
  limit: u64,
  entry_time: u64,
  event_time: u64,
  next_order: Option<NonNull<Order>>,
  prev_order: Option<NonNull<Order>>,
  parent_limit: Option<NonNull<Limit>>,
}

impl Order {
  fn push_order(limit Option<NonNull<Limit>>, new_order Option<NonNull<Order>>) -> u8 {
    if limit.is_none() || new_order.is_none() {
      return 0
    }
    

  }
}

/*  
push_order(limit, new_order)
1. check limit->limit_price != new_order->limit
2. update new_order's parent limit to be limit
3. add the new_order to the front of limit->head_order
4. increment limit->order_count by 1
5. update limit->size by new_order->shares
6. update limit->total_volume by new_order->shares * limit->limit_price
*/

/*
pop_order(limit)
1. check limit->tail_order != NULL (the linked list is not 0 len)
2. remove the tail_order and update limit->{size, total_volume}
3. return the poped order
*/

/*
remove_order(order)
1. remove it from somewhere in the list
*/

/*
create_root() -> Limit
1. create Limit
2. set limit->limitPrice = -inf
3. return limit
*/

/*
** add new Limit to the limit tree
add_new_limit(root: Limit*, limit: Limit*)
1. assert: limit should not exist
2. set limit's left and right children to NULL
3. 
*/