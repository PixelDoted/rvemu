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
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Terminal,
};
use rvcore::Volatile;

use crate::{instruction::{decode_instruction, encode_instruction}, TickResult};

pub enum UIEvent {
    Nothing,
    Tick,
    Exit,
}

pub struct EditInfo {
    text: String,
    index: usize,
    is_memory: bool,
}

pub struct UserInterface {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    core_hz: f32,
    continuous: bool,
    stop_at_breakpoint: bool,
    message: Option<String>,

    edit: Option<EditInfo>,

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

            core_hz: 10.0,
            continuous: false,
            stop_at_breakpoint: true,
            message: None,

            edit: None,

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
                        self.registers_scroll += 1;
                    } else if self.cursor.1[0] < 0 && self.registers_scroll > 0 {
                        self.registers_scroll -= 1;
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
                        self.memory_scroll += 1;
                    } else if self.cursor.1[1] < 0 && self.memory_scroll > 0 {
                        self.memory_scroll -= 1;
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

            if let Some(info) = &self.edit {
                let popup_block = Block::default().title("Edit value").borders(Borders::ALL).style(Style::default().bg(Color::DarkGray));

                let x = 40;
                let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(3), Constraint::Fill(1)]).split(area);
                let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(x), Constraint::Fill(1)]).split(layout[1])[1];

                let widget = Paragraph::new(info.text.clone()).block(popup_block);
                Clear.render(layout, frame.buffer_mut());
                frame.render_widget(widget, layout);
                frame.set_cursor(layout.x + 1 + info.text.len() as u16, layout.y + 1);
            }
        })?;

        Ok(())
    }

    pub fn event(&mut self, rv_base: &mut rv32i::RV32I) -> Result<UIEvent, Box<dyn Error>> {
        let timeout = 1.0 / if self.continuous {
            self.core_hz
        } else {
            10.0
        };
        
        if event::poll(std::time::Duration::from_secs_f32(timeout))? {
            match event::read()? {
                event::Event::Key(key) if key.kind == KeyEventKind::Press => match &mut self.edit {
                    Some(info) => {
                        match key.code {
                            KeyCode::Char(char) => {
                                info.text.push(char);
                            }
                            KeyCode::Backspace => {
                                info.text.pop();
                            }

                            KeyCode::Esc => {
                                self.edit = None;
                            }
                            KeyCode::Enter => {
                                let number = if let Ok(number) = info.text.parse::<u32>() {
                                    number
                                } else if let Some(ins) = encode_instruction(&info.text) {
                                    ins
                                } else {
                                    return Ok(UIEvent::Nothing);
                                };
                                
                                if info.is_memory {
                                    rv_base.bus().dram.store(info.index * 4, 32, number as u32);
                                } else {
                                    rv_base.set(info.index, number as i32);
                                }

                                self.edit = None;
                            }
                            _ => (),
                        }
                    }
                    None => {
                        match key.code {
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
                                self.cursor.1[self.cursor.0 as usize] -= 1;
                            }
                            KeyCode::Down => {
                                self.cursor.1[self.cursor.0 as usize] += 1;
                            }
                            KeyCode::Left => if self.cursor.0 > 0 {
                                self.cursor.0 -= 1;
                            }
                            KeyCode::Right => if self.cursor.0 < 1 {
                                self.cursor.0 += 1;
                            }

                            KeyCode::Enter => {
                                let index = if self.cursor.0 == 1 {
                                    self.memory_scroll + self.cursor.1[self.cursor.0 as usize] as usize
                                } else {
                                    self.registers_scroll + self.cursor.1[self.cursor.0 as usize] as usize
                                };
                                let text = if self.cursor.0 == 1 {
                                    rv_base.bus().dram.load(index * 4, 32).to_string()
                                } else {
                                    rv_base.get(index).to_string()
                                };
                                self.edit = Some(EditInfo { text, index, is_memory: self.cursor.0 == 1 });
                            }

                            _ => (),
                        }
                    }
                    
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

impl Drop for UserInterface {
    fn drop(&mut self) {
        stdout()
            .execute(LeaveAlternateScreen)
            .expect("Failed to leave alternate screen");
        disable_raw_mode().expect("failed to disable raw mode");
    }
}
