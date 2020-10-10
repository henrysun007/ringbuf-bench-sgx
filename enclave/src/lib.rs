// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#![feature(test)]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate ringbuf;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::slice;
use std::io::{self, Write};

use std::time::{Instant};
use std::untrusted::time::InstantEx;
use ringbuf::{RingBuffer, Producer, Consumer};

pub struct RbBencher;

impl RbBencher {
    pub fn run(rb_capacity: usize, total_transfer: usize, buf_size: usize) {
        assert!(rb_capacity >= buf_size);

        println!("rb_capacity = {} bytes", rb_capacity);
        println!("total_transfer = {} bytes", total_transfer);
        println!("buf_size = {} bytes", buf_size);

        println!("Start running the benchmark...");
        let rb = RingBuffer::<u8>::new(rb_capacity);
        let (mut producer, mut consumer) = rb.split();

        let mut count = 0;
        let input_buf = std::hint::black_box(vec![0_u8; buf_size]);
        let mut output_buf = vec![0_u8; buf_size];
        while count < total_transfer {
            let nbytes = producer.push_slice(&input_buf[..buf_size]);
            debug_assert!(nbytes == buf_size);

            let nbytes = consumer.pop_slice(&mut output_buf[..buf_size]);
            debug_assert!(nbytes == buf_size);

            count += buf_size;
        }
        std::hint::black_box(output_buf);
    }
}

#[no_mangle]
pub extern "C" fn ecall_main(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let rb_capacity : usize = 1024 * 1024;
    let total_transfer : usize = 16 * 1024 * 1024 * 1024;
    let buf_size : usize = 4 * 1024;

    let from = Instant::now();

    RbBencher::run(rb_capacity, total_transfer, buf_size);

    let elapsed = from.elapsed();
    let secs = elapsed.as_secs_f64();
    let throughput = (total_transfer as f64) / 1000.0 / 1000.0 / secs;
    println!("throughput = {} MB/s", throughput);

    sgx_status_t::SGX_SUCCESS
}
