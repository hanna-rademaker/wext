use std::ops::DerefMut as _;

pub trait WeakRcRefCellCallbackExt<T> {
    fn callback<F, C>(&self, future: F, callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static;
}

impl<T: 'static> WeakRcRefCellCallbackExt<T> for std::rc::Weak<std::cell::RefCell<T>> {
    fn callback<F, C>(&self, future: F, mut callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static,
    {
        let weak = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let output = future.await;
            match weak.upgrade() {
                None => log::warn!("callback triggered with dropped receiver"),
                Some(strong) => match strong.try_borrow_mut() {
                    Ok(mut guard) => callback(guard.deref_mut(), output),
                    Err(_) => log::warn!("callback triggered with already borrowed receiver (maybe due to a leak or panic)"),
                },
            }
        })
    }
}
