/*

ADA Staking CLI Tool (Not Offical or Investment Advice ⚠️)
Copyright (C) 2022  zulrah <email none of your business 🤣>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.


*/

use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
struct StakedCardanoPool {
    ada: f64,                  // Total amount of ADA which uses 6 decimal places can be changed in the source code format options
    fetch_price_via_api: bool, // Optional fetch price from a trusted API currently not implemented
    initial_price: f64, // Starting price in USD
    price_yield: f64,  // Daily average increase in price of ADA 1% is a good conservative number
    annual_yield: f64, // Expressed as a fraction for example 5% is 0.05
    epoch_in_days: u16, // How many days before a payout happens this is fixed by ADA currently 5 days but can be changed for future purposes
    years_holding: u8, // How many years will it be staked
}

#[derive(Debug, Default)]
struct StakedCardanoPoolResult {
    final_ada_amount: f64,
    final_ada_price: f64,
}

impl StakedCardanoPoolResult {
    fn new(final_ada_amount: f64, final_ada_price: f64) -> Self {
        StakedCardanoPoolResult {
            final_ada_amount: final_ada_amount,
            final_ada_price: final_ada_price,
        }
    }

    fn total(&self) -> f64 {
        self.final_ada_amount * self.final_ada_price
    }

    fn yield_as_percentage(&self, pool_info : &StakedCardanoPool) -> f64 {
          ((pool_info.initial_price * pool_info.ada) / self.total()) * 100.0
    }
}

fn calculate_staked_pool(pool: &StakedCardanoPool) -> StakedCardanoPoolResult {
    let mut ada = pool.ada;
    let mut price = pool.initial_price;
    let days = ((pool.years_holding as f64) * 365.25) as u16 + 1;
    let epochs_per_year =  365.25 / (pool.epoch_in_days as f64);
    let mut ada_per_year = ada * pool.annual_yield;
    println!("Initial ADA Per Year (Excluding Compounding Interest): {}", ada_per_year);
    println!("Day 0: {} ADA @ ${:.2} = ${:.2}", ada, price, ada * price);
    for day in 1..days { 
        if day > 0 && (day % pool.epoch_in_days) == 0 {
            ada += ada_per_year / epochs_per_year;
            ada_per_year = ada * pool.annual_yield;
            price *= pool.price_yield; // Increase price by average positive change no point in calculating a downard trend but you may use less than 1
            println!("Day {}: {} ADA @ ${:.2} = ${:.2} [Pay Day: Yes]", day, ada, price, ada * price);
        }
        else {
            price *= pool.price_yield; // Increase price by average positive change no point in calculating a downard trend but you may use less than 1
            println!("Day {}: {} ADA @ ${:.2} = ${:.2} [Pay Day: No]", day, ada, price, ada * price);
        }
        
    }
    StakedCardanoPoolResult::new(ada, price)
}

fn main() {
    if let Ok(buffer) = read_to_string("pool.json") {
        let pool_info: StakedCardanoPool = serde_json::from_str(&buffer).unwrap();
        let result = calculate_staked_pool(&pool_info);
        println!("Final Result: {} ADA @ ${:.2} = ${:.2} Gainz: {:.2}%", result.final_ada_amount, result.final_ada_price, result.total(), 100.0 + result.yield_as_percentage(&pool_info));

    } else {
        println!("Failed to find pool.json in current working directory!");
    }
}
