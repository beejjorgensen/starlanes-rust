use super::*;

impl StarLanes {
    /// Determine the largest neighbor company.
    ///
    /// In order for this to behave like the original game, this depends
    /// on neighbor_count() assessing neighbors in the order N, S, E, W.
    /// Yes, we could sort them, but it would be a pain in the butt.
    /// Maybe have it return a hash instead of an array?
    fn get_largest_neighbor_company(&self, neighbors: &NeighborCounts) -> usize {
        let mut max_size: u64 = 0;
        let mut max_size_co_num: usize = 0;

        for Point(r, c) in &neighbors.companies {
            if let MapCell::Company(co_num) = self.map.get(*r, *c) {
                let company_size = self.companies[co_num as usize].size;

                if company_size > max_size {
                    max_size = company_size;
                    max_size_co_num = co_num as usize;
                }
            } else {
                panic!("merge: expected a company at {},{}", *r, *c);
            }
        }

        max_size_co_num
    }

    /// Compute total shares held by all players for a specific company.
    fn total_shares_outstanding(&self, co_num: usize) -> u64 {
        if !self.companies[co_num].in_use {
            panic!("total_shares_outstanding: company {co_num} is not in use");
        }

        let mut total: u64 = 0;

        for p in &self.players {
            total += p.get_holdings(co_num) as u64;
        }

        total
    }

    /// Compute the MergeInfo struct for a given player.
    fn get_merge_info(&self, player: &Player, smaller_co: usize) -> MergeInfo {
        let old_stock = player.get_holdings(smaller_co);
        // New stock is old_stock divided by conversion factor
        // rounded to nearest integer.
        let new_stock =
            (old_stock + DEFAULT_MERGE_SHARE_CONVERSION / 2) / DEFAULT_MERGE_SHARE_CONVERSION;

        let total_shares = self.total_shares_outstanding(smaller_co);
        let shares_held = old_stock;
        let smaller_co_price = self.companies[smaller_co].share_price;

        let bonus_paid = DEFAULT_MERGE_BONUS_FACTOR * smaller_co_price as i64 * shares_held
            / total_shares as i64;

        MergeInfo {
            old_stock,
            new_stock,
            bonus_paid,
        }
    }

    /// Merge companies.
    pub(crate) fn merge(
        &mut self,
        move_point: Point,
        neighbors: &NeighborCounts,
        events: &mut Vec<Event>,
    ) {
        let biggest_co_num = self.get_largest_neighbor_company(neighbors);

        // Loop through all possible mergee companies. This assumes the
        // companies are in N, S, E, W order in the NeighborCounts
        // struct in order to match the original game.
        for Point(cr, cc) in &neighbors.companies {
            let mut merge_info = Vec::new();

            let company = if let MapCell::Company(c) = self.map.get(*cr, *cc) {
                c as usize
            } else {
                panic!("merge: expected to find a company at that cell")
            };

            if company == biggest_co_num {
                continue; // This is the merger, not a mergee.
            }

            // Get merge info for all players
            let player_merge_info: Vec<_> = self
                .players
                .iter()
                .map(|p| self.get_merge_info(p, company))
                .collect();

            // Run through all the players computing and adding their
            // bonuses.
            for (p, mi) in self.players.iter_mut().zip(player_merge_info) {
                p.add_holdings_signed(biggest_co_num, mi.new_stock);
                p.add_cash(mi.bonus_paid);

                merge_info.push(mi);
            }

            let event = Event::Merge(biggest_co_num, company, merge_info);
            events.push(event);

            // Convert all map spaces and mark company not in use
            self.map.convert(company, biggest_co_num);
            self.companies[company].in_use = false;

            // Add company sizes and prices
            self.companies[biggest_co_num].size += self.companies[company].size;
            self.companies[biggest_co_num].share_price += self.companies[company].share_price;

            // Stock split check
            self.stock_split(biggest_co_num, events);
        }

        // Put player move on the map
        let Point(move_r, move_c) = move_point;
        self.map
            .set(move_r, move_c, MapCell::Company(biggest_co_num as u32));

        // The old game didn't do this, but there should be 1 more added
        // to the company size after the merge wraps up
        //self.companies[biggest_co_num].size += 1;
    }
}
