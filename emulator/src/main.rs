use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
// use nalgebra::Vector3 as Vector3;
use clap::{App, Arg};
use engine::device;
use engine::measure;
use engine::utils::{Coords, Trace};
use ndarray;
use ndarray_linalg::*;
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::rc::Rc;
use std::time::SystemTime;
use std::{thread, time};
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};
mod devices;
use devices::Device;

fn create_dist_measure(d1: &dyn Device, d2: &dyn Device) -> measure::Distance {
    let ts = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u32,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    measure::Distance {
        id: [d1.did(), d2.did()],
        distance: (d1.position() - d2.position()).norm(),
        timestamp: ts,
    }
}

#[derive(Serialize)]
struct Packet<T> {
    cmd: u32,
    data: T,
}

#[derive(Deserialize)]
struct ConfigAnchor {
    id: u32,
    pos: [f32; 3],
}

#[derive(Deserialize)]
struct ConfigAnchorsFile {
    anchor: Vec<ConfigAnchor>,
}

struct Config {
    deviation: f32,
    addr: String,
}

fn load_config(args: &clap::ArgMatches) -> Result<Config, std::io::Error> {
    let mut config: Config = Config {
        deviation: 0.0,
        addr: String::default(),
    };
    config.addr = args
        .value_of("addr")
        .unwrap_or("ws://127.0.0.1:2794")
        .to_string();
    config.deviation = args
        .value_of("deviation")
        .unwrap_or("0")
        .parse::<f32>()
        .unwrap();
    Result::Ok(config)
}

fn load_anchors(args: &clap::ArgMatches) -> Result<Vec<Box<dyn Device>>, std::io::Error> {
    let path = args.value_of("anchors").unwrap();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Could not find config file, using default!");
            return Result::Err(e);
        }
    };

    let mut config_anchors = String::new();
    file.read_to_string(&mut config_anchors)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let anc_conf: ConfigAnchorsFile = match toml::from_str(&config_anchors) {
        Ok(conf) => conf,
        Err(e) => return Result::Err(std::io::Error::from(e)),
    };

    let mut anchors: Vec<Box<dyn Device>> = Vec::new();
    for a in anc_conf.anchor.iter() {
        let mut dev = devices::Stationary {
            did: a.id,
            position: ndarray::arr1(&a.pos),
        };
        dev.init();
        anchors.push(Box::new(dev));
    }
    Result::Ok(anchors)
}

fn load_tags(args: &clap::ArgMatches) -> Result<Vec<Box<dyn Device>>, std::io::Error> {
    let path = args.value_of("tags_interpolated").unwrap();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Could not find config file, using default!");
            return Result::Err(e);
        }
    };

    let mut tag_conf_str = String::new();
    file.read_to_string(&mut tag_conf_str)
        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let tag_conf = match devices::Interpolated::parse_config(&tag_conf_str) {
        Ok(conf) => Rc::new(conf),
        Err(e) => return Result::Err(std::io::Error::from(e)),
    };

    let cnt: u32 = args
        .value_of("tags_cnt")
        .unwrap_or("1")
        .parse::<u32>()
        .unwrap();
    let mut tags: Vec<Box<dyn Device>> = Vec::new();
    for i in 1..cnt + 1 {
        let mut dev = devices::Interpolated::create(i, &tag_conf);
        dev.init();
        tags.push(Box::new(dev));
    }
    Result::Ok(tags)
}

fn send_anchors_descriptions(
    anchors: &Vec<Box<dyn Device>>,
    msg_sink: &mut futures::sink::Wait<futures::sync::mpsc::Sender<websocket::OwnedMessage>>,
) {
    for a in anchors.iter() {
        let pos = a.position();
        let m = device::Description {
            id: a.did(),
            timestamp: 0,
            pos: {
                Trace {
                    coords: Coords([pos[0], pos[0], pos[0]]),
                    timestamp: 0,
                }
            },
        };
        let packet = Packet { cmd: 2, data: m };
        let txt = serde_json::to_string(&packet).unwrap();
        let msg = OwnedMessage::Text(txt);
        msg_sink
            .send(msg)
            .expect("Sending message across stdin channel.");
    }
}

fn measure_loop(args: &clap::ArgMatches, sink: mpsc::Sender<OwnedMessage>, print_stats: bool) {
    let mut anchors = load_anchors(args).unwrap();
    let mut tags = load_tags(args).unwrap();
    let mut stdin_sink = sink.wait();
    let mut counter = 0;
    let freq = args.value_of("freq").unwrap().parse::<f32>().unwrap();
    if freq <= 0.0 {
        panic!("Measures frequency must be a positive number");
    }
    let dt = 1.0 / freq;
    if print_stats {
        println!(
            "used config: {} anchors, {} tags, {}s measure period",
            anchors.len(),
            tags.len(),
            dt
        );
    }
    thread::sleep(time::Duration::from_secs_f32(1.0));

    // Send measurements
    send_anchors_descriptions(&anchors, &mut stdin_sink);
    loop {
        for t in tags.iter_mut() {
            let tag: &mut dyn Device = &mut **t;
            for a in anchors.iter() {
                let anc: &dyn Device = &**a;
                let m: measure::Distance = create_dist_measure(anc, tag);
                let packet = Packet { cmd: 1, data: m };
                let txt = serde_json::to_string(&packet).unwrap();
                let msg = OwnedMessage::Text(txt);
                // Send message to websocket server
                stdin_sink
                    .send(msg)
                    .expect("Sending message across stdin channel.");
                counter += 1;
            }
            tag.update(dt);
        }

        for a in anchors.iter_mut() {
            let anc: &mut dyn Device = &mut **a;
            anc.update(dt);
        }
        print!("\rMessage counter: {:>6}", counter);
        std::io::stdout().flush().unwrap();
        // sleep
        thread::sleep(time::Duration::from_secs_f32(dt));
    }
}

// Async websocket chat client
fn main() {
    let args = App::new("RTLS device emulator")
        .version("0.1")
        .author("Karol Trzciński <k.trzcinski95@gmail.com>")
        .about("Emulate RTLS devices behavior")
        .arg(
            Arg::with_name("anchors")
                .short("a")
                .long("anchors")
                .value_name("FILE")
                .help("Anchors description file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("tags_interpolated")
                .short("i")
                .long("tags_interpolated")
                .value_name("FILE")
                .help("tags path description")
                .required(true),
        )
        .arg(
            Arg::with_name("tags_cnt")
                .short("c")
                .long("tags_counter")
                .default_value("1")
                .help("number of tags copies"),
        )
        .arg(
            Arg::with_name("deviation")
                .short("d")
                .long("deviation")
                .default_value("0")
                .help("measures standard deviation"),
        )
        .arg(
            Arg::with_name("freq")
                .short("f")
                .long("freq")
                .default_value("3")
                .help("measures frequency"),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .default_value("ws://127.0.0.1:2794")
                .help("server address, eg. ws://127.0.0.1:2794")
                .required(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    let config = load_config(&args).unwrap();
    let mut show_stats = true;

    loop {
        // Construct new Tokio runtime environment
        let mut runtime = tokio::runtime::current_thread::Builder::new()
            .build()
            .unwrap();

        let (usr_msg, stdin_ch) = mpsc::channel(0);
        let usr_msg_copy_measures = usr_msg.clone();

        let args_copy_measures = args.clone();
        thread::spawn(move || measure_loop(&args_copy_measures, usr_msg_copy_measures, show_stats));
        show_stats = false;

        // Spawn new thread to read user input
        // stdin isn't supported in mio yet, so we use a thread
        // see https://github.com/carllerche/mio/issues/321
        thread::spawn(|| {
            let mut input = String::new();
            let mut stdin_sink = usr_msg.wait();
            loop {
                // Read user input from stdin
                input.clear();
                stdin().read_line(&mut input).unwrap();

                // Trim whitespace and match input to known chat commands
                // If input is unknown, send trimmed input as a chat message
                let trimmed = input.trim();
                let (close, msg) = match trimmed {
                    "/close" => (true, OwnedMessage::Close(None)),
                    "/ping" => (false, OwnedMessage::Ping(b"PING".to_vec())),
                    _ => (false, OwnedMessage::Text(trimmed.to_string())),
                };
                // Send message to websocket server
                stdin_sink
                    .send(msg)
                    .expect("Sending message across stdin channel.");
                // If user entered the "/close" command, break the loop
                if close {
                    break;
                }
            }
        });

        // Construct a new connection to the websocket server
        println!("Connecting to {}{}", config.addr, "/dev_data");
        let runner = ClientBuilder::new([config.addr.as_str(), "/dev_data"].concat().as_str())
            .unwrap()
            .add_protocol("rust-websocket")
            .async_connect_insecure()
            .and_then(|(duplex, _)| {
                let (sink, stream) = duplex.split();
                stream
                    // Iterate over message as they arrive in stream
                    .filter_map(|message| {
                        println!("Received Message: {:?}", message);
                        // Respond to close or ping commands from the server
                        match message {
                            OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                            _ => None,
                        }
                    })
                    // Takes in messages from both sinks
                    .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
                    // Return a future that completes once all incoming data from the above streams has been processed into the sink
                    .forward(sink)
            });
        // Start our websocket client runner in the Tokio environment
        runtime.block_on(runner);
        thread::sleep(time::Duration::from_secs(5))
    }
}
