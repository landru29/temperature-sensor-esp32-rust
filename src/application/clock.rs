use std::sync::{Mutex, OnceLock};
use esp_idf_hal::{
    timer::{
        config::{AlarmConfig, TimerConfig},
        TimerDriver,
    },
    units::Hertz,
};

pub static TIMER_DRIVER: OnceLock<Mutex<TimerConfiguration<'static>>> = OnceLock::new();

pub struct TimerConfiguration<'d> {
    timer: TimerDriver<'d>,
    counter: u32,
}

impl<'d> TimerConfiguration<'d> {
    pub fn setup_timer() -> anyhow::Result<()> {
        let config = TimerConfig {
            resolution: Hertz(1_000_000),
            ..Default::default()
        };
        let mut timer = TimerDriver::new(&config)?;

        timer.subscribe(|_event| {
            if let Some(timer) = TIMER_DRIVER.get() {
                let mut timer = timer.lock().unwrap();
                timer.counter += 1;
            }
        })?;

        timer.set_alarm_action(Some(&AlarmConfig {
            alarm_count: 1_000_000,
            auto_reload_on_alarm: true,
            ..Default::default()
        }))?;

        timer.enable()?;

        TIMER_DRIVER.set(Mutex::new(TimerConfiguration { timer, counter: 0 }))
            .map_err(|_| anyhow::anyhow!("TIMER already initialized"))?;

        anyhow::Ok(())
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.timer.start()?;
        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.timer.stop()?;
        Ok(())
    }

    pub fn reset(&mut self) -> anyhow::Result<()> {
        self.counter = 0;
        Ok(())
    }

    pub fn get_counter(&self) -> String {
        let counter = self.counter;
        let hour=counter / 3600;
        let minute=(counter % 3600) / 60;
        let second=counter % 60;
        format!("{:02}:{:02}:{:02}", hour, minute, second)
    }
}


pub fn start_timer() -> anyhow::Result<()> {
    let mut timer = TIMER_DRIVER.get()
        .ok_or_else(|| anyhow::anyhow!("Timer not initialized"))?
        .lock()
        .unwrap();

    timer.start()?;

    anyhow::Ok(())
}

pub fn stop_timer() -> anyhow::Result<()> {
    let mut timer = TIMER_DRIVER.get()
        .ok_or_else(|| anyhow::anyhow!("Timer not initialized"))?
        .lock()
        .unwrap();

    timer.stop()?;

    anyhow::Ok(())
}

pub fn get_timer_counter() -> anyhow::Result<String> {
    let timer = TIMER_DRIVER.get()
        .ok_or_else(|| anyhow::anyhow!("Timer not initialized"))?
        .lock()
        .unwrap();

    let counter = timer.get_counter();

    anyhow::Ok(counter)
}

pub fn reset_timer() -> anyhow::Result<()> {
    let mut timer = TIMER_DRIVER.get()
        .ok_or_else(|| anyhow::anyhow!("Timer not initialized"))?
        .lock()
        .unwrap();

    timer.reset()?;

    anyhow::Ok(())
}