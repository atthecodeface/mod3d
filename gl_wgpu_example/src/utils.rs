//a Imports
use std::path::Path;

//a run_to_completion
pub mod rtc {
    use std::future::Future;
    use std::task::Poll::Ready;
    static RTC_VTABLE: &std::task::RawWakerVTable = &std::task::RawWakerVTable::new(
        |x| std::task::RawWaker::new(x, RTC_VTABLE), // clone: unsafe fn(_: *const ()) -> RawWaker,
        |_| (),                                      // wake: unsafe fn(_: *const ()),
        |_| (),                                      // wake_by_ref: unsafe fn(_: *const ()),
        |_| (),                                      // drop: unsafe fn(_: *const ())
    );

    //fp run_to_completion
    pub fn run_to_completion<T, F: Future<Output = T>>(thing: F) -> T {
        let data = &() as *const ();
        let raw_waker = std::task::RawWaker::new(data, RTC_VTABLE);
        let waker = unsafe { std::task::Waker::from_raw(raw_waker) };
        let mut context = std::task::Context::from_waker(&waker);

        let mut adapter = Box::pin(thing);
        loop {
            if let Ready(val) = adapter.as_mut().poll(&mut context) {
                return val;
            }
        }
    }
}

pub fn read_file(shader_paths: &[&Path], filename: &str) -> Result<String, anyhow::Error> {
    if let Ok(x) = std::fs::read_to_string(filename) {
        Ok(x)
    } else {
        for p in shader_paths {
            let pb = p.join(filename);
            if let Ok(x) = std::fs::read_to_string(&pb) {
                println!("Shader: {x}");
                return Ok(x);
            }
        }
        Err(anyhow::anyhow!("Failed to read shader program {filename}"))
    }
}
