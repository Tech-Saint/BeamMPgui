use eframe::egui;
use rfd::FileDialog;
use egui::FontTweak;
use std::io::prelude::*;
use std::{fs,env,fs::File,path::Path};
use regex::Regex;




fn main()  -> Result<(), eframe::Error> {
       
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let (MPdir , NGdir )= read_config();


    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("BeamMP_gui", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}

fn read_config() -> (String,String){
    /* Returns Tuple of MP and then NG dirs. */
    static EMPTYCFG: &str ="path_of_Beam=''\npath_of_BeamMP_Executable=''\n";
    let cfg_path: String;

    let current_dir= get_current_working_dir();
    println!("{current_dir}");

    if cfg!(windows) {
        cfg_path = format!("{}\\src\\config.cfg",current_dir);
    } else {
        cfg_path = format!("{}/src/config.cfg",current_dir);
    }

    println!("{cfg_path}");

    let mut _found: bool= false;
    let mut cfg: String = String::new();

    /* Makes sure that the CFG exists and reads it. */
    while _found != true{

        let cfg_temp = fs::read_to_string(&cfg_path);
        cfg = match cfg_temp {
            Ok(string) =>{
                _found=true;
                string
            },
            Err(_e) =>{
                let path = Path::new(&cfg_path);
                let display = path.display();

                let mut file = match File::create(&path) {
                    Err(why) => panic!("Fatal Error: couldn't create {}: {}", display, why),
                    Ok(file) => file,
                };

                match file.write_all(&EMPTYCFG.as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => continue
                }
            }
        };
    };
    /* Read from CFG */
    let re_beamng: Regex = Regex::new(r"Beam='(?<dir>\w+)'").unwrap();
    let re_beammp: Regex = Regex::new(r"BeamMP.*'(?<dir>\w+)'").unwrap();

    let caps = re_beamng.captures(&cfg);
        let beamng_dir = match caps {
            Some(value) => value["dir"].to_string(),
            None => open_file_explorer("Select your BeamNG folder."),
        };
    println!("BeamNG dir: {beamng_dir}");

    let caps = re_beammp.captures(&cfg);
    let beammp_dir = match caps {
        Some(value) => value["dir"].to_string(),
        None => open_file_explorer("Select your BeamMP folder."),
    }; 

    println!("BeamMP dir: {beammp_dir}");
    let wrotecfg= format!("path_of_Beam='{}'\npath_of_BeamMP_Executable='{}'\n",beamng_dir,beammp_dir);
    let path = Path::new(&cfg_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Fatal Error: couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(&wrotecfg.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {},
    }
    return (beammp_dir , beamng_dir);
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

fn open_file_explorer( title:&str) -> String{
    let folder: String;
    let path = FileDialog::new()
            .set_title(&title)
            .set_directory("/")
            .pick_folder();
    let _folder=path.unwrap().into_os_string().into_string();
        folder = match _folder {
            Ok(string) =>{
                string
            },
            Err(_e) =>{
                panic!("Fatal Error: Unable to open File explorer!\nPlease manually add the path of your BeamNG and BeamMP Folders to the new Config file.");
            }
        };
    println!("{folder}");
    return folder;
}

