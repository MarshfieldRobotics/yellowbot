#![no_std]
#![no_main]

use core::time::Duration;
use vex_rt::{prelude::*, select};

mod drive;

struct PissBot {
    drive: Mutex<drive::Drive>,
    controller: Controller,
}

impl Robot for PissBot {
    fn new(peripherals: Peripherals) -> Self {
        Self {
            drive: Mutex::new(drive::Drive {
                left_drive: peripherals
                    .port01
                    .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, false)
                    .unwrap(),
                right_drive: peripherals
                    .port02
                    .into_motor(Gearset::EighteenToOne, EncoderUnits::Degrees, true)
                    .unwrap(),
            }),
            controller: peripherals.master_controller,
        }
    }

    fn initialize(&mut self, _ctx: Context) {
        println!("initialize");
        // Do any extra initialization here.
    }

    fn autonomous(&mut self, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(&mut self, ctx: Context) {
        println!("opcontrol");

        // This loop construct makes sure the drive is updated every 10
        // milliseconds.
        let mut l = Loop::new(Duration::from_millis(10));
        loop {
            // Update the motors.
            self.drive.lock().run(
                self.controller.left_stick.get_y().unwrap(),
                self.controller.right_stick.get_y().unwrap(),
            );

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => continue,
            }
        }
    }

    fn disabled(&mut self, _ctx: Context) {
        println!("disabled");
        // This runs when the robot is in disabled mode.
    }
}

entry!(PissBot);
