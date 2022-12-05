use std::sync::{Arc, Barrier, mpsc::{self, Sender, Receiver}, atomic::{AtomicI64, Ordering}};

use itertools::Itertools;
use threadpool::ThreadPool;

use super::intcode;

fn last_thruster_signal(code: &[i64], phase_setting: &[i64]) -> i64 {
    phase_setting.iter().fold(0, |last_output, input| intcode::execute_op_code(&code, &[*input, last_output])[0])
}

pub fn find_largest_last_thruster_signal(code: &[i64]) -> i64 {
    (0i64 ..= 4i64).permutations(5).map(|phase_setting| last_thruster_signal(&code, &phase_setting)).max().unwrap()
}

struct Stage {
    input_channel: mpsc::Receiver<i64>,
    output_channel: mpsc::Sender<i64>,
    last_produced_value: i64
}

impl intcode::IO for Stage {
    // May block.
    fn read(&mut self) -> i64 {
        match self.input_channel.recv() {
            Ok(value) => value,
            Err(_) => 0
        }
    }

    // Send to the output channel.
    fn write(&mut self, value: i64) {
        self.last_produced_value = value;
        self.output_channel.send(value).unwrap_or_default();
    }
}

fn last_thruster_signal_with_feedback_loop(code: &[i64], phase_setting: &[i64], pool: &ThreadPool) -> i64 {
    let n = phase_setting.len();

    let mut senders = Vec::<Sender<i64>>::new();
    let mut receivers = Vec::<Receiver<i64>>::new();

    for (i, (s, r)) in (0 .. n).map(|i| (i, mpsc::channel::<i64>())) {
        // Initial values.
        s.send(phase_setting[i]).unwrap_or_default();
        if i == 0 { s.send(0).unwrap_or_default(); }

        senders.insert(if i == 0 { 0 } else { i - 1 }, s);
        receivers.push(r);
    }

    // Prepare each pair of received and sender for the each stages.
    let mut channels: Vec<(Receiver<i64>, Sender<i64>)> = receivers.drain(..).zip(senders.drain(..)).collect();

    let result = Arc::new(AtomicI64::new(0));
    let barrier = Arc::new(Barrier::new(2));

    for (i, (receiver, sender)) in channels.drain(..).enumerate() {
        let code_copy = Vec::<i64>::from(code);
        let barrier = barrier.clone();
        let result = result.clone();

        pool.execute(
            move || {
                let mut stage = Stage { input_channel: receiver, output_channel: sender, last_produced_value: 0 };
                intcode::execute_op_code_with_custom_io(&code_copy, &mut stage);
                if i == 4 {
                    result.store(stage.last_produced_value, Ordering::Relaxed);
                    barrier.wait();
                }
            }
        )
    }

    barrier.wait();
    result.load(Ordering::Relaxed)
}

pub fn find_largest_last_thruster_signal_with_feedback_loop(code: &[i64]) -> i64 {
    let pool = ThreadPool::new(5);
    (5i64 ..= 9i64).permutations(5).map(|phase_setting| last_thruster_signal_with_feedback_loop(&code, &phase_setting, &pool)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_1() {
        let code = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let phase_setting = [4,3,2,1,0];
        assert_eq!(last_thruster_signal(&code, &phase_setting), 43210);
    }

    #[test]
    fn part1_sample_2() {
        let code = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let phase_setting = [0,1,2,3,4];
        assert_eq!(last_thruster_signal(&code, &phase_setting), 54321);
    }

    #[test]
    fn part1_sample_3() {
        let code = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let phase_setting = [1,0,4,3,2];
        assert_eq!(last_thruster_signal(&code, &phase_setting), 65210);
    }

    #[test]
    fn part2_sample_1() {
        let code = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let phase_setting = [9,8,7,6,5];
        let pool = ThreadPool::new(5);
        assert_eq!(last_thruster_signal_with_feedback_loop(&code, &phase_setting, &pool), 139_629_729);
    }

    #[test]
    fn part2_sample_2() {
        let code = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let phase_setting = [9,7,8,5,6];
        let pool = ThreadPool::new(5);
        assert_eq!(last_thruster_signal_with_feedback_loop(&code, &phase_setting, &pool), 18_216);
    }
}