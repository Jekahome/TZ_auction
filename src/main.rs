#![allow(dead_code)]

use std::collections::HashMap;

use id::Id;
use nft_lot::NFT;
use user::User;

mod id {
    use uid::Id as IdT;

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct T(());

    pub type Id = IdT<T>;
}

mod nft_lot {
    use crate::Id;

    #[derive(Debug)]
    pub struct NFT {
        uid: Id,
        user_id: Option<Id>,
    }
    // impl Copy for NFT{}
    impl Clone for NFT {
        fn clone(&self) -> Self {
            unimplemented!()
        }
    }

    impl NFT {
        pub fn new() -> Self {
            Self {
                uid: Id::new(),
                user_id: None,
            }
        }
        pub fn set_user(&mut self, id: Id) {
            self.user_id = Some(id);
        }
        pub fn get_uid(&self) -> Id {
            self.uid
        }
        pub fn get_user_uid(&self) -> Option<Id> {
            self.user_id
        }
    }
}

mod user {
    use crate::Id;
    #[derive(Debug)]
    pub struct User {
        uid: Id,
        balance: u128,
    }
    impl User {
        pub fn new(balance: u128) -> Self {
            Self {
                uid: Id::new(),
                balance,
            }
        }
        pub fn inc(&mut self, amount: u128) -> Option<u128> {
            if let Some(balance) = (self.balance).checked_add(amount) {
                self.balance = balance;
                return Some(amount);
            } else {
                unimplemented!()
            }
        }
        pub fn dec(&mut self, amount: u128) -> Option<u128> {
            if self.balance > amount {
                if let Some(balance) = self.balance.checked_sub(amount) {
                    self.balance = balance;
                    return Some(amount);
                } else {
                    unimplemented!()
                }
            }
            None
        }
        pub fn get_uid(&self) -> Id {
            self.uid
        }
        pub fn get_balance(&self) -> u128 {
            self.balance
        }
    }
}

struct Auction<'a> {
    lot: Id,
    owner_lot: &'a mut User,
    redemption_price: u128,
    reserve_price: u128,
    user_bets: HashMap<Id, u128>,

    win: Option<(Id, u128)>,
    is_finish: bool,
}

impl<'a> Auction<'a> {
    fn new(lot: Id, owner_lot: &'a mut User, redemption_price: u128, reserve_price: u128) -> Self {
        Self {
            lot: lot,
            owner_lot: owner_lot,
            redemption_price,
            reserve_price,
            user_bets: HashMap::new(),

            win: None,
            is_finish: false,
        }
    }

    pub fn get_bet(&mut self, user_id: Id) -> Option<u128> {
        if !self.is_finish {
            if self.user_bets.contains_key(&user_id) {
                let max_bet = self.get_max_bet().unwrap();
                if max_bet.0 != user_id {
                    return self.user_bets.remove(&user_id);
                }
            }
        }
        None
    }

    pub fn bet(&mut self, user_id: Id, bet: u128) -> bool {
        if !self.is_finish {
            if self.user_bets.contains_key(&user_id) {
                let max_bet = self.get_max_bet().unwrap();
                if max_bet.0 != user_id || (max_bet.0 == user_id && max_bet.1 < self.reserve_price)
                {
                    if let Some(x) = self.user_bets.get_mut(&user_id) {
                        *x += bet;
                        if *x >= self.redemption_price {
                            self.win = Some((user_id, *x));
                            self.finalize(self.owner_lot.get_uid());
                        }
                    }
                }
            } else {
                self.user_bets.insert(user_id, bet);
                if bet >= self.redemption_price {
                    self.win = Some((user_id, bet));
                    self.finalize(self.owner_lot.get_uid());
                }
            }
            return true;
        } else {
            println!("Time end");
        }
        false
    }

    pub fn get_lot(&mut self, user_id: Id, nft: &mut NFT) -> Option<u128> {
        if self.is_finish {
            if self.user_bets.contains_key(&user_id) {
                if let Some(win) = self.win {
                    if win.0 == user_id {
                        nft.set_user(user_id);
                    } else {
                        return Some(*self.user_bets.get(&user_id).unwrap());
                    }
                } else {
                    return Some(*self.user_bets.get(&user_id).unwrap());
                }
            }
        }
        None
    }

    pub fn finalize(&mut self, user_id: Id) {
        if self.owner_lot.get_uid() == user_id && !self.is_finish {
            self.is_finish = true;
            if self.win.is_none() {
                if self.user_bets.len() > 1 {
                    let win = self.get_max_bet().unwrap();
                    if win.1 >= self.reserve_price {
                        self.win = Some((win.0, win.1));
                    }
                } else if self.user_bets.len() == 1 {
                    let win = self
                        .user_bets
                        .drain()
                        .take(1)
                        .collect::<Vec<(Id, u128)>>()
                        .pop()
                        .unwrap();

                    if win.1 >= self.reserve_price {
                        self.win = Some(win);
                    }
                }
            }
            if let Some((_, amount)) = self.win {
                self.owner_lot.inc(amount);
            }
        }
    }

    fn get_max_bet(&self) -> Option<(Id, u128)> {
        self.user_bets
            .iter()
            .reduce(|a, b| if a.1 >= b.1 { a } else { b })
            .map(|e| (*e.0, *e.1))
    }
}

fn main() {
    println!("Run test:`cargo test`");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex_win_lot() {
        let mut user_1 = User::new(10000);
        let owner_id = user_1.get_uid();

        let mut nft = NFT::new();
        nft.set_user(user_1.get_uid());

        let mut user_2 = User::new(10000);
        let mut user_3 = User::new(10000);

        let mut auction = Auction::new(nft.get_uid(), &mut user_1, 2000, 500);

        if let Some(bet) = user_2.dec(500) {
            if !auction.bet(user_2.get_uid(), bet) {
                user_2.inc(bet);
            }
        }
        if let Some(bet) = user_3.dec(700) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
            }
        }

        auction.finalize(owner_id);

        if let Some(bet) = auction.get_lot(user_2.get_uid(), &mut nft) {
            user_2.inc(bet);
            assert!(true);
        } else {
            assert!(false);
        }

        assert_eq!(None, auction.get_lot(user_3.get_uid(), &mut nft));

        assert_eq!(user_1.get_balance(), 10700_u128);
        assert_eq!(user_2.get_balance(), 10000_u128);
        assert_eq!(user_3.get_balance(), 9300_u128);
        assert_eq!(nft.get_user_uid().unwrap().get(), user_3.get_uid().get());
    }

    #[test]
    fn test_ex_get_bet_fail() {
        let mut user_1 = User::new(10000);
        let owner_id = user_1.get_uid();

        let mut nft = NFT::new();
        nft.set_user(user_1.get_uid());

        let mut user_2 = User::new(10000);
        let mut user_3 = User::new(10000);

        let mut auction = Auction::new(nft.get_uid(), &mut user_1, 2000, 500);

        // user_2 bet 500
        if let Some(bet) = user_2.dec(500) {
            if !auction.bet(user_2.get_uid(), bet) {
                user_2.inc(bet);
            }
        }
        // user_3 bet 1000
        if let Some(bet) = user_3.dec(1000) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
            }
        }
        // user_2 bet 1000
        if let Some(bet) = user_2.dec(1000) {
            if !auction.bet(user_2.get_uid(), bet) {
                user_2.inc(bet);
            }
        }
        // user_3 bet 1000
        if let Some(bet) = user_3.dec(1000) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
            }
        }

        // total bet user_2 1500
        // total bet user_3 2000

        assert_eq!(None, auction.get_bet(user_3.get_uid()));

        auction.finalize(owner_id);

        if let Some(bet) = auction.get_lot(user_2.get_uid(), &mut nft) {
            user_2.inc(bet);
            assert!(true);
        } else {
            assert!(false);
        }

        assert_eq!(None, auction.get_lot(user_3.get_uid(), &mut nft));

        assert_eq!(user_1.get_balance(), 12000_u128);
        assert_eq!(user_2.get_balance(), 10000_u128);
        assert_eq!(user_3.get_balance(), 8000_u128);
        assert_eq!(nft.get_user_uid().unwrap().get(), user_3.get_uid().get());
    }

    #[test]
    fn test_bet_fail() {
        let mut user_1 = User::new(10000);
        let owner_id = user_1.get_uid();

        let mut nft = NFT::new();
        nft.set_user(user_1.get_uid());

        let mut user_2 = User::new(10000);
        let mut user_3 = User::new(10000);

        let mut auction = Auction::new(nft.get_uid(), &mut user_1, 2000, 500);

        // user_2 bet 500
        if let Some(bet) = user_2.dec(500) {
            if !auction.bet(user_2.get_uid(), bet) {
                user_2.inc(bet);
            }
        }
        // user_3 bet 1000
        if let Some(bet) = user_3.dec(1000) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
            }
        }
        // user_2 bet 1000
        if let Some(bet) = user_2.dec(1000) {
            if !auction.bet(user_2.get_uid(), bet) {
                user_2.inc(bet);
            }
        }
        // user_3 bet 1000
        if let Some(bet) = user_3.dec(1000) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
            }
        }

        // total bet user_2 1500
        // total bet user_3 2000

        assert_eq!(None, auction.get_bet(user_3.get_uid()));

        if let Some(bet) = user_3.dec(1000) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
                assert!(true);
            } else {
                assert!(false);
            }
        }

        auction.finalize(owner_id);

        if let Some(bet) = auction.get_lot(user_2.get_uid(), &mut nft) {
            user_2.inc(bet);
            assert!(true);
        } else {
            assert!(false);
        }

        assert_eq!(None, auction.get_lot(user_3.get_uid(), &mut nft));

        assert_eq!(user_1.get_balance(), 12000_u128);
        assert_eq!(user_2.get_balance(), 10000_u128);
        assert_eq!(user_3.get_balance(), 8000_u128);
        assert_eq!(nft.get_user_uid().unwrap().get(), user_3.get_uid().get());
    }

    #[test]
    fn test_reserve_price_fail() {
        let mut user_1 = User::new(10000);
        let owner_id = user_1.get_uid();

        let mut nft = NFT::new();
        nft.set_user(user_1.get_uid());

        let mut user_2 = User::new(10000);
        let mut user_3 = User::new(10000);

        let mut auction = Auction::new(nft.get_uid(), &mut user_1, 2000, 500);

        // user_2 bet 200
        if let Some(bet) = user_2.dec(200) {
            if !auction.bet(user_2.get_uid(), bet) {
                user_2.inc(bet);
            }
        }
        // user_3 bet 300
        if let Some(bet) = user_3.dec(300) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
            }
        }

        assert_eq!(None, auction.get_bet(user_3.get_uid()));

        if let Some(bet) = user_3.dec(100) {
            if !auction.bet(user_3.get_uid(), bet) {
                user_3.inc(bet);
                assert!(false);
            } else {
                assert!(true);
            }
        }

        // total bet user_2 200
        // total bet user_3 400

        auction.finalize(owner_id);

        if let Some(bet) = auction.get_lot(user_2.get_uid(), &mut nft) {
            user_2.inc(bet);
            assert!(true);
        } else {
            assert!(false);
        }

        if let Some(bet) = auction.get_lot(user_3.get_uid(), &mut nft) {
            user_3.inc(bet);
            assert!(true);
        } else {
            assert!(false);
        }

        assert_eq!(user_1.get_balance(), 10000_u128);
        assert_eq!(user_2.get_balance(), 10000_u128);
        assert_eq!(user_3.get_balance(), 10000_u128);
        assert_eq!(nft.get_user_uid().unwrap().get(), user_1.get_uid().get());
    }
}
