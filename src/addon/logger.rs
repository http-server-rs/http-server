use anyhow::Result;
use chrono::{DateTime, Utc};
use http::header::USER_AGENT;
use http::Method;
use hyper::Body;
use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::server::middleware::{Request, Response};

pub struct Logger {
    buffer_writer: BufferWriter,
}

impl Logger {
    pub fn new() -> Self {
        let buffer_writer = BufferWriter::stdout(ColorChoice::Always);

        Logger { buffer_writer }
    }

    pub async fn log(&mut self, request: Request<Body>, response: Response<Body>) -> Result<()> {
        let mut buffer = self.buffer_writer.buffer();
        let request = request.lock().await;
        let response = response.lock().await;

        // UTC Time
        let moment: DateTime<Utc> = Utc::now();
        write!(&mut buffer, "[{:?}] \"", moment)?;

        // HTTP Request Method
        let method = request.method();

        match *method {
            Method::GET => buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?,
            Method::POST | Method::PUT | Method::PATCH => {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?
            }
            Method::DELETE => buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?,
            _ => buffer.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?,
        };

        write!(&mut buffer, "{} ", method)?;
        buffer.reset()?;

        // HTTP Request URI and Version
        write!(&mut buffer, "{} {:?} ", request.uri(), request.version())?;

        // HTTP Response Status Code
        match response.status().as_u16() {
            100..=199 => {
                // Informational Responses
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            }
            200..=299 => {
                // Successful responses
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            }
            300..=399 => {
                // Redirection messages
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
            }
            400..=499 => {
                // Client error responses
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 140, 0))))?;
            }
            500..=599 => {
                // Server error responses
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            }
            _ => {
                // Unknown response codes
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
            }
        }
        write!(&mut buffer, "{}", response.status())?;
        buffer.reset()?;

        // HTTP Request User Agent
        let user_agent = if let Some(value) = request.headers().get(USER_AGENT) {
            value.to_str()?
        } else {
            "N/A"
        };
        write!(&mut buffer, "\" \"{}\" ", user_agent)?;

        writeln!(&mut buffer)?;
        self.buffer_writer.print(&buffer)?;
        buffer.reset()?;

        Ok(())
    }
}
