use bapao_trans_protocal;
pub use bapao_trans_protocal::trans_content::TransUnitType;
use std::{collections::HashMap, thread, time::Duration};

pub struct AppListener<T>
where
    T: Fn() -> TransUnitType,
{
    listener: HashMap<&'static str, T>,
}

impl<T> AppListener<T>
where
    T: Fn() -> TransUnitType,
{
    pub fn new() -> Self {
        AppListener {
            listener: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: &'static str, callback: T) {
        self.listener.insert(key, callback);
    }

    pub async fn listen(&self) {
        let mut trans_listener = bapao_trans_protocal::BtpListener::new();

        loop {
            thread::sleep(Duration::new(10, 0));

            let mut incoming_data = trans_listener.accept().await;

            incoming_data.iter_mut().for_each(|unit| {
                let req_content = unit.get();

                let callback = &self.listener.get(&req_content[..]).unwrap();

                let res_content = callback();

                let res_unit = unit.set(res_content);

                trans_listener.stash(res_unit);
            });
        }
    }
}
