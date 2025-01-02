#[derive(Clone, Copy)]
pub struct AppState {
    pub count: i32,
}

#[derive(Copy, Clone)]
pub struct AppStateLock(AppState);
impl AppStateLock {
    const fn new(v: AppState) -> Self {
        Self(v)
    }

    pub fn unwrap_rw(mut self) -> (AppStateReader, AppStateWriter) {
        (
            AppStateReader(&self.0 as *const AppState),
            AppStateWriter(&mut self.0 as *mut AppState),
        )
    }

    pub fn wrap_rw(r: AppStateReader, w: AppStateWriter) -> Self {
        Self(r.read())
    }
}

pub struct AppStateReader(*const AppState);
impl AppStateReader {
    pub fn read(&self) -> AppState {
        unsafe { *self.0 }
    }
}

pub struct AppStateWriter(*mut AppState);
impl AppStateWriter {
    pub fn write(&mut self, v: AppState) {
        unsafe { *self.0 = v }
    }
}

impl AppState {
    pub const fn new() -> Self {
        AppState { count: 0 }
    }
}

pub static app_state: AppStateLock = AppStateLock::new(AppState::new());
