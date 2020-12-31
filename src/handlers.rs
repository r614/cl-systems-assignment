use std::{
    cmp,
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

use url::Url;

pub fn make_request(url: &Url) -> String {
    let host = url.host_str().unwrap();
    let path = url.path();

    //println!("Hostname: {}", host);
    //println!("Path: {}", path);

    let mut stream = TcpStream::connect(format!("{}:80", host)).expect(
        "Error connecting sockets to hostname, check if the URL matches the format in --help",
    );

    let header = format!(
        "GET {:} HTTP/1.1\r\nHost: {:}\r\nConnection: close\r\n\r\n",
        path, host
    );

    let _ = stream.write(String::as_bytes(&header));

    let mut bytes = vec![];

    stream.read_to_end(&mut bytes).unwrap();
    let result = std::str::from_utf8(&bytes).unwrap();
    return result.to_string();
}

fn retrieve_status_code(response: &String) -> i32 {
    let v: Vec<&str> = response.split(' ').collect();
    return v[1].parse::<i32>().unwrap();
}

fn get_median(vec: &mut Vec<u128>) -> f32 {
    if vec.is_empty() {
        return 0.0;
    }
    vec.sort();

    let index = vec.len() / 2;

    if vec.len() % 2 == 1 {
        vec[index] as f32
    } else {
        (vec[index - 1] as f32 + vec[index] as f32) / 2.0
    }
}

fn average(nums: &mut Vec<u128>) -> f32 {
    return nums.iter().sum::<u128>() as f32 / nums.len() as f32;
}

pub fn measure_metrics(count: i32, request_url: &Url) {
    println!("Attempting to analyze {} requests.", count);
    let mut error_codes = vec![];
    let mut times = vec![];
    let mut min_size = usize::MAX;
    let mut max_size = usize::MIN;

    for _ in 1..count + 1 {
        //println!("Request: {}", c);
        let start = Instant::now();
        let response = make_request(request_url);
        let status = retrieve_status_code(&response);
        let end = start.elapsed();
        times.push(end.as_millis());

        if status >= 400 {
            error_codes.push(status);
        }
        let byte_size = response.len();

        min_size = cmp::min(min_size, byte_size);
        max_size = cmp::max(max_size, byte_size);
    }

    println!("{} requests analyzed.", count);
    let min_value = times.iter().min();
    match min_value {
        None => println!("Min value was not found"),
        Some(i) => println!("Fastest Time: {} milliseconds", i),
    }
    let max_value = times.iter().max();
    match max_value {
        None => println!("Max value was not found"),
        Some(i) => println!("Slowest Time: {} milliseconds", i),
    }

    let success_rate: f32 = (count as f32 - error_codes.len() as f32) / (count as f32) * 100.0;

    println!("Mean Request Time: {} milliseconds", average(&mut times));
    println!(
        "Median Request Time: {} milliseconds",
        get_median(&mut times)
    );
    println!("{}% requests succeeded", success_rate);
    println!("Error Codes received: {:#?}", error_codes);
    println!("Largest Response size: {} bytes", max_size);
    println!("Smallest Response size: {} bytes", min_size);
}
