use std::collections::HashMap;

// Note:
// 1. the linked list is FIFO, push to head, pop from the tail


struct Order {
  order_id: u64,
  is_buy: bool,
  shares: u64,
  limit: u64,
  next_order: Option<u64>,
  prev_order: Option<u64>,
  parent_limit: Option<u64>,
}

struct Limit {
  limit_price: u64,
  size: u64,
  total_vol: u64,
  parent: u64,
  order_count: u64,
  left_child: Option<u64>,
  right_child: Option<u64>,
  head_order: Option<u64>,
  tail_order: Option<u64>,
}

struct OrderBook {
  orders_ownership_map: HashMap<u64, Order>,
  limits_ownership_map: HashMap<u64, Limit>,
  limits_lookup_map: HashMap<u64, u64>,
  buy_tree_root: Option<u64>,
  sell_tree_root: Option<u64>,
}

fn add_order(ob: &mut OrderBook, mut order: Order) {
  let order_id = order.order_id;

  let limit_id_opt = ob.limits_lookup_map.get(&order.limit).copied();
  if limit_id_opt == None {
    // No limit node, create one

    add_to_tree();

    return
  }

  // Existing limit node, append to the list
  let limit_id = limit_id_opt.unwrap();
  let limit = ob.limits_ownership_map.get_mut(&limit_id).unwrap();

  order.parent_limit = Some(limit_id);
  order.next_order = limit.head_order.clone();
  order.prev_order = None;

  if limit.head_order != None {
    let order_id = limit.head_order.unwrap();
    let head_order = ob.orders_ownership_map.get_mut(&order_id).unwrap();
    head_order.prev_order = Some(order.order_id);
  } else {
    limit.tail_order = Some(order_id);
  }

  limit.head_order = Some(order_id);
  limit.order_count += 1;
  limit.size += order.shares;
  limit.total_vol += order.shares * limit.limit_price;

  ob.orders_ownership_map.insert(order_id, order);
}

fn drain_limit(shares_to_execute: u64, best_sell_limit: &mut Limit) -> (u64, bool) {
  // 1. while limit->tail_order != NULL and shares_to_execute > 0:
  // 1.1 let curr_shares = limit->tail_order->shares
  // 1.2 if curr_shares > shares_to_execute:
  // 1.2.1 limit->tail_order->shares = curr_shares - shares_to_execute
  // 1.2.2 shares_to_execute = 0 and break
  // 1.3 else:
  // 1.3.1 limit->tail_order = limit->tail_order->prev
  // 1.3.2 shares_to_execute -= limit->tail_order->shares
  // 1.3.3 remove tail_order and remove this order from hash map
  // 1.3.4 handle the head pointer edge cases when we approach one-node list (don't consider empty list because when it's empty we kick this limit out)
  // 1.4 if the limit's linked list is empty now, signal delete: delete this limit from tree and hash map (we don't delete immediately because we want to use it to find the predecessor in the parent function)
  // return shares_to_execute, delete
}

fn execute_buy_order(shares_to_execute: u64, expected_price: u64) {
  // 1. while shares_to_execute > 0 and best_sell_price is not NULL we do:
  // 1.1 Get the best sell Limit price best_sell_price (we keep track of this Limit object in the OrderBook struct so O(1)). Note this is always the smallest price in the sell tree, which is physically the leftmost node.
  // 1.2 If best_sell_price > expected price: break
  // 1.3 Execute as much as we can, call drain_limit(), update shares_to_execute
  // 1.4 If signalled to delete:
  // 1.4.1 Find its largest predecessor, set as the best_sell_price in OrderBook struct, delete the Limit
  // 1.5 If shares_to_execute == 0, means execution is done, return
  // 1.6 If shares_to_execute > 0, means we need to add this one to the buy tree, add the new Order at the Limit price, call add_order()
}

/* ---------------------------------------------------------------------- */
/* ------------------------ test helpers -------------------------------- */
/* ---------------------------------------------------------------------- */

fn format_order(o: &Order) -> String {
  let typ = if o.is_buy { "buy" } else { "sell" };
  return format!("<Order {}: {} {} shares at limit {}>", o.order_id, typ, o.shares, o.limit);
}

fn print_orders(ob: &OrderBook, head: Option<u64>, tail: Option<u64>) {
  let mut res = String::from("[");
  if head == None {
    let tail_id = tail.unwrap();
    let tail_order = ob.orders_ownership_map.get(&tail_id).unwrap();
    res.push_str(&format_order(tail_order));
    res.push_str("]");
    println!("{}", res);
    return
  }
  let mut head_p = head.clone();
  while head_p != None {
    let head_id = head_p.unwrap();
    let head_order = ob.orders_ownership_map.get(&head_id).unwrap();
    res.push_str(&format_order(head_order));
    head_p = head_order.next_order;
  }
  res.push_str("]");
  println!("{}", res);
}

fn print_prices_orders(ob: &OrderBook) {
  println!("prices and orders:");
  for (price, limit_id) in &ob.limits_lookup_map {
    let limit = ob.limits_ownership_map.get(&limit_id).unwrap();
    print_orders(ob, limit.head_order, limit.tail_order);
  }
}

fn main() {
  // u64 => Order
  let orders_ownership_map: HashMap<u64, Order> = HashMap::new();
  // u64 => Limit
  let limits_ownership_map: HashMap<u64, Limit> = HashMap::new();

  // limit_price => limit_id
  let limits_lookup_map: HashMap<u64, u64> = HashMap::new();

  let mut ob = OrderBook {
    orders_ownership_map, limits_ownership_map, limits_lookup_map
  };

  let order_1 = Order { order_id: 1, is_buy: true, shares: 10, limit: 500, next_order: None, prev_order: None, parent_limit: None };
  let order_2 = Order { order_id: 2, is_buy: false, shares: 20, limit: 400, next_order: None, prev_order: None, parent_limit: None };

  add_order(&mut ob, order_1);
  add_order(&mut ob, order_2);

  print_prices_orders(&ob);
}
