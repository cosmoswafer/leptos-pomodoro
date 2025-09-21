use leptos::prelude::*;
use gloo_timers::callback::Interval;

#[derive(Clone, Copy, PartialEq)]
pub enum TimerState {
    Work,
    ShortBreak,
    LongBreak,
}

impl TimerState {
    fn duration_minutes(&self) -> u32 {
        match self {
            TimerState::Work => 25,
            TimerState::ShortBreak => 5,
            TimerState::LongBreak => 15,
        }
    }

    fn next_state(&self, work_sessions: u32) -> Self {
        match self {
            TimerState::Work => {
                if work_sessions % 4 == 0 {
                    TimerState::LongBreak
                } else {
                    TimerState::ShortBreak
                }
            }
            TimerState::ShortBreak | TimerState::LongBreak => TimerState::Work,
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            TimerState::Work => "Work Time",
            TimerState::ShortBreak => "Short Break",
            TimerState::LongBreak => "Long Break",
        }
    }

    fn emoji(&self) -> &'static str {
        match self {
            TimerState::Work => "🍅",
            TimerState::ShortBreak => "☕",
            TimerState::LongBreak => "🛌",
        }
    }
}

#[component]
pub fn PomodoroTimer() -> impl IntoView {
    let (current_state, set_current_state) = signal(TimerState::Work);
    let (time_left, set_time_left) = signal(25 * 60); // 25 minutes in seconds
    let (is_running, set_is_running) = signal(false);
    let (work_sessions, set_work_sessions) = signal(0u32);

    // Effect to handle timer updates
    Effect::new(move |_| {
        if is_running.get() {
            let interval = Interval::new(1000, move || {
                set_time_left.update(|time| {
                    if *time > 0 {
                        *time -= 1;
                    } else {
                        // Timer finished
                        set_is_running.set(false);
                        
                        let current = current_state.get();
                        if current == TimerState::Work {
                            set_work_sessions.update(|sessions| *sessions += 1);
                        }
                        
                        let next = current.next_state(work_sessions.get());
                        set_current_state.set(next);
                        set_time_left.set(next.duration_minutes() * 60);
                        
                        // Play a notification sound (browser notification)
                        if let Some(window) = web_sys::window() {
                            let _ = window.alert_with_message(&format!("{} finished! Time for {}", 
                                current.display_name(), 
                                next.display_name()));
                        }
                    }
                });
            });
            
            // Store the interval to clean up later
            std::mem::forget(interval);
        }
    });

    let start_pause_timer = move |_| {
        set_is_running.update(|running| *running = !*running);
    };

    let reset_timer = move |_| {
        set_is_running.set(false);
        set_time_left.set(current_state.get().duration_minutes() * 60);
    };

    let switch_mode = move |new_state: TimerState| {
        move |_| {
            set_is_running.set(false);
            set_current_state.set(new_state);
            set_time_left.set(new_state.duration_minutes() * 60);
        }
    };

    let format_time = move |seconds: u32| -> String {
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    view! {
        <div class="pomodoro-timer">
            <div class="timer-header">
                <h2 class="timer-state">
                    {move || current_state.get().emoji()} " " 
                    {move || current_state.get().display_name()}
                </h2>
                <div class="session-counter">
                    "Work Sessions Completed: " {move || work_sessions.get()}
                </div>
            </div>

            <div class="timer-display">
                <div class="time-circle" class:running=move || is_running.get()>
                    <span class="time-text">
                        {move || format_time(time_left.get())}
                    </span>
                </div>
            </div>

            <div class="timer-controls">
                <button 
                    class="control-btn primary" 
                    on:click=start_pause_timer
                >
                    {move || if is_running.get() { "Pause" } else { "Start" }}
                </button>
                <button 
                    class="control-btn secondary" 
                    on:click=reset_timer
                >
                    "Reset"
                </button>
            </div>

            <div class="mode-selector">
                <button 
                    class="mode-btn"
                    class:active=move || current_state.get() == TimerState::Work
                    on:click=switch_mode(TimerState::Work)
                >
                    "🍅 Work (25m)"
                </button>
                <button 
                    class="mode-btn"
                    class:active=move || current_state.get() == TimerState::ShortBreak
                    on:click=switch_mode(TimerState::ShortBreak)
                >
                    "☕ Short Break (5m)"
                </button>
                <button 
                    class="mode-btn"
                    class:active=move || current_state.get() == TimerState::LongBreak
                    on:click=switch_mode(TimerState::LongBreak)
                >
                    "🛌 Long Break (15m)"
                </button>
            </div>
        </div>
    }
}