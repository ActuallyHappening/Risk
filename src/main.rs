use std::collections::HashMap;

use risk::app_tracing;
use risk::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Army {
	pub six: u8,
	pub eight: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct DiceChoice {
	six: u8,
	eight: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum AttackingDice {
	Finished,
	Attack(DiceChoice),
}

impl Army {
	pub fn attacking_dice(&self) -> Option<DiceChoice> {
		if self.six + self.eight < 1 {
			return None;
		}

		todo!()
	}
}

type Prob = num::rational::Ratio<u64>;

#[derive(Debug, Clone, Copy)]
pub struct State {
	pub turn: u8,
	pub attacker: Army,
	pub defender: Army,
}

pub struct Turn(HashMap<State, Prob>);

impl Turn {
	pub fn advance(mut self) -> Self {
		todo!()
	}
}

fn main() -> color_eyre::Result<()> {
	app_tracing::install_tracing("info,risk=trace")?;
	trace!("Started tracing");

	Ok(())
}
