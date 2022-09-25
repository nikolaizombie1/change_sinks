use std::process::Command;
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
    let mut curr_default = String::from(String::from_utf8_lossy(&default_str));
    curr_default.truncate(curr_default.len()-1);
    let outputs: Vec<String> = outputs.into_iter().filter(|x| !x.is_empty()).clone().collect();
    let mut index = 0;
    for (i,sink) in outputs.clone().into_iter().enumerate() {
        if sink == curr_default {
            index = i;
            break;
            }
        }
    index = index+1;
    index = index%outputs.len();
    let new_sink = outputs.get(index).unwrap().clone();
    let pactl_command = "pactl set-default-sink ".to_string() + &new_sink;
    Command::new("bash").arg("-c").arg(pactl_command).spawn().expect("Error spawning shell");
}