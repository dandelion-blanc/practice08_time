/* practice08_time by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 * 時間に関する基礎的な型について
 * 歴やタイムゾーンといった発展的な型はcrate chronoを参照
 * std以外にcrate timeもあるが，chronoを弱めたレベル
 *
 */

use std::*;


fn main()
{
	let sleeptime :u64 = 1200;		// [ms]
	let start_real = time::SystemTime::now();
	let start_cpu = time::Instant::now();

	thread::sleep(time::Duration::from_millis(sleeptime));

	let end_cpu = start_cpu.elapsed();
	let end_real = start_real.elapsed().unwrap();

	println!("sleep time : {:.5}[sec] ({ }[msec])", sleeptime as f64 /1000.0, sleeptime);
	println!("cpu time   : { }.{:05}[sec]", end_cpu.as_secs(), end_cpu.as_millis());
	println!("real time   : { }.{:05}[sec]", end_real.as_secs(), end_real.as_millis())

//冪乗に関する速度比較(powf・powi・pow)

}

#[cfg(test)]
mod tests {
	
	#[test]
	fn test_duration_div() {
		let t = std::time::Duration::from_millis(1200);
		println!("{:?}", t.checked_div(7).unwrap());
	}
}