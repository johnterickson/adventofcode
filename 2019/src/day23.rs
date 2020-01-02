use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::VecDeque;

use crossbeam_utils::thread;
use std::sync::Mutex;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use crate::intcode::*;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

struct Node {
    queue: VecDeque<isize>,
    empty_reads: usize,
}

#[aoc(day23, part1)]
fn part1(program: &[isize]) -> isize {

    let node_count : usize = 50;

    let nodes : Vec<Mutex<Node>> = (0..node_count).map(|n| {
        let mut queue = VecDeque::new();
        queue.push_front(n as isize);
        Mutex::new(Node {
            queue,
            empty_reads: 0
        })
    }).collect();

    let eof_y = Mutex::new(None);
    let n = std::sync::atomic::AtomicUsize::new(0);

    thread::scope(|scope| {
        for _n in 0..node_count {
            scope.spawn(|_| {
                let n = n.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let mut node = IntCode::new(program);
                let mut out_addr = None;
                let mut out_x = None;

                node.run( |action: CallbackAction| {
                    match action {
                        CallbackAction::ReadInput => {
                            {
                                let eof_y = eof_y.lock().unwrap();
                                if eof_y.is_some() {
                                    return None;
                                }
                            }

                            let mut node = nodes[n].lock().unwrap();
                            if let Some(xy) = node.queue.pop_front() {
                                node.empty_reads = 0;
                                Some(xy)
                            } else {
                                node.empty_reads += 1;
                                std::thread::yield_now();
                                Some(-1)
                            }
                        }
                        CallbackAction::WriteOutput(output) => {
                            if out_addr.is_none() {
                                out_addr = Some(output as usize);
                            } else if out_x.is_none() {
                                out_x = Some(output);
                            } else {
                                if out_addr.unwrap() == 255 {
                                    let mut eof_y = eof_y.lock().unwrap();
                                    if eof_y.is_none() {
                                        *eof_y = Some(output);
                                        dbg!(eof_y);
                                    }
                                } else {
                                    let mut node = nodes[out_addr.unwrap()].lock().unwrap();
                                    node.queue.push_back(out_x.unwrap());
                                    node.queue.push_back(output);
                                }
                                out_addr = None;
                                out_x = None;
                            }

                            None
                        }
                    }
                });
            });
        }
    });

    let eof_y = eof_y.lock().unwrap();
    eof_y.unwrap()
}

use std::sync::atomic::Ordering;

#[aoc(day23, part2)]
fn part2(program: &[isize]) -> isize {
    let node_count : usize = 50;
    let idle_threshold = 1000;

    let nodes : Vec<Mutex<Node>> = (0..node_count).map(|n| {
        let mut queue = VecDeque::new();
        queue.push_front(n as isize);
        Mutex::new(Node {
            queue,
            empty_reads: 0
        })
    }).collect();

    let final_answer = Mutex::new(None);
    let nat_queue : Mutex<VecDeque<(isize,isize)>> = Mutex::new(VecDeque::new());
    let n = std::sync::atomic::AtomicUsize::new(0);

    thread::scope(|scope| {
        scope.spawn(|_| {
            let mut last_nat_packet = None;
            let mut last_nat_packet_y_sent = None;
            while final_answer.lock().unwrap().is_none() {
                {
                    let mut nat_queue = nat_queue.lock().unwrap();
                    while let Some(packet) = nat_queue.pop_front() {
                        last_nat_packet = Some(packet);
                    }
                }
                // std::thread::yield_now();

                if nodes.iter().all(|node| {
                    let node = node.lock().unwrap();
                    node.empty_reads > idle_threshold && node.queue.len() == 0
                }) {
                    let (x,y) = last_nat_packet.unwrap();
                    if last_nat_packet_y_sent == Some(y) {
                        *final_answer.lock().unwrap() = Some(y);
                    } else {

                        let mut node = nodes[0].lock().unwrap();
                        node.queue.push_back(x);
                        node.queue.push_back(y);

                        last_nat_packet_y_sent = Some(y);
                    }
                }
            }
        });

        for _n in 0..node_count {
            scope.spawn(|_| {
                let n = n.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let mut node = IntCode::new(program);
                let mut out_addr = None;
                let mut out_x = None;

                node.run( |action: CallbackAction| {
                    match action {
                        CallbackAction::ReadInput => {
                            if final_answer.lock().unwrap().is_some() {
                                return None;
                            }

                            let mut node = nodes[n].lock().unwrap();
                            if let Some(xy) = node.queue.pop_front() {
                                node.empty_reads = 0;
                                Some(xy)
                            } else {
                                node.empty_reads += 1;
                                std::thread::yield_now();
                                Some(-1)
                            }
                        }
                        CallbackAction::WriteOutput(output) => {
                            if out_addr.is_none() {
                                out_addr = Some(output as usize);
                            } else if out_x.is_none() {
                                out_x = Some(output);
                            } else {
                                if out_addr.unwrap() == 255 {
                                    let mut nat_queue = nat_queue.lock().unwrap();
                                    nat_queue.push_back((out_x.unwrap(), output));
                                } else {
                                    let mut node = nodes[out_addr.unwrap()].lock().unwrap();
                                    node.queue.push_back(out_x.unwrap());
                                    node.queue.push_back(output);
                                }
                                out_addr = None;
                                out_x = None;
                            }

                            None
                        }
                    }
                });
            });
        }
    });

    let final_answer = final_answer.lock().unwrap();
    final_answer.unwrap()
}

