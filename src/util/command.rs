// use crate::util::scenes::Scene;
//
// const VALID_SWITCHES: [&str; 6] = ["-h", "--help", "-a", "-p", "-s", "-w"];
//
// /// Command holds the parsed commands from the command line
// #[derive(Debug)]
// pub struct Command {
//     // -a is aspect ratio flag
//     pub aspect_ratio: f64,
//     // -w is width flag
//     pub width: u32,
//     // -s is samples per pixel
//     pub samples_per_pixel: u32,
//     // will be a scene number 1..=6
//     pub scene: Scene,
// }
//
// impl Default for Command {
//     /// Returns the default parameters that will be used to render a scene consisting of
//     /// the Cornell Box at a 16:9 aspect ratio, width = 1024, samples_per_pixel = 500
//     fn default() -> Self {
//         Self {
//             aspect_ratio: 16.0 / 9.0,
//             width: 1024,
//             samples_per_pixel: 500,
//             scene: Scene::CornellBox,
//         }
//     }
// }
//
// impl Command {
//     pub fn new(args: Vec<String>) -> Result<Command, String> {
//         // if no options given, render CornellBox by default
//         if args.len() == 1 {
//             return Ok(Command::default());
//         }
//
//         Command::validate_switches(&args)?;
//
//         let mut command = Command::default();
//
//         // check for help switch
//         if args.iter().any(|s| s == "-h" || s == "--help") {
//             return Err(HELP.to_string());
//         }
//
//         // check for and parse width
//         if let Some(width) = Command::parse_width(&args) {
//             command = Command {
//                 width: width?,
//                 ..command
//             }
//         }
//
//         // check for and parse aspect ratio
//         if let Some(ratio) = Command::parse_aspect_ratio(&args) {
//             command = Command {
//                 aspect_ratio: ratio?,
//                 ..command
//             }
//         }
//
//         // check for and parse samples per pixel
//         if let Some(spp) = Command::parse_samples_per_pixel(&args) {
//             command = Command {
//                 samples_per_pixel: spp?,
//                 ..command
//             }
//         }
//
//         // check for scene number
//         let scene = Command::parse_scene_num(&args)?;
//         command = Command { scene, ..command };
//
//         Ok(command)
//     }
//
//     /// parse aspect ratio. must be a float > 1.0
//     fn parse_aspect_ratio(args: &[String]) -> Option<Result<f64, String>> {
//         if let Some(idx) = args.iter().position(|e| e == "-a") {
//             if let Some(ratio) = args.get(idx + 1) {
//                 match ratio.parse::<f64>() {
//                     Ok(ratio) if ratio > 1.0 => Some(Ok(ratio)),
//                     _ => Some(Err(ASPECT_HELP.to_string())),
//                 }
//             } else {
//                 Some(Err("aspect ratio value missing".to_string()))
//             }
//         } else {
//             None
//         }
//     }
//
//     /// parse the width parameter
//     fn parse_width(args: &[String]) -> Option<Result<u32, String>> {
//         if let Some(idx) = args.iter().position(|e| e == "-w") {
//             if let Some(width) = args.get(idx + 1) {
//                 match width.parse::<u32>() {
//                     Ok(width) if width > 0 => Some(Ok(width)),
//                     _ => Some(Err("width must be an integer > 0".to_string())),
//                 }
//             } else {
//                 Some(Err("width value missing".to_string()))
//             }
//         } else {
//             None
//         }
//     }
//
//     /// parse samples per pixel
//     fn parse_samples_per_pixel(args: &[String]) -> Option<Result<u32, String>> {
//         if let Some(idx) = args.iter().position(|e| e == "-p") {
//             if let Some(spp) = args.get(idx + 1) {
//                 match spp.parse::<u32>() {
//                     Ok(samps) => Some(Ok(samps)),
//                     _ => Some(Err("samples per pixel must be an integer > 0".to_string())),
//                 }
//             } else {
//                 Some(Err("samples per pixel missing".to_string()))
//             }
//         } else {
//             None
//         }
//     }
//
//     /// parse scene number parameter
//     fn parse_scene_num(args: &[String]) -> Result<Scene, String> {
//         if let Some(idx) = args.iter().position(|e| e == "-s") {
//             if let Some(scene_num) = args.get(idx + 1) {
//                 if let Ok(num) = scene_num.parse::<u32>() {
//                     match Scene::map_to_scene(num) {
//                         Some(scene) => Ok(scene),
//                         _ => Err(SCENE_HELP.to_string()),
//                     }
//                 } else {
//                     Err("samples per pixel must be an integer > 0".to_string())
//                 }
//             } else {
//                 Err(SCENE_HELP.to_string())
//             }
//         } else {
//             // default scene to (FINAL_SCENE)
//             Ok(Scene::Final)
//         }
//     }
//
//     // make sure all switches are valid
//     fn validate_switches(args: &[String]) -> Result<bool, String> {
//         for (i, sw) in args.iter().enumerate().filter(|(_i, s)| s.starts_with('-')) {
//             if !VALID_SWITCHES.contains(&&**sw) {
//                 return Err(format!("{} is not a valid option\n{}", args[i], HELP));
//             }
//         }
//         Ok(true)
//     }
// }
//
// const HELP: &str = r#"
// raytracer [-w WIDTH] [-p SAMPLES_PER_PIXEL] [-a ASPECT_RATIO] [-s SCENE_NUMBER]
//
// WIDTH = width of the rendered image, defaults to 1024
// SAMPLES_PER_PIXEL = number of multisamples to take for each pixel. defaults to 500.
//                     improves image quality, but increases render time.
// ASPECT_RATIO = should be a floating point number >= 1.0. Defaults to 1.77  Some examples:
//                1.77 = a 16:9 aspect ratio
//                1.6  = a 16:10 aspect ratio
//                1.33 = a 4:3 apect ratio
//                1.43 = IMAX film format
//                1.85 = U.S. widescreen cinema format
// SCENE_NUMBER = scene number to render:
//                1 = Random Spheres
//                2 = Two Perlin Spheres
//                3 = Texture mapped Earth
//                4 = Cornell Box
//                5 = Cornell Box with two smoke cubes
//                6 = Final Scene (random boxes, spheres, lit by a single light)
// "#;
//
// const SCENE_HELP: &str = r#"
// scene should be an integer between 1 and 6
// 1 = Random Spheres
// 2 = Two Perlin Spheres
// 3 = Texture mapped Earth
// 4 = Cornell Box
// 5 = Cornell Box with two smoke cubes
// 6 = Final Scene (random boxes, spheres)
// "#;
//
// const ASPECT_HELP: &str = r#"
// aspect ratio should be a floating point number >= 1.0  For example:
// 1.77 = a 16:9 aspect ratio
// 1.6  = a 16:10 aspect ratio
// 1.33 = a 4:3 apect ratio
// 1.43 = IMAX film format
// 1.85 = U.S. widescreen cinema format
// "#;
//
// #[cfg(test)]
// mod tests {
//     use super::Command;
//     use crate::util::scenes::Scene;
//
//     #[test]
//     fn valid_cornell_box_scene_number() {
//         let s: Vec<String> = "raytracer -w 400 -a 1.777 -p 1000 -s 1"
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect();
//         let com = Command::new(s);
//
//         assert!(com.is_ok());
//         assert_eq!(com.unwrap().scene, Scene::RandomSpheres);
//     }
//
//     #[test]
//     fn invalid_cornell_box_scene_number() {
//         let s: Vec<String> = "raytracer -w 400 -a 1.777 -p 1000 -s 8"
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect();
//         let com_res = Command::new(s);
//
//         assert!(com_res.is_err());
//     }
//
//     #[test]
//     fn scene_defaults_to_final_scene() {
//         let s: Vec<String> = "raytracer -w 400 -a 1.777 -p 1000"
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect();
//         let com_res = Command::new(s);
//
//         assert!(com_res.is_ok());
//         assert_eq!(com_res.unwrap().scene, Scene::Final);
//     }
//
//     #[test]
//     fn samples_per_pixel_set_to_10000() {
//         let s: Vec<String> = "raytracer -w 400 -a 1.777 -p 10000"
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect();
//         let com_res = Command::new(s);
//
//         assert!(com_res.is_ok());
//         assert_eq!(com_res.unwrap().samples_per_pixel, 10_000);
//     }
//
//     #[test]
//     fn samples_per_pixel_defaults_to_500() {
//         let s: Vec<String> = "raytracer -w 400 -a 1.777"
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect();
//         let com_res = Command::new(s);
//
//         assert!(com_res.is_ok());
//         assert_eq!(com_res.unwrap().samples_per_pixel, 500);
//     }
// }
