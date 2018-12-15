use std::thread;
use std::clone::Clone;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};


#[derive(Clone)]
struct Store<T: Clone, U> where T: Send, U: Send {
    state: T,
    reducer: fn(&mut T, U),
}

#[derive(Clone)]
struct StartedStore<T: Clone, U> where T: Send, U: Send {
    state: Arc<Mutex<T>>,
    channel: Sender<U>
}


impl<T: 'static + Clone, U: 'static> Store<T, U> where T: Send, U: Send {
    pub fn new(reducer: fn(&mut T, U), initial_state: T) -> Self {
        Store {
            state: initial_state,
            reducer,
        }
    }

    pub fn start(self) -> StartedStore<T, U> {
        let mut dirty: T = self.state.clone();
        let state = Arc::new(Mutex::new(self.state));
        let reducer = self.reducer;
        let thread_state = state.clone();

        let (sender, receiver) = channel();

        thread::spawn(move || {
            for action in receiver {
                reducer(&mut dirty, action);
                thread_state.lock().unwrap().clone_from(&dirty);
            }
        });

        StartedStore {
            state,
            channel: sender,
        }
    }
}

impl<T: Clone, U> StartedStore<T, U> where T: Send, U: Send {
    pub fn get(&self) -> T {
        self.state.lock().unwrap().clone()
    }

    pub fn send(&self, action: U) {
        self.channel.send(action).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_reducer() {
        enum Action { Increment, Reset };

        #[derive(Clone)]
        struct State(u32);

        fn reducer(s: &mut State, a: Action) {
            match a {
                Action::Increment => s.0 += 1,
                Action::Reset => s.0 = 0,
            }
        }

        let store = Store::new(reducer, State(0));
        let store = store.start();
        assert!(store.get().0 == 0);

        store.send(Action::Increment);
        thread::sleep_ms(10);
        assert!(store.get().0 == 1);

        store.send(Action::Increment);
        store.send(Action::Increment);
        store.send(Action::Increment);
        thread::sleep_ms(10);
        assert!(store.get().0 == 4);

        store.send(Action::Reset);
        thread::sleep_ms(10);
        assert!(store.get().0 == 0);
    }
}
