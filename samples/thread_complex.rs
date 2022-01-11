use std::sync::Arc;
use std::thread;
use std::time;


struct Complex {
    real: f32,
    imag: f32,
}

#[allow(dead_code)]
impl Complex {
    fn new(real: f32, imag: f32) -> Complex {
        println!("Constructing complex number: {:}+{:}i", real, imag);
        Complex{real:real, imag:imag}
    }
 
    fn print(&self) {
        println!("complex number: {:?}+{:?}i", self.real, self.imag);
    }
}
 
impl Drop for Complex {
    fn drop(&mut self) {
        println!("Dropping complex number: {:}+{:}i", self.real, self.imag);
    }
}
 
struct ComplexNumberOwner {
    id: i32,
    value: Arc<Complex>
}
 
impl ComplexNumberOwner {
    fn print(&self) {
        println!("owner: number #{} with value {}+{}i",
                 self.id,
                 self.value.real,
                 self.value.imag,
        );
    }
}
 
fn delay(ms : u64) {
    let amount = time::Duration::from_millis(ms);
    thread::sleep(amount);
}
 
fn start_threads() {
    let c = Arc::new(Complex::new(1.0, 1.0));
 
    for id in 0..10 {
        let owner = ComplexNumberOwner{id:id,
                                       // CLONE
                                       value: c.clone(),
        };
 
        thread::spawn(move || {
            owner.print();
            // to see aloc/deloc
            delay(400);
        });
    }
}
 
fn main() {
    println!("#STARTING THREADS\n");

    start_threads();

    println!("\n#ALL THREADS STARTED\n");

    // wait to see all threads + also drop info
    delay(2000);
}
