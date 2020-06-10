use std::error::Error;
use std::env;
use crate::util::scenes::Scene;

/// Command holds the parsed commands from the command line
pub struct Command {
    // -a is aspect ratio flag
    pub aspect_ratio: f64,
    // -w is width flag
    pub width: u32,
    // -s is samples per pixel
    pub samples_per_pixel: u32,
    // will be a scene number 1..=6
    pub scene: Scene,
}

impl Default for Command {
    /// Returns the default parameters that will be used to render a scene consisting of
    /// the Cornell Box at a 16:9 aspect ratio, width = 1024, samples_per_pixel = 500
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            width: 1024,
            samples_per_pixel: 500,
            scene: Scene::CornellBox,
        }
    }
}

impl Command {
    pub fn new(args: Vec<String>) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("must provide a scene number to render");
        }

        let mut command = Command::default();

        // if two args received, the second must be a scene number
        if args.len() == 2 {
            let scene = args.iter().last().ok_or("scene number is missing")?;
            let scene = Command::parse_scene(scene)?;
            return Ok(Command { scene, ..command })
        }

        // check for and parse width
        if let Some(width) = Command::parse_width(&args) {
            command = Command { width: width?, ..command }
        }

        // check for and parse aspect ratio
        if let Some(ratio) = Command::parse_aspect_ratio(&args) {
            command = Command { aspect_ratio: ratio?, ..command }
        }

        // check for and parse samples per pixel
        if let Some(spp) = Command::parse_samples_per_pixel(&args) {
            command = Command { samples_per_pixel: spp?, ..command }
        }

        let scene = args.iter().last().ok_or("scene number is missing")?;
        let scene = Command::parse_scene(scene)?;
        command = Command { scene: scene, ..command };

        Ok(command)
    }

    /// parse aspect ratio. must be a float > 1.0
    fn parse_aspect_ratio(args: &Vec<String>) -> Option<Result<f64, &'static str>> {
        if let Some(idx) = args.iter().position(|e| e == "-a") {
            if let Some(ratio) = args.get(idx + 1) {
                match ratio.parse::<f64>() {
                    Ok(ratio) if ratio > 1.0 => Some(Ok(ratio)),
                    _ => Some(Err("aspect ratio must be a valid float > 1.0")),
                }
            } else {
                Some(Err("aspect ratio value missing"))
            }
        } else {
            None
        }
    }

    /// parse the width parameter
    fn parse_width(args: &Vec<String>) -> Option<Result<u32, &'static str>> {
        if let Some(idx) = args.iter().position(|e| e == "-w") {
            if let Some(width) = args.get(idx + 1) {
                match width.parse::<u32>() {
                    Ok(width) if width > 0 => Some(Ok(width)),
                    _ => Some(Err("width must be an integer > 0")),
                }
            } else {
                Some(Err("width value missing"))
            }
        } else {
            None
        }
    }

    /// parse samples per pixel
    fn parse_samples_per_pixel(args: &Vec<String>) -> Option<Result<u32, &'static str>> {
        if let Some(idx) = args.iter().position(|e| e == "-s") {
            if let Some(spp) = args.get(idx + 1) {
                match spp.parse::<u32>() {
                    Ok(samps) => Some(Ok(samps)),
                    _ => Some(Err("samples per pixel must be an integer > 0")),
                }
            } else {
                Some(Err("samples per pixel missing"))
            }
        } else {
            None
        }

    }

    /// parse scene number parameter
    fn parse_scene(num: &str) -> Result<Scene, &'static str> {
        match num.parse::<u8>() {
            Ok(n) if n >= 1 && n <= 6  => Ok(Scene::map_to_scene(n)),
            _ => Err(SCENE_HELP)
        }
    }
}

const SCENE_HELP: &str = r#"
            scene should be an integer between 1 and 6
            1 = Random Spheres
            2 = Two Perlin Spheres
            3 = Texture mapped Earth
            4 = Cornell Box
            5 = Cornell Box with smoky primitives
            6 = Final Scene (random boxes, spheres)
            "#;
