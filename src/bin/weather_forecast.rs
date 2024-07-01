use kocli::features::weather_forecast::weather;

fn main() {
    let code = String::from("130000");
    weather::run(code);
}
