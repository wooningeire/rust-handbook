mod list;
mod my_box;
mod custom_smart_pointer;
mod messenger;
mod list2;
mod memory_leak;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    list::run();
    my_box::run();
    custom_smart_pointer::run();
    // messenger::run();
    list2::run();
    memory_leak::run();
}