/*
 * @author	Antoine "Anthony" Louis Thibaut Sébert
 * @date	03/03/2019
 */

// https://wiki.osdev.org/Loading_a_Process
// https://wiki.osdev.org/Multiprocessor_Scheduling
// https://wiki.osdev.org/Scheduling_Algorithms

pub mod admitter;
pub mod dispatcher;
pub mod swapper;

use spin::RwLock;
use array_init::array_init;
use super::process::Task;
use arraydeque::ArrayDeque;
use core::{ptr::null_mut, sync::atomic::AtomicPtr};
use lazy_static::lazy_static;
use spin::Mutex;

// should be replaced by a set
lazy_static! {
	pub static ref PROCESS_TABLE: [RwLock<Option<Task>>; 256] = { array_init(|_| RwLock::new(None)) };
}

lazy_static! {
	pub static ref READY_QUEUE: Mutex<ArrayDeque<[Task; 256]>> = Mutex::new(ArrayDeque::new());
}

lazy_static! {
	pub static ref RUNNING: AtomicPtr<Task> = AtomicPtr::new(null_mut());
}

lazy_static! {
	pub static ref BLOCKED_QUEUE: Mutex<ArrayDeque<[Task; 256]>> = Mutex::new(ArrayDeque::new());
}

pub fn process_exists(pid: u8) -> bool {
	let result = PROCESS_TABLE[pid as usize].read().is_some();
	result
}

// terminate a job
pub fn terminate(pid: u8) -> bool {
	if !process_exists(pid) {
		return false;
	}
	let reset_pt_index = | pid: usize | {
		let mut guard = PROCESS_TABLE[pid].write();
		*guard = None;
	};
	// match process state
		// remove from running
		// remove from ready queue
		// remove from blocked queue
	reset_pt_index(pid as usize);

	true
}