use std::iter;
use std::time::Instant;
use std::collections::HashSet;
use uinput;
use {Result as Res, Error};
use input;
use config::{Binding, Group, group};
use util::iter;
use super::{util, Button};

#[derive(Debug)]
pub struct TriggerRight<'a> {
	normal: Option<&'a Group>,
	shift:  Option<&'a Group>,

	shifted: bool,
}

impl<'a> TriggerRight<'a> {
	pub fn load(normal: Option<&'a Group>, shift: Option<&'a Group>) -> Res<TriggerRight<'a>> {
		Ok(TriggerRight {
			normal: normal,
			shift:  shift,

			shifted: false,
		})
	}

	pub fn group(&self) -> Option<&Group> {
		if self.shifted {
			self.shift
		}
		else {
			self.normal
		}
	}

	pub fn bindings(&self) -> Option<&group::Bindings> {
		self.group().map(|g| &g.bindings)
	}
}

impl<'a> Button for TriggerRight<'a> {
	fn button(&mut self, device: &mut uinput::Device, _at: Instant, button: input::Button, press: bool) -> Res<HashSet<&Binding>> {
		let bindings = if let Some(bindings) = self.bindings() {
			match bindings {
				&group::Bindings::Trigger { ref click } => {
					match button {
						input::Button::TriggerRight => iter(click.iter().flat_map(|b| b.iter())),
						_                          => unreachable!(),
					}
				}

				_ =>
					return Err(Error::NotSupported)
			}
		}
		else {
			iter(iter::empty())
		};

		util::button(device, bindings, press)
	}
}
