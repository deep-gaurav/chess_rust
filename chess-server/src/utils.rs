pub fn getavatarcolor(name: &str) -> String {
    let firstchar = name.chars().next().unwrap_or('.');
    let hue = (random(firstchar as u32) * 360_f64) as u32;
    format!("hsl({},100%,50%)", hue)
}

pub fn random(seed: u32) -> f64 {
    let x = ((seed + 1) as f64).sin() * 10000_f64;
    x - x.floor()
}
