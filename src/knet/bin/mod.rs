mod util;

use std::fs::File;
use std::io::{self, BufReader, Read};
use util::*;

#[derive(Debug)]
pub struct KnetBinEarthquakeData {}

#[allow(dead_code)]
pub fn load_knet(mut reader: BufReader<File>) -> io::Result<KnetBinEarthquakeData> {
    // https://www.kyoshin.bosai.go.jp/kyoshin/man/knetbinary.html

    // 強震WIN32ヘッダブロック
    let kyoshin_win32_header = read_bytes(&mut reader, 4);
    if kyoshin_win32_header != [0x0A, 0x02, 0x00, 0x00] {
        panic!("Only K-Net format is supported.");
    }

    // 情報ブロック
    // 情報ブロックヘッダ
    let information_metadata = read_bytes(&mut reader, 12);
    println!("information_metadata: {:02x?}", information_metadata);
    // 情報データブロック（1）　　観測点情報（3成分地表観測点）
    let station_metadata = read_bytes(&mut reader, 4);
    println!("station_metadata: {:02x?}", station_metadata);

    // 観測点に関する情報
    // 緯度
    let latitude = read_latitude(&mut reader);
    println!("latitude: {}", latitude);
    // 経度
    let longitude = read_longitude(&mut reader);
    println!("longitude: {}", longitude);
    // 標高
    let altitude = read_altitude(&mut reader);
    println!("altitude: {}", altitude);
    // 観測点コード
    let station_code = read_ascii(&mut reader, 12);
    println!("station_code: {}", station_code);
    // データ開始時刻
    let data_start_timestamp = read_bytes(&mut reader, 8);
    println!("data_start_timestamp: {:02x?}", data_start_timestamp);
    // 計測時間長
    let data_time_length = read_u32(&mut reader);
    println!("data_time_length: {:02x?}", data_time_length);
    // 最終時刻校正時刻
    let data_corrected_timestamp = read_bytes(&mut reader, 8);
    println!(
        "data_corrected_timestamp: {:02x?}",
        data_corrected_timestamp
    );
    // 校正手段
    let correction_type = read_bytes(&mut reader, 1);
    println!("correction_type: {:02x?}", correction_type);
    // 測地系
    let geodetic_system = read_bytes(&mut reader, 1);
    println!("geodetic_system: {:02x?}", geodetic_system);
    // 地震計機種コード
    let seismograph_model_code = read_bytes(&mut reader, 2);
    println!("seismograph_model_code: {:02x?}", seismograph_model_code);
    // サンプリングレート
    let sampling_rate = read_u16(&mut reader);
    println!("sampling_rate: {}", sampling_rate);
    // 成分数
    let component_count = read_u8(&mut reader);
    println!("component_count: {}", component_count);
    // 移設フラグ
    let relocation_count = read_u8(&mut reader);
    println!("relocation_count: {}", relocation_count);

    // 南北成分（NS）に関する情報
    let ns_metadata = read_bytes(&mut reader, 20);
    println!("ns_metadata: {:02x?}", ns_metadata);

    // 東西成分（EW）に関する情報
    let ew_metadata = read_bytes(&mut reader, 20);
    println!("ew_metadata: {:02x?}", ew_metadata);

    // 上下成分（UD）に関する情報
    let ud_metadata = read_bytes(&mut reader, 20);
    println!("ud_metadata: {:02x?}", ud_metadata);

    // 情報データブロック（2）　　震源情報 ※即時公開データの場合はこの情報データブロックは無い
    // 情報種別
    let hypocenter_type = read_bytes(&mut reader, 2);
    println!("hypocenter_type: {:02x?}", hypocenter_type);
    // 情報データサイズ
    let hypocenter_data_length = read_u16(&mut reader);
    println!("hypocenter_data_length: {}", hypocenter_data_length);

    // 震源に関する情報
    let hypocenter = read_bytes(&mut reader, hypocenter_data_length as usize);
    println!("hypocenter: {:02x?}", hypocenter);

    // 秒ブロック
    // 秒ブロックヘッダ
    // サンプリング先頭時刻
    let sampling_start_timestamp = read_bytes(&mut reader, 8);
    println!(
        "sampling_start_timestamp: {:02x?}",
        sampling_start_timestamp
    );
    // フレーム時間長
    let channel_frame_time_length = read_u32(&mut reader);
    println!("channel_frame_time_length: {}", channel_frame_time_length);
    // データブロック長
    let channel_metadata_data_length = read_u32(&mut reader);
    println!(
        "channel_metadata_data_length: {}",
        channel_metadata_data_length
    );

    // チャンネルデータブロック　　1チャンネル目（NS）
    // 組織ID 観測網ID チャンネル番号
    let channel_header = read_bytes(&mut reader, 4);
    println!("{:02x?}", channel_header);
    // 圧縮データ
    let channel_data = read_bytes(&mut reader, channel_metadata_data_length as usize);
    println!("{:02x?}", channel_data);

    // // チャンネルデータブロック　　2チャンネル目（EW）
    // // 組織ID 観測網ID チャンネル番号
    // let channel_header = read_bytes(&mut reader, 4);
    // println!("{:02x?}", channel_header);
    // // 圧縮データ
    // let channel_data = read_bytes(&mut reader, channel_metadata_data_length as usize);
    // println!("{:02x?}", channel_data);

    // // チャンネルデータブロック　　3チャンネル目（UD）
    // // 組織ID 観測網ID チャンネル番号
    // let channel_header = read_bytes(&mut reader, 4);
    // println!("{:02x?}", channel_header);
    // // 圧縮データ
    // let channel_data = read_bytes(&mut reader, channel_metadata_data_length as usize);
    // println!("{:02x?}", channel_data);

    let bytes = reader.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
    println!(
        "next [0x01, 0x10, 0x2f] position: {:?}",
        bytes
            .windows(3)
            .position(|window| window == [0x01, 0x10, 0x2f])
    );

    Ok(KnetBinEarthquakeData {})
}
