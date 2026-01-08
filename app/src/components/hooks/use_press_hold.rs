use std::cell::Cell;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct UsePressHold {
    pub progress: RwSignal<f64>,
    pub is_holding: RwSignal<bool>,
    interval_id: Rc<Cell<Option<i32>>>,
    last_update: Rc<Cell<f64>>,
    duration: f64,
    on_complete: Callback<()>,
    disabled: Signal<bool>,
}

impl UsePressHold {
    fn clear_interval(&self) {
        if let Some(id) = self.interval_id.get() {
            window().clear_interval_with_handle(id);
        }
        self.interval_id.set(None);
    }

    pub fn on_pointer_down(&self) {
        if self.disabled.get() {
            return;
        }

        self.clear_interval();
        self.is_holding.set(true);
        self.last_update.set(js_sys::Date::now());

        let progress_signal = self.progress;
        let is_holding_signal = self.is_holding;
        let interval_id = Rc::clone(&self.interval_id);
        let last_update = Rc::clone(&self.last_update);
        let duration = self.duration;
        let on_complete = self.on_complete;

        let callback = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
            if !is_holding_signal.get() {
                return;
            }

            let now = js_sys::Date::now();
            let last = last_update.get();
            let delta = now - last;
            last_update.set(now);

            let progress_delta = delta / duration;
            let new_progress = (progress_signal.get() + progress_delta).min(1.0);
            progress_signal.set(new_progress);

            if new_progress >= 1.0 {
                on_complete.run(());
                if let Some(id) = interval_id.get() {
                    window().clear_interval_with_handle(id);
                }
                interval_id.set(None);
                is_holding_signal.set(false);
                progress_signal.set(0.0);
            }
        });

        if let Ok(id) =
            window().set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 16)
        {
            self.interval_id.set(Some(id));
        }

        callback.forget();
    }

    pub fn on_pointer_up(&self) {
        self.clear_interval();
        self.is_holding.set(false);

        if self.progress.get() <= 0.0 {
            return;
        }

        self.last_update.set(js_sys::Date::now());

        let progress_signal = self.progress;
        let is_holding_signal = self.is_holding;
        let interval_id = Rc::clone(&self.interval_id);
        let last_update = Rc::clone(&self.last_update);
        let duration = self.duration;

        let callback = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
            if is_holding_signal.get() {
                return;
            }

            let now = js_sys::Date::now();
            let last = last_update.get();
            let delta = now - last;
            last_update.set(now);

            let progress_delta = delta / duration;
            let new_progress = (progress_signal.get() - progress_delta).max(0.0);
            progress_signal.set(new_progress);

            if new_progress <= 0.0 {
                if let Some(id) = interval_id.get() {
                    window().clear_interval_with_handle(id);
                }
                interval_id.set(None);
            }
        });

        if let Ok(id) =
            window().set_interval_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), 16)
        {
            self.interval_id.set(Some(id));
        }

        callback.forget();
    }
}

/// Press-and-hold interaction pattern.
/// Progress fills while holding, drains when released.
/// Calls `on_complete` when progress reaches 1.0, then resets.
pub fn use_press_hold(duration_ms: u32, on_complete: Callback<()>, disabled: Signal<bool>) -> UsePressHold {
    UsePressHold {
        progress: RwSignal::new(0.0),
        is_holding: RwSignal::new(false),
        interval_id: Rc::new(Cell::new(None)),
        last_update: Rc::new(Cell::new(0.0)),
        duration: duration_ms as f64,
        on_complete,
        disabled,
    }
}