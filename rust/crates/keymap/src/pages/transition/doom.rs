use alloc::boxed::Box;
use qmk::{framebuffer::Framebuffer, screen::Screen};

use crate::{
    page::Page,
    random::rand,
    state::{InputHandler, PAGE},
};

use super::TransitionHandler;

#[derive(Clone, Copy)]
struct Column {
    until: u8,
    y: u8,
}

impl Column {
    fn from_prev(last: &Column) -> Self {
        let rand = (rand() % 3) as i32 - 1;
        let until = (last.until as i32 + rand).clamp(0, u8::MAX as i32) as u8;
        Self { until, y: 0 }
    }

    fn first() -> Self {
        Self { until: 0, y: 0 }
    }

    fn tick(&mut self) {
        self.until = self.until.saturating_sub(1);
        if self.until == 0 {
            self.y = self.y.saturating_add(6);
        }
    }
}

pub struct DoomTransition {
    to: Box<dyn Page>,
    columns: [Column; Screen::OLED_DISPLAY_WIDTH / 2],
}

impl TransitionHandler for DoomTransition {
    fn new(to: Box<dyn Page>) -> Self
    where
        Self: Sized,
    {
        let mut columns = [Column::first(); Screen::OLED_DISPLAY_WIDTH / 2];
        for i in 0..Screen::OLED_DISPLAY_WIDTH / 2 {
            if i == 0 {
                continue;
            }

            columns[i] = Column::from_prev(&columns[i - 1]);
        }

        Self { to, columns }
    }

    fn render(&mut self, renderer: &mut crate::page::RenderInfo) -> bool {
        let mut from = PAGE.borrow_ref_mut(renderer.cs);
        let mut from_framebuffer = Framebuffer::new();
        let mut from_renderer = crate::page::RenderInfo {
            framebuffer: &mut from_framebuffer,
            cs: renderer.cs,
            tick: renderer.tick,
            input: &mut InputHandler::new(),
            actions: renderer.actions,
        };
        from.render(&mut from_renderer);
        drop(from);
        drop(from_renderer);

        if self
            .columns
            .iter()
            .any(|c| (c.y as usize) < Screen::OLED_DISPLAY_HEIGHT / 4)
        {
            while let Some(_) = renderer.input.poll() {}
        }

        self.to.render(renderer);

        for column in self.columns.iter_mut() {
            column.tick();
        }

        for (i, column) in self.columns.iter().enumerate() {
            let x = i * 2;
            let y = column.y;
            let width = 2;
            let height = Screen::OLED_DISPLAY_HEIGHT;

            let slice = from_framebuffer.get_framebuffer_at(x, 0, width, height);
            renderer
                .framebuffer
                .draw_framebuffer_at(x, y, width, height, &slice);
        }

        self.columns
            .iter()
            .all(|column| column.y as usize >= Screen::OLED_DISPLAY_HEIGHT)
    }

    fn take_page(self: Box<Self>) -> Box<dyn Page> {
        self.to
    }
}
