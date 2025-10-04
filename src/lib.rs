#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use asr::{
    future::{next_tick, retry},
    settings::Gui,
    timer::{self, TimerState},
    watcher::Watcher,
    Address, Process,
    deep_pointer::DeepPointer,
    string::ArrayCString,
};

mod settings;
use settings::Settings;
mod data;
use data::Area;

asr::async_main!(stable);

const PROCESS_NAME: &str = "swkotor.exe";

fn get_module_base(process: &Process) -> Address {
    match process.get_module_range(PROCESS_NAME) {
        Ok((base, _)) if base != Address::NULL => base,
        _ => Address::new(0x0040_0000),
    }
}

async fn main() {
    let mut settings = Settings::register();

    loop {
        let process = retry(|| Process::attach(PROCESS_NAME)).await;

        process
            .until_closes(async {
                let _base = get_module_base(&process);

                let mut watchers = Watchers::default();
                let addresses = Addresses::init(&process).await;

                loop {
                    settings.update();

                    let _ = update_loop(&process, &addresses, &mut watchers);

                    match timer::state() {
                        TimerState::Running | TimerState::Paused => {
                            handle_running_state(&mut watchers, &settings)
                        }
                        TimerState::NotRunning => {
                            if start_timer(&watchers) {
                                timer::start();
                            }
                        }
                        TimerState::Ended => timer::reset(),
                        _ => {}
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}

impl Default for Area {
    fn default() -> Self {
        Area::Unknown(String::new())
    }
}

struct Watchers {
    area: Watcher<Area>,
    is_loading: Watcher<bool>,
    entered_areas: Vec<Area>,
    old_area: Area,
    load_bar_value: i32,
    tickcount_value: u32,
    message_box_value: u32,
    message_box_callback_value: u32,
    modal_stack_first_value: u32,
    modal_stack_second_value: u32,
    modal_count_value: u32,
    end_state_value: u32,
    trust_load: bool,
    is_message_box_open: bool,
    is_message_box_amg: bool,
    is_mb_on_modal_stack: bool,
}

impl Default for Watchers {
    fn default() -> Self {
        Self {
            area: Watcher::default(),
            is_loading: Watcher::default(),
            entered_areas: vec![Area::END_M01AA],
            old_area: Area::Unknown(String::new()),
            load_bar_value: 0,
            tickcount_value: 0,
            message_box_value: 0,
            message_box_callback_value: 0,
            modal_stack_first_value: 0,
            modal_stack_second_value: 0,
            modal_count_value: 0,
            end_state_value: 0,
            trust_load: true,
            is_message_box_open: false,
            is_message_box_amg: false,
            is_mb_on_modal_stack: false,
        }
    }
}

struct Addresses {
    area: DeepPointer<5>,
    tickcount: DeepPointer<5>,
    end_state: Address,
    load_bar: DeepPointer<6>,
    message_box: DeepPointer<5>,
    message_box_callback: DeepPointer<6>,
    modal_stack_first: DeepPointer<7>,
    modal_stack_second: DeepPointer<7>,
    modal_count: DeepPointer<6>,
}

impl Addresses {
    async fn init(process: &Process) -> Self {
        let base = get_module_base(process);

        Self {
            area: DeepPointer::new_32bit(base, &[0x3A39E8, 0x4C, 0x0].map(|x| x as u64)),
            tickcount: DeepPointer::new_32bit(base, &[0x3B935C, 0x54, 0x64, 0x18C].map(|x| x as u64)),
            end_state: base + 0x3BB4E4,
            load_bar: DeepPointer::new_32bit(base, &[0x3A39FC, 0x4, 0x4, 0x278, 0xC4].map(|x| x as u64)),
            message_box: DeepPointer::new_32bit(base, &[0x3A39FC, 0x4, 0x4, 0x40, 0x98].map(|x| x as u64)),
            message_box_callback: DeepPointer::new_32bit(base, &[0x3A39FC, 0x4, 0x4, 0x40, 0x98, 0x68].map(|x| x as u64)),
            modal_stack_first: DeepPointer::new_32bit(base, &[0x3A39FC, 0x4, 0x4, 0x40, 0x38, 0x94, 0x0].map(|x| x as u64)),
            modal_stack_second: DeepPointer::new_32bit(base, &[0x3A39FC, 0x4, 0x4, 0x40, 0x38, 0x94, 0x4].map(|x| x as u64)),
            modal_count: DeepPointer::new_32bit(base, &[0x3A39FC, 0x4, 0x4, 0x40, 0x38, 0x98].map(|x| x as u64)),
        }
    }
}

fn update_loop(process: &Process, addresses: &Addresses, watchers: &mut Watchers) -> Result<(), String> {
    if let Ok(raw_area) = addresses.area.deref::<ArrayCString<10>>(process) {
        let area_str = raw_area.validate_utf8().unwrap_or("unknown");
        let area = Area::from(area_str);
        if watchers.old_area != area {
            watchers.old_area = area.clone();
        }
        watchers.area.update(Some(area));
    }

    watchers.load_bar_value = addresses.load_bar.deref_offsets(process)
        .ok().and_then(|addr| process.read::<i32>(addr).ok()).unwrap_or(0);

    watchers.message_box_value = addresses.message_box.deref_offsets(process)
        .ok().and_then(|addr| process.read::<u32>(addr).ok()).unwrap_or(0);

    watchers.message_box_callback_value = addresses.message_box_callback.deref_offsets(process)
        .ok().and_then(|addr| process.read::<u32>(addr).ok()).unwrap_or(0);

    watchers.modal_stack_first_value = addresses.modal_stack_first.deref_offsets(process)
        .ok().and_then(|addr| process.read::<u32>(addr).ok()).unwrap_or(0);

    watchers.modal_stack_second_value = addresses.modal_stack_second.deref_offsets(process)
        .ok().and_then(|addr| process.read::<u32>(addr).ok()).unwrap_or(0);

    watchers.modal_count_value = addresses.modal_count.deref_offsets(process)
        .ok().and_then(|addr| process.read::<u32>(addr).ok()).unwrap_or(0);

    watchers.tickcount_value = addresses.tickcount.deref_offsets(process)
        .ok().and_then(|addr| process.read::<u32>(addr).ok()).unwrap_or(0);

    watchers.end_state_value = process.read::<u32>(addresses.end_state).ok().unwrap_or(0);

    watchers.is_message_box_open = watchers.modal_count_value > 0;
    watchers.is_message_box_amg = watchers.message_box_callback_value == 0x005F1BD0;
    watchers.is_mb_on_modal_stack =
        (watchers.modal_stack_first_value == watchers.message_box_value)
        || (watchers.modal_count_value > 1 && watchers.modal_stack_second_value == watchers.message_box_value);

    if watchers.load_bar_value == 0 {
        watchers.trust_load = true;
    } else if watchers.is_message_box_open && watchers.is_message_box_amg && watchers.is_mb_on_modal_stack {
        watchers.trust_load = false;
    }

    watchers.is_loading.update(Some(watchers.load_bar_value > 0 && watchers.trust_load));

    asr::print_message(&format!("area={:?}", watchers.area.pair.as_ref().map(|p| &p.current)));
    asr::print_message(&format!("tick={}", watchers.tickcount_value));
    asr::print_message(&format!("loadbar={} trust={}", watchers.load_bar_value, watchers.trust_load));
    asr::print_message(&format!("mb={} cb={:X} modal={} end={}", watchers.message_box_value, watchers.message_box_callback_value, watchers.modal_count_value, watchers.end_state_value));

    Ok(())
}

fn handle_running_state(watchers: &mut Watchers, settings: &Settings) {
    if let Some(area_pair) = &watchers.area.pair {
        if area_pair.old != area_pair.current {
            if should_split(&area_pair.current, settings, &mut watchers.entered_areas) {
                watchers.entered_areas.push(area_pair.current.clone());
                timer::split();
            }
            if settings.BastilaSaved {
                if area_pair.old == Area::TAR_M03AF && area_pair.current == Area::TAR_M02AF {
                    timer::split();
                }
            }
        }
    }

    if let Some(area_pair) = &watchers.area.pair {
        if area_pair.current == Area::STA_M45AD && watchers.end_state_value == 1132924759 {
            if !watchers.entered_areas.contains(&Area::STA_M45AD) {
                watchers.entered_areas.push(Area::STA_M45AD);
                timer::split();
            }
        }
    }

    if let Some(is_loading) = &watchers.is_loading.pair {
        if is_loading.current {
            timer::pause_game_time();
        } else {
            timer::resume_game_time();
        }
    }
}

fn start_timer(watchers: &Watchers) -> bool {
    if let Some(area_pair) = &watchers.area.pair {
        if area_pair.current == Area::END_M01AA {
            let tick = watchers.tickcount_value;
            if tick > 0 && tick < 30 {
                timer::start();
                timer::set_game_time(asr::time::Duration::seconds_f64(0.75));
                return true;
            }
        }
    }
    false
}

fn should_split(area: &Area, settings: &Settings, entered_areas: &mut Vec<Area>) -> bool {
    let (enabled, unlim) = settings.for_area(area);
    if !enabled {
        return false;
    }
    if unlim {
        return true;
    }
    if !entered_areas.contains(area) {
        return true;
    }
    false
}
