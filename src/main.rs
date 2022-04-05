extern crate log;
extern crate simplelog;
use std::time::Duration;
use std::fs;
use simplelog::*;

fn test1() {
    let klens = [1, 2, 4, 8, 16, 32, 64];
    let n = 100;
    for klen in klens {
        let mut total_enc = Duration::new(0, 0);
        let mut total_dec = Duration::new(0, 0);
        for _i in 0..n {
            let (enc, dec) = libsm::sm2::encrypt::sm2_encrypt_decrypt_test(klen);
            total_enc += enc;
            total_dec += dec;
            log::info!("[sm2] {} bit, encrypt elapsed: {:.2?}", klen * 8, enc);
            log::info!("[sm2] {} bit, decrypt elapsed: {:.2?}", klen * 8, enc);
        }
        let total_enc_millis = total_enc.as_secs_f64();
        let avg_enc_f64 = total_enc_millis / (n as f64);
        let avg_enc = Duration::from_secs_f64(avg_enc_f64);

        let total_dec_millis = total_dec.as_secs_f64();
        let avg_dec_f64 = total_dec_millis / (n as f64);
        let avg_dec = Duration::from_secs_f64(avg_dec_f64);

        log::info!("[sm2] {} bit, average encrypt elapsed: {:.2?}", klen * 8, avg_enc);
        log::info!("[sm2] {} bit, average decrypt elapsed: {:.2?}", klen * 8, avg_dec);
    }
}

fn test2() {
    let n = 100;
    let mut total_duration = Duration::new(0, 0);
    for _ in 0..n {
        let duration = libsm::sm2::encrypt::sm2_generate_keypair_test();
        log::info!("[sm2] keypair generation elapsed {:.2?}", duration);
        total_duration += duration;
    }
    let duration_millis = total_duration.as_secs_f64();
    let avg_duration_f64 = duration_millis / (n as f64);
    let avg_duration = Duration::from_secs_f64(avg_duration_f64);
    log::info!("average duration: {:.2?}", avg_duration);
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Info, Config::default(), fs::File::create("sm2-keypair.log").unwrap()),
    ]).unwrap();
    
    test2();
}
