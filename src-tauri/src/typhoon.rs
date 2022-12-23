extern crate error_chain;
extern crate tempdir;
use bytes::Bytes;
use error_chain::error_chain;
use reqwest;
use reqwest::Response;
use serde_json::json;
use std::env::current_dir;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::copy;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::path::PathBuf;
use zip::ZipArchive;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn extract(mut archive: ZipArchive<Cursor<Bytes>>, root: &PathBuf) -> PathBuf {
    let mut file = archive.by_index(0).unwrap();
    let outpath = if let Some(path) = file.enclosed_name() {
        root.join(path.to_owned())
    } else {
        println!("Wrong path to extract");
        PathBuf::new()
    };
    if let Some(p) = outpath.parent() {
        if !p.exists() {
            create_dir_all(p).unwrap();
        }
    }
    let mut outfile = File::create(&outpath).unwrap();
    copy(&mut file, &mut outfile).unwrap();
    // Get and Set permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if let Some(mode) = file.unix_mode() {
            fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
        }
    }

    outpath
}

fn parse_header(words: Vec<&str>) -> serde_json::Value {
    let mut header = json!({});
    if words.len() == 9 {
        header = json!({
            "Indicator": words[0],
            "InternationalNumberID": words[1],
            "Lines": words[2].parse::<u8>().unwrap(),
            "TropicalCycloneNumberID": words[3],
            "InternationalNumberIDRp": words[4],
            "Flag": words[5],
            "TimeDelta": words[6].parse::<u8>().unwrap(),
            "Name": words[7],
            "Date": words[8],
        });
    }
    if words.len() == 8 {
        header = json!({
            "Indicator": words[0],
            "InternationalNumberID": words[1],
            "Lines": words[2].parse::<u8>().unwrap(),
            "InternationalNumberIDRp": words[3],
            "Flag": words[4],
            "TimeDelta": words[5].parse::<u8>().unwrap(),
            "Name": words[6],
            "Date": words[7],
        });
    }
    if words.len() == 7 {
        header = json!({
            "Indicator": words[0],
            "InternationalNumberID": words[1],
            "Lines": words[2].parse::<u8>().unwrap(),
            "InternationalNumberIDRp": words[3],
            "Flag": words[4],
            "TimeDelta": words[5].parse::<u8>().unwrap(),
            "Date": words[6],
        });
    }
    header
}

fn parse_point(words: Vec<&str>) -> serde_json::Value {
    let mut point = json!({});
    if words.len() == 6 {
        point = json!({
            "Time": words[0],
            "Indicator": words[1],
            "Grade": words[2],
            "Latitude": words[3].parse::<f64>().unwrap() / 10.0,
            "Longitude": words[4].parse::<f64>().unwrap() / 10.0,
            "Pressure": words[5].parse::<u32>().unwrap(),
        });
    }
    if words.len() == 7 {
        point = json!({
            "Time": words[0],
            "Indicator": words[1],
            "Grade": words[2],
            "Latitude": words[3].parse::<f64>().unwrap() / 10.0,
            "Longitude": words[4].parse::<f64>().unwrap() / 10.0,
            "Pressure": words[5].parse::<u32>().unwrap(),
            "MaxWindSpeed": words[6].parse::<u32>().unwrap(),
        });
    }
    if words.len() == 11 {
        let w7 = words[7];
        let dir50 = &w7[0..1];
        let longest_radius50 = &w7[2..5];
        let w9 = words[9];
        let dir30 = &w9[0..1];
        let longest_radius30 = &w9[2..5];
        point = json!({
            "Time": words[0],
            "Indicator": words[1],
            "Grade": words[2],
            "Latitude": words[3].parse::<f64>().unwrap() / 10.0,
            "Longitude": words[4].parse::<f64>().unwrap() / 10.0,
            "Pressure": words[5].parse::<u32>().unwrap(),
            "MaxWindSpeed": words[6].parse::<u32>().unwrap(),
            "Direction50": dir50.parse::<u8>().unwrap(),
            "LongestRadius50": longest_radius50.parse::<u16>().unwrap(),
            "ShortestRadius50": words[8].parse::<u16>().unwrap(),
            "Direction30": dir30.parse::<u8>().unwrap(),
            "LongestRadius30": longest_radius30.parse::<u16>().unwrap(),
            "ShortestRadius30": words[10].parse::<u16>().unwrap(),
        });
    }
    if words.len() == 12 {
        let w7 = words[7];
        let dir50 = &w7[0..1];
        let longest_radius50 = &w7[2..5];
        let w9 = words[9];
        let dir30 = &w9[0..1];
        let longest_radius30 = &w9[2..5];
        point = json!({
            "Time": words[0],
            "Indicator": words[1],
            "Grade": words[2],
            "Latitude": words[3].parse::<f64>().unwrap() / 10.0,
            "Longitude": words[4].parse::<f64>().unwrap() / 10.0,
            "Pressure": words[5].parse::<u32>().unwrap(),
            "MaxWindSpeed": words[6].parse::<u32>().unwrap(),
            "Direction50": dir50.parse::<u8>().unwrap(),
            "LongestRadius50": longest_radius50.parse::<u16>().unwrap(),
            "ShortestRadius50": words[8].parse::<u16>().unwrap(),
            "Direction30": dir30.parse::<u8>().unwrap(),
            "LongestRadius30": longest_radius30.parse::<u16>().unwrap(),
            "ShortestRadius30": words[10].parse::<u16>().unwrap(),
            "Landfall": true,
        });
    }
    point
}

fn parse_raw(path: PathBuf) -> serde_json::Value {
    // Spec format https://www.jma.go.jp/jma/jma-eng/jma-center/rsmc-hp-pub-eg/Besttracks/e_format_bst.html
    println!("parse raw. {:?}", path);
    let file = File::open(path).expect("file not found!");
    let reader = BufReader::new(file);
    let mut typhoons = json!([]);
    let mut typhoon = json!({});
    let mut points = json!([]);

    let mut first_flag = true;
    for line in reader.lines() {
        let _line = line.unwrap();
        let words: Vec<&str> = _line.split_whitespace().collect();
        if words[0] == "66666" {
            // Header
            if !first_flag {
                typhoon["points"] = points.clone();
                typhoon["header"]["Last"] = points[0]["Time"].clone();
                typhoons.as_array_mut().unwrap().push(typhoon.clone());
                points.as_array_mut().unwrap().clear();
                typhoon.as_object_mut().unwrap().clear();
            }
            let header = parse_header(words);
            typhoon["header"] = header;
            first_flag = false;
            continue;
        }
        // Point
        let point = parse_point(words);
        points.as_array_mut().unwrap().push(point);
    }
    typhoon["points"] = points.clone();
    typhoon["header"]["Last"] = points[0]["Time"].clone();
    typhoons.as_array_mut().unwrap().push(typhoon.clone());
    typhoons
}

pub(crate) async fn sync() -> Result<()> {
    // https://www.jma.go.jp/jma/jma-eng/jma-center/rsmc-hp-pub-eg/Besttracks/bst_all.zip
    println!("sync function called.");

    // asert application root
    let path_root = current_dir()?.join(".typhoon");
    create_dir_all(path_root.clone())?;

    // Get the file
    let target =
        "https://www.jma.go.jp/jma/jma-eng/jma-center/rsmc-hp-pub-eg/Besttracks/bst_all.zip";
    let response: Response = reqwest::get(target).await?;

    // Unzip data pack
    let content = Cursor::new(response.bytes().await?);
    println!("unzip path root: {:?}", path_root);
    let archive = zip::ZipArchive::new(content).unwrap();
    let ex_file = extract(archive, &path_root);
    println!("parsing data");
    let typhoons = parse_raw(ex_file);
    let f_data_path = path_root.join("data.json");
    fs::write(
        f_data_path,
        serde_json::to_string_pretty(&typhoons).unwrap(),
    )
    .unwrap();
    println!("syncd success");
    Ok(())
}

pub(crate) fn get() -> Result<serde_json::Value> {
    let file_name = "data.json";
    let path_root = current_dir()?.join(".typhoon");
    let _path = path_root.join(file_name);
    let file = OpenOptions::new()
        .read(true)
        .open(_path)
        .expect(&format!("{} should open readonly", file_name));
    let reader = BufReader::new(file);
    let json: serde_json::Value = match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(_) => {
            println!("file should be proper JSON");
            json!([])
        }
    };
    Ok(json)
}
