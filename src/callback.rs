use std::ops::DerefMut as _;

pub trait RcRefCellCallbackExt<T> {
    fn strong_await_queue<F, C>(&self, future: F, callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static;
    fn strong_queue<C>(&self, callback: C)
    where
        C: FnMut(&mut T) + 'static;
}

impl<T: 'static> RcRefCellCallbackExt<T> for std::rc::Rc<std::cell::RefCell<T>> {
    fn strong_await_queue<F, C>(&self, future: F, mut callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static,
    {
        let strong = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let output = future.await;
            rc_ref_cell_try_run(&strong, |r| callback(r, output))
        })
    }

    fn strong_queue<C>(&self, mut callback: C)
    where
        C: FnMut(&mut T) + 'static,
    {
        self.strong_await_queue(async {}, move |r, ()| callback(r))
    }
}

pub trait WeakRcRefCellCallbackExt<T> {
    fn await_queue<F, C>(&self, future: F, callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static;
    fn queue<C>(&self, callback: C)
    where
        C: FnMut(&mut T) + 'static;
}

impl<T: 'static> WeakRcRefCellCallbackExt<T> for std::rc::Rc<std::cell::RefCell<T>> {
    fn await_queue<F, C>(&self, future: F, callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static,
    {
        std::rc::Rc::downgrade(self).await_queue(future, callback)
    }

    fn queue<C>(&self, callback: C)
    where
        C: FnMut(&mut T) + 'static,
    {
        std::rc::Rc::downgrade(self).queue(callback)
    }
}

impl<T: 'static> WeakRcRefCellCallbackExt<T> for std::rc::Weak<std::cell::RefCell<T>> {
    fn await_queue<F, C>(&self, future: F, mut callback: C)
    where
        F: std::future::Future + 'static,
        C: FnMut(&mut T, F::Output) + 'static,
    {
        let weak = self.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let output = future.await;
            weak_ref_cell_try_run(&weak, move |r| callback(r, output))
        })
    }

    fn queue<C>(&self, mut callback: C)
    where
        C: FnMut(&mut T) + 'static,
    {
        self.await_queue(async {}, move |r, ()| callback(r))
    }
}

fn weak_ref_cell_try_run<T>(weak_ref_cell: &std::rc::Weak<std::cell::RefCell<T>>, callback: impl FnOnce(&mut T)) {
    match weak_ref_cell.upgrade() {
        None => log::warn!("callback triggered with dropped receiver"),
        Some(strong) => rc_ref_cell_try_run(&strong, callback),
    }
}
fn rc_ref_cell_try_run<T>(rc_ref_cell: &std::rc::Rc<std::cell::RefCell<T>>, callback: impl FnOnce(&mut T)) {
    match rc_ref_cell.try_borrow_mut() {
        Ok(mut guard) => callback(guard.deref_mut()),
        Err(_) => log::warn!("callback triggered with already borrowed receiver (maybe due to a leak or panic)"),
    }
}
