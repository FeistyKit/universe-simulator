#![allow(unused)]
use std::sync::mpsc::Sender;

use sfml::{graphics::FloatRect, system::Vector2};

use crate::{
    gui::{ClickResponse, GuiWidget},
    transmission::{GuiToGraphicsEvent, GuiToSimEvent},
};

