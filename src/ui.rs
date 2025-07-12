use crossterm::{style::style, terminal::window_size};
use ratatui::{
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create new JSON",
        Style::default().fg(Color::Rgb(0, 255, 0))
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Rgb(255, 255, 0))
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);

    let current_navigation = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Rgb(0, 255, 0))),
            CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Rgb(255, 255, 0))),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::Rgb(255, 0, 0)))
        } 
        .to_owned(),

        Span::styled(" | ", Style::default().fg(Color::Rgb(255, 255, 255))), 

        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => Span::styled("Editing JSON Key", Style::default().fg(Color::Rgb(0, 255, 0))),
                    CurrentlyEditing::Value => Span::styled("Editing JSON Value", Style::default().fg(Color::Rgb(0, 255, 0))),
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::Rgb(169, 169, 169))) 
            }
        }
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation)).block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit | (e) to make a new pair",
                Style::default().fg(Color::Rgb(255, 0, 0))
            ), 

            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel | (TAB) to switch boxes | (ENTER) to complete",
                Style::default().fg(Color::Rgb(255, 0, 0))
            ), 

            CurrentScreen::Exiting => Span::styled(
                "(q) to quit | (e) to make a new pair",
                Style::default().fg(Color::Rgb(255, 0, 0))
            ), 
        }
    };

    let key_hints_footer = Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_hints_footer, footer_chunks[1]);    

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Rgb(169, 169, 169)));

        let area = centered_rectangle(60, 25, frame.area());

        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::Rgb(255, 255, 200)).fg(Color::Rgb(0, 0, 0));

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area());

        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::Rgb(169, 169, 169)));

        let exit_text = Text::styled(
            "Would you like output the buffer as JSON (Y/N)", 
            Style::default().fg(Color::Rgb(255, 0, 0)));

        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rectangle(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
}

fn centered_rectangle(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    
}
