use std::ops::DerefMut;

pub trait EventListenerExt {
    fn prevent_default(target: &web_sys::EventTarget, event_type: impl Into<std::borrow::Cow<'static, str>>) -> Self;
}

impl EventListenerExt for gloo::events::EventListener {
    fn prevent_default(target: &web_sys::EventTarget, event_type: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        gloo::events::EventListener::new_with_options(target, event_type, gloo::events::EventListenerOptions::enable_prevent_default(), |e| {
            e.prevent_default()
        })
    }
}

pub trait WeakRcRefCellEventListenerExt<T> {
    fn event_listener<F>(
        &self,
        target: &web_sys::EventTarget,
        event_type: impl Into<std::borrow::Cow<'static, str>>,
        callback: F,
    ) -> gloo::events::EventListener
    where
        F: FnMut(&mut T, &web_sys::Event) + 'static;
}

impl<T: 'static> WeakRcRefCellEventListenerExt<T> for std::rc::Weak<std::cell::RefCell<T>> {
    fn event_listener<F>(
        &self,
        target: &web_sys::EventTarget,
        event_type: impl Into<std::borrow::Cow<'static, str>>,
        mut callback: F,
    ) -> gloo::events::EventListener
    where
        F: FnMut(&mut T, &web_sys::Event) + 'static,
    {
        let weak = self.clone();
        gloo::events::EventListener::new_with_options(
            target,
            event_type,
            gloo::events::EventListenerOptions::enable_prevent_default(),
            move |e| match weak.upgrade() {
                None => log::warn!("event listener triggered with dropped value"),
                Some(strong) => match strong.try_borrow_mut() {
                    Ok(mut guard) => callback(guard.deref_mut(), e),
                    Err(_) => log::warn!("event listener triggered with already borrowed value (maybe due to a leak or panic)"),
                },
            },
        )
    }
}
