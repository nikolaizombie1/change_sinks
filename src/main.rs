use std::process::Command;
use regex::Regex;
pub struct SinkNames {
    sink: String,
    name: String,

}
fn main() {
    let sinks = Command::new("bash").
    arg("-c").
    arg("pactl list sinks short | awk '{print $2}'").
    output().
    expect("Shell process failed");
    let out = String::from_utf8_lossy(&sinks.stdout);
    let outs = String::from(out);
    let out_vec= outs.split("\n");
    let mut outputs: Vec<String> = vec![];
    for x in out_vec {
        outputs.push(String::from(x));
    }
    let default_str = Command::new("bash").
        arg("-c").
        arg("pactl info | awk '/Default Sink: /{print $3}'").
        output().
        expect("Shell process failed").stdout;
    let default_string = String::from(String::from_utf8_lossy(&default_str));
    let outputs: Vec<String> = outputs.into_iter().filter(|x| !x.is_empty()).clone().collect();
    println!("{:?} {}",outputs, default_string);
    let index: usize = 0;
}
