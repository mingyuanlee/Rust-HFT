use std::collections::HashMap;

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
}

fn add_order(ob &mut OrderBook, order Order) {
  let limit_id_opt = ob.limits_lookup_map.get(&order.limit);
  if limit_id_opt == None {
    // No limit node, create one
    return
  }
  // Existing limit node, append to the list
  let limit_id = limit_id_opt.unwrap();
  let limit = ob.limits_lookup_map.get(&limit_id).unwrap();

  order.parent_limit = limit_id;
  order.next_order = limit.head_order.clone();
  new_order.prev_order = None;

  if limit.head_order != None {
    let order_id = limit.head_order.unwrap();
    let head_order = ob.orders_ownership_map.get(&order_id).unwrap();
    head_order.prev_order = Some(order.order_id);
  } else {
    limit.tail_order = Some(order.order_id);
  }

  limit.head_order = Some(order.order_id);
  limit.order_count += 1;
  limit.size += order.shares;
  limit.total_vol += order.shares * limit.limit_price;
}

fn main() {
  // u64 => Order
  let mut orders_ownership_map: HashMap<u64, Order> = HashMap::new();
  // u64 => Limit
  let mut limits_ownership_map: HashMap<u64, Limit> = HashMap::new();

  // limit_price => limit_id
  let mut limits_lookup_map: HashMap<u64, u64> = HashMap::new();

  let mut ob = OrderBook {
    orders_ownership_map, limits_ownership_map, limits_lookup_map
  };

  let mut order_1 = Order {
    order_id: 1,
    is_buy: true,
    shares: 10,
    limit: 500,
    next_order: None,
    prev_order: None,
    parent_limit: None,
  }

  add_order(mut &ob, order_1);
}
