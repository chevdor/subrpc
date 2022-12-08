use serde::{Deserialize, Serialize};

#[allow(clippy::derive_hash_xor_eq)]
/// Simple stats to help picking the best endpoint
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct EndpointStats {
	pub failures: u16,
	pub success: u16,
	pub latency: f32,
}

impl std::hash::Hash for EndpointStats {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.failures.hash(state);
		self.success.hash(state);
	}
}

impl EndpointStats {
	pub fn add(&mut self, state: bool, latency: Option<f32>) {
		if state {
			self.success += 1;
			if let Some(l) = latency {
				self.latency = (l * self.success as f32 + l) / self.success as f32;
			}
		} else {
			self.failures += 1;
		}
	}

	pub fn score(&self) -> f32 {
		(self.success - self.failures) as f32 * 1f32 / self.latency / 10f32
	}
}
