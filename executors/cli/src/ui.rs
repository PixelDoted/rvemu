use std::{
    error::Error,
    io::{stdout, Stdout},
};

use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use rvcore::Volatile;

use crate::{instruction::decode_instruction, TickResult};

pub enum UIEvent {
    Nothing,
    Tick,
    Exit,
}

pub struct UserInterface {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    continuous: bool,
    stop_at_breakpoint: bool,
    message: Option<String>,

    typed_number: i32,

    cursor: (i32, [i32; 2]),
    registers_scroll: usize,
    memory_scroll: usize,
}

impl UserInterface {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(Self {
            terminal,

            continuous: false,
            stop_at_breakpoint: true,
            message: None,

            typed_number: 0,

            cursor: (0, [0; 2]),
            registers_scroll: 0,
            memory_scroll: 0,
        })
    }

    pub fn render(&mut self, rv_base: &rv32i::RV32I) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let area = frame.size();
            let sections =
                Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(1)]).split(area);
            let footer = sections[1];
            let sections = Layout::horizontal(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(30),
                Constraint::Percentage(50),
            ])
            .split(sections[0]);

            {
                let mut text = format!(
                    " [q] Quit | [s] Step | [space] continuous | [b] stop at breakpoint || continuous: {}",
                    self.continuous
                );

                if let Some(message) = self.message.as_ref() {
                    text = format!("{} || {}", text, message);
                }

                frame.render_widget(Paragraph::new(text).white(), footer);
            }

            let registers = {
                let height = sections[0].height as i32 - 3;
                if self.cursor.0 == 0 {
                    if self.cursor.1[0] > height
                        && self.registers_scroll < 31usize.saturating_sub(height as usize)
                    {
                        self.registers_scroll += (self.cursor.1[0] / height).abs() as usize;
                    } else if self.cursor.1[0] < 0 && self.registers_scroll > 0 {
                        self.registers_scroll -= (self.cursor.1[0] / height).abs() as usize;
                    }

                    self.cursor.1[0] = self.cursor.1[0].clamp(0, height.min(31));
                }

                let items = (self.registers_scroll
                    ..(self.registers_scroll + area.height as usize).min(32))
                    .map(|i| {
                        let value = rv_base.get(i);
                        let mut widget = ListItem::new(Text::raw(format!("{:2}: {}", i, value)));
                        if self.cursor.0 == 0
                            && self.cursor.1[0] + self.registers_scroll as i32 == i as i32
                        {
                            widget = widget.on_dark_gray();
                        }

                        widget
                    });

                let selected = self.cursor.1[0] + self.registers_scroll as i32;
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title_top("Registers")
                    .title_bottom(format!("{}/{}", selected + 1, 32));
                List::new(items).block(block)
            };

            let visible_memory: Vec<(usize, u32)> = {
                let dram_size = rv_base.bus_ref().dram.size() / 4;
                (self.memory_scroll
                    ..(self.memory_scroll + area.height as usize).min(dram_size)).map(|i| {
                        (i*4, rv_base.bus_ref().dram.load(i * 4, 32))
                    }).collect()
            };

            let memory = {
                let height = sections[1].height as i32 - 3;
                let dram_size = rv_base.bus_ref().dram.size() / 4;

                if self.cursor.0 == 1 {
                    if self.cursor.1[1] > height {
                        self.memory_scroll +=
                            (self.cursor.1[1] / height).abs() as usize;
                    } else if self.cursor.1[1] < 0 && self.memory_scroll > 0 {
                        self.memory_scroll = self.memory_scroll.saturating_sub(
                            (self.cursor.1[1] / height).abs() as usize,
                        );
                    }

                    self.cursor.1[1] = self.cursor.1[1].clamp(0, height);
                    self.memory_scroll =
                        self.memory_scroll.clamp(0, dram_size - height as usize - 1);
                }

                let items = visible_memory.iter()
                    .map(|(i, v)| {
                        let mut text =
                            Text::raw(format!("{}: {}", i, v));
                        if self.cursor.0 == 1
                            && self.cursor.1[1] + self.memory_scroll as i32 == *i as i32 / 4
                        {
                            text = text.on_dark_gray();
                        } else if *rv_base.pc() as usize == *i {
                            text = text.on_blue();
                        }

                        text
                    });

                let selected = self.cursor.1[1] + self.memory_scroll as i32;
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title_top(format!("Memory───PC: {}", rv_base.pc()))
                    .title_bottom(format!("{}/{}", selected + 1, dram_size));

                List::new(items).block(block)
            };

            let instructions = {
                let items = visible_memory.iter().map(|(i, v)| {
                    let ins = decode_instruction(*v);
                    let mut text = Text::raw(format!("{}: {}", i, ins));
                    if self.cursor.0 == 1 && self.cursor.1[1] + self.memory_scroll as i32 == *i as i32 / 4 {
                        text = text.on_dark_gray();
                    } else if *rv_base.pc()  as usize == *i {
                        text = text.on_blue();
                    }

                    text
                });
                
                let block = Block::default().borders(Borders::ALL).title_top("Instructions");
                List::new(items).block(block)
            };

            frame.render_widget(registers, sections[0]);
            frame.render_widget(memory, sections[1]);
            frame.render_widget(instructions, sections[2]);
        })?;

        Ok(())
    }

    pub fn event(&mut self, rv_base: &rv32i::RV32I) -> Result<UIEvent, Box<dyn Error>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                event::Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => return Ok(UIEvent::Exit),
                    KeyCode::Char('c') if key.modifiers.intersects(KeyModifiers::CONTROL) => {
                        return Ok(UIEvent::Exit);
                    }

                    KeyCode::Char(' ') => {
                        self.continuous = !self.continuous;
                    }
                    KeyCode::Char('s') => {
                        self.continuous = false;
                        return Ok(UIEvent::Tick);
                    }
                    KeyCode::Char('b') => {
                        self.stop_at_breakpoint = !self.stop_at_breakpoint;
                    }

                    KeyCode::Up => {
                        self.cursor.1[self.cursor.0 as usize] =
                            self.cursor.1[self.cursor.0 as usize].saturating_sub(self.typed_number);
                        self.typed_number = 1;
                    }
                    KeyCode::Down => {
                        self.cursor.1[self.cursor.0 as usize] =
                            self.cursor.1[self.cursor.0 as usize].saturating_add(self.typed_number);
                        self.typed_number = 1;
                    }
                    KeyCode::Left => {
                        self.cursor.0 -= 1;
                    }
                    KeyCode::Right => {
                        self.cursor.0 += 1;
                    }

                    KeyCode::Char('1') => {
                        self.type_number(1);
                    }
                    KeyCode::Char('2') => {
                        self.type_number(2);
                    }
                    KeyCode::Char('3') => {
                        self.type_number(3);
                    }
                    KeyCode::Char('4') => {
                        self.type_number(4);
                    }
                    KeyCode::Char('5') => {
                        self.type_number(5);
                    }
                    KeyCode::Char('6') => {
                        self.type_number(6);
                    }
                    KeyCode::Char('7') => {
                        self.type_number(7);
                    }
                    KeyCode::Char('8') => {
                        self.type_number(8);
                    }
                    KeyCode::Char('9') => {
                        self.type_number(9);
                    }
                    KeyCode::Char('0') => {
                        self.type_number(0);
                    }

                    _ => (),
                },

                _ => (),
            }
        }

        if self.continuous {
            Ok(UIEvent::Tick)
        } else {
            Ok(UIEvent::Nothing)
        }
    }

    pub fn tick_event(&mut self, result: TickResult) {
        self.message = None;

        match result {
            TickResult::Nothing => (),
            TickResult::ECall => todo!("Ecall"),
            TickResult::EBreak => {
                if self.stop_at_breakpoint {
                    self.continuous = false;
                }

                self.message = Some("Breakpoint".into());
            }
        }
    }
}

impl UserInterface {
    fn type_number(&mut self, number: i32) {
        self.typed_number = self.typed_number.saturating_mul(10).saturating_add(number);
    }
}

impl Drop for UserInterface {
    fn drop(&mut self) {
        stdout()
            .execute(LeaveAlternateScreen)
            .expect("Failed to leave alternate screen");
        disable_raw_mode().expect("failed to disable raw mode");
    }
}
