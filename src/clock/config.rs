use esp_idf_hal::{
    timer::{
        config::{AlarmConfig, TimerConfig},
        TimerDriver,
    },
    units::Hertz,
};

static mut COUNTER: u32 = 0;

pub struct TimerConfiguration<'d> {
    timer: TimerDriver<'d>,
}

impl<'d> TimerConfiguration<'d> {
    pub fn setup_timer() -> anyhow::Result<Self> {
        let config = TimerConfig {
            resolution: Hertz(1_000_000),
            ..Default::default()
        };
        let mut timer = TimerDriver::new(&config)?;

        timer.subscribe(|_event| {
            unsafe {
                COUNTER += 1;
            }
        })?;

        timer.set_alarm_action(Some(&AlarmConfig {
            alarm_count: 1_000_000,
            auto_reload_on_alarm: true,
            ..Default::default()
        }))?;

        timer.enable()?;

        Ok(TimerConfiguration { timer })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.timer.start()?;
        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.timer.stop()?;
        Ok(())
    }

    pub fn get_counter(&self) -> String {
        let counter = unsafe { COUNTER };
        let hour=counter / 3600;
        let minute=(counter % 3600) / 60;
        let second=counter % 60;
        format!("{:02}:{:02}:{:02}", hour, minute, second)
    }
}