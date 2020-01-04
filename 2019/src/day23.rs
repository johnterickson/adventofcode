use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::VecDeque;
use std::time::Duration;

use crossbeam_utils::thread;
use std::sync::Mutex;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError, RecvTimeoutError};

use crate::intcode::*;

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Result<Vec<isize>, ParseIntError> {
    input.split(',').map(|l| l.parse()).collect()
}

struct Node1 {
    send: SyncSender<isize>,
    recv: Receiver<isize>,
    empty_reads: usize,
}

#[aoc(day23, part1)]
fn part1(program: &[isize]) -> isize {

    let node_count : usize = 50;

    let nodes : Vec<Mutex<Node1>> = (0..node_count).map(|n| {
        let (send, recv) = sync_channel(1000);
        send.send(n as isize).unwrap();
        Mutex::new(Node1 {
            send,
            recv,
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
                            if let Ok(xy) = node.recv.try_recv() { //recv_timeout(Duration::from_millis(10)) {
                                node.empty_reads = 0;
                                Some(xy)
                            } else {
                                node.empty_reads += 1;
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
                                        dbg!(output);
                                    }
                                } else {
                                    let mut node = nodes[out_addr.unwrap()].lock().unwrap();
                                    node.send.send(out_x.unwrap()).unwrap();
                                    node.send.send(output).unwrap();
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
    }).unwrap();

    let eof_y = eof_y.lock().unwrap();
    eof_y.unwrap()
}

use std::sync::atomic::Ordering;
use std::sync::{Arc,RwLock, Barrier};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Packet {
    Name(usize),
    Data(isize,isize),
}

struct Node {
    send: SyncSender<Packet>,
    is_idle: bool,
}


const NAT : usize = 255;

struct Router {
    nodes: RwLock<Option<BTreeMap<usize,RwLock<Node>>>>
}

impl Router {
    fn new() -> Router {
        Router {
            nodes: RwLock::new(Some(BTreeMap::new())),
        }
    }

    fn send(&self, addr: usize, packet: Packet) {
        let nodes = self.nodes.read().unwrap();
        if let Some(nodes) = &*nodes {
            // println!("Sending {:?} to {}", packet, addr);
            let node = nodes[&addr].read().unwrap();
            node.send.send(packet).unwrap();
        }
    }

    fn attach(&self, addr: usize) -> Receiver<Packet> {
        let mut nodes = self.nodes.write().unwrap();
        let nodes = &mut *nodes;
        let nodes = nodes.as_mut().unwrap();
        let (send, recv) = sync_channel(1000);
        nodes.insert(addr, RwLock::new(Node {
            send,
            is_idle: false,
        }));
        recv
    }

    fn mark_not_idle(&self, addr: usize) {
        let nodes = self.nodes.read().unwrap();
        if let Some(nodes) = &*nodes {
            let mut node = nodes[&addr].write().unwrap();
            node.is_idle = false;
        }
    }

    fn mark_idle(&self, addr: usize) {
        let mut nodes = self.nodes.write().unwrap();
        if let Some(nodes) = &mut *nodes {
            let mut node = nodes[&addr].write().unwrap();
            node.is_idle = true;
        }
    }

    fn check_for_idle(&self) -> bool {
        let nodes = self.nodes.read().unwrap();
        let nodes = &*nodes;
        let nodes = nodes.as_ref().unwrap();
        let mut active_nodes = nodes.iter()
            .filter(|(k,_v)| **k != NAT)
            .filter(|(_addr,node)| !node.read().unwrap().is_idle);
        if let Some((addr,node)) = active_nodes.next() {
            // println!("Some nodes are not idle: e.g. node {} has {} empty reads.", addr, node.empty_reads);
            false
        } else {
            // reset
            for n in nodes.values() {
                n.write().unwrap().is_idle = false;
            }
            true
        }
    }

    fn shutdown(&self) {
        let mut nodes = self.nodes.write().unwrap();
        *nodes = None;
    }
}

fn run_nat(router: &Router, nat: Receiver<Packet>) -> Option<isize> {

    let mut top_packet = None;
    let mut last_sent_y = None;
    loop {
        match nat.try_recv() {
            Ok(packet) => {
                top_packet = Some(packet);
            },
            Err(TryRecvError::Disconnected) => { return None; },
            Err(TryRecvError::Empty) => { },
        }

        if router.check_for_idle() {
            // println!("Idle!");
        } else {
            // println!("Not idle!");
            std::thread::yield_now();
            continue;
        }
        
        if let Some(Packet::Data(x,y)) = top_packet {
            top_packet = None;
            router.send(0, Packet::Data(x,y));
            if last_sent_y == Some(y) {
                return Some(y);
            } else {
                last_sent_y = Some(y);
            }
        }
    }

    unreachable!();
}

fn run_node(program: &[isize], addr: usize, router: &Router, recv: Receiver<Packet>) {
    let mut node = IntCode::new(program);
    
    let mut out_addr = None;
    let mut out_x = None;

    let mut in_y = None;

    node.run( |action: CallbackAction| {
        match action {
            CallbackAction::ReadInput => {

                if let Some(y) = in_y {
                    router.mark_not_idle(addr);
                    in_y = None;
                    return Some(y);
                }

                match recv.recv_timeout(Duration::from_millis(10)) {
                    Ok(packet) => {
                        router.mark_not_idle(addr);
                        match packet {
                            Packet::Name(n) => {
                                assert_eq!(None, in_y);
                                Some(n as isize)
                            },
                            Packet::Data(x,y) => {
                                assert_eq!(None, in_y);
                                in_y = Some(y);
                                Some(x)
                            }
                        }
                    },
                    Err(RecvTimeoutError::Timeout) => {
                        router.mark_idle(addr);
                        std::thread::yield_now();
                        Some(-1)
                    },
                    Err(RecvTimeoutError::Disconnected) => None,
                }
            },
            CallbackAction::WriteOutput(output) => {
                if out_addr.is_none() {
                    out_addr = Some(output as usize);
                } else if out_x.is_none() {
                    out_x = Some(output);
                } else {
                    let addr = out_addr.unwrap();
                    router.send(addr, Packet::Data(out_x.unwrap(),output));
                    out_addr = None;
                    out_x = None;
                }

                None
            }
        }
    });
}

struct NodeArgs<'a> {
    addr: usize,
    router: &'a Router,
    start_barrier: &'a Barrier,
    program: &'a [isize],
}

#[aoc(day23, part2)]
fn part2(program: &[isize]) -> isize {
    let node_count : usize = 50;

    let mut router = Router::new();
    let start_barrier = Barrier::new(node_count + 1);
    let node_indices : Vec<_> = (0..node_count).collect();

    let final_answer = thread::scope(|scope| {
        let nat_result = scope.spawn(|_| {
            start_barrier.wait();
            let recv = router.attach(NAT);
            let answer = run_nat(&router, recv);
            router.shutdown();
            answer
        });

        let mut node_results = Vec::new();

        for addr in node_indices {
            let args = NodeArgs {
                addr,
                router: &router,
                start_barrier: &start_barrier,
                program
            };
            node_results.push(scope.spawn(move |_| {
                let addr = args.addr;
                let router = args.router;
                let start_barrier = args.start_barrier;
                let program = args.program;
                let recv = router.attach(addr);
                router.send(addr, Packet::Name(addr));
                start_barrier.wait();
                run_node(program, addr, &router, recv);
            }));
        }

        for r in node_results {
            r.join().unwrap();
        }

        nat_result.join().unwrap()
    }).unwrap();

    final_answer.unwrap()
}

