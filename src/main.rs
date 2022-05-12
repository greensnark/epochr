use anyhow::Result;
use std::env;
use std::time;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", current_epoch_millis()?);
        return Ok(());
    }

    let empty_arg = String::from("");
    if args.len() == 2 && args.iter().nth(1).unwrap_or_else(|| &empty_arg) == "--rfc-3339" {
        println!("{}", current_rfc3339_time());
        return Ok(());
    }

    for arg in args.iter().skip(1) {
        println!("{}", convert_arg_epoch_rfc3339(&arg)?);
    }

    Ok(())
}

fn convert_arg_epoch_rfc3339(arg: &str) -> Result<String> {
    arg.parse::<u128>()
        .map(|epoch_sms| unix_to_datetime(force_to_epoch_millis(epoch_sms)).to_rfc3339())
        .or_else(|_| {
            Ok({
                let time = chrono::DateTime::parse_from_rfc3339(arg)?;
                ((time.timestamp() as u128) * 1000 + (time.timestamp_subsec_millis() as u128))
                    .to_string()
            })
        })
}

fn unix_to_datetime(unix_time: u128) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(
            (unix_time / 1000) as i64,
            ((unix_time % 1000) * 1_000_000) as u32,
        ),
        chrono::Utc,
    )
}

fn force_to_epoch_millis(epoch_sms: u128) -> u128 {
    if epoch_sms > 10000000000 {
        epoch_sms
    } else {
        epoch_sms * 1000
    }
}

fn current_epoch_millis() -> Result<u128, time::SystemTimeError> {
    Ok(time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)?
        .as_millis())
}

fn current_rfc3339_time() -> String {
    chrono::Local::now().to_rfc3339()
}
