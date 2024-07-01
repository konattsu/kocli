use kocli::features::rand_img::shuffle;

fn main() {
    let img_path =
        String::from("C:/Users/soram/Pictures/background/vs_background/main");

    shuffle::run(None);
    shuffle::run(Some(img_path));
}
