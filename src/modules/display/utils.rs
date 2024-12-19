use regex::Regex;
use unicode_width::UnicodeWidthStr;

use crate::modules::display::product::Product;

impl Product<'_> {
	pub fn trunc_address(mut address: String, max_width: usize) -> String {
		let address_len = address.chars().count();

		address = if address_len > max_width {
			// For most locations with overly long addresses, the results seem to be better if
			// truncated between the first and second comma instead the penultimate and last comma.
			// let last_comma = title.matches(',').count();
			let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
			let re = Regex::new(&prep_re).unwrap();

			re.replace(&address, "$1$2").to_string()
		} else {
			address
		};

		if address_len > max_width {
			address = Self::trunc_address(address, max_width);
		}

		address
	}
}

pub fn pad_string_to_width(s: &str, total_width: usize) -> String {
	let current_width = s.width(); // Effective width of the string
	if current_width >= total_width {
		s.to_string() // No padding needed if already wide enough
	} else {
		let padding = total_width - current_width;
		format!("{}{}", s, " ".repeat(padding))
	}
}

pub fn style_number(mut num: i32, sub: bool) -> String {
	const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
	const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

	let digits = if sub { SUBSCRIPT_DIGITS } else { SUPERSCRIPT_DIGITS };

	let mut result = String::new();

	if num == 0 {
		result.push(digits[0]);
		return result;
	}

	if num < 0 {
		num = -num;
		result.push(if sub { '₋' } else { '⁻' });
	}

	let mut started = false;
	let mut power_of_ten = 1_000_000_000;
	for _ in 0..10 {
		let digit = num / power_of_ten;
		num -= digit * power_of_ten;
		power_of_ten /= 10;
		if digit != 0 || started {
			started = true;
			result.push(digits[digit as usize]);
		}
	}

	result
}

#[cfg(test)]
pub mod common_tests {
	use crate::modules::config;
	use crate::modules::display::product::Product;
	use crate::modules::localization;
	use crate::modules::params::Params;
	use crate::modules::weather;
	use std::collections::{HashMap, HashSet};
	use std::sync::OnceLock;

	pub static TEST_PRODUCT: OnceLock<Product> = OnceLock::new();
	pub static TEST_PARAMS: OnceLock<Params> = OnceLock::new();

	#[allow(clippy::too_many_lines)]
	pub fn init_test_product() -> Product<'static> {
		Product {
			address: String::from("Berlin, Germany"),
			weather: weather::Weather {
				current_weather: weather::Current {
					temperature: 14.0,
					windspeed: 10.7,
					winddirection: 160.0,
					weathercode: 3,
					time: String::from("2024-10-07T13:15"),
				},
				hourly_units: weather::HourlyUnits {
					temperature_2m: String::from("°C"),
					relativehumidity_2m: String::from("%"),
					apparent_temperature: String::from("°C"),
					surface_pressure: String::from("hPa"),
					dewpoint_2m: String::from("°C"),
					windspeed_10m: String::from("km/h"),
					precipitation: String::from("mm"),
				},
				hourly: weather::Hourly {
					temperature_2m: vec![
						8.6, 8.2, 8.0, 7.8, 7.5, 7.2, 7.0, 6.8, 6.8, 7.3, 8.7, 10.6, 12.4, 13.7, 14.7, 15.1, 15.0,
						14.6, 14.4, 14.1, 13.9, 13.6, 13.4, 13.1, 12.7, 12.5, 12.3, 12.1, 12.0, 12.0, 12.2, 12.3, 12.6,
						13.2, 14.5, 16.0, 17.7, 19.0, 20.0, 20.3, 20.3, 20.1, 19.2, 18.4, 17.9, 17.3, 16.8, 16.4, 15.9,
						15.8, 15.7, 14.8, 14.4, 14.3, 14.1, 13.9, 13.7, 13.9, 14.5, 15.6, 16.9, 17.9, 17.1, 17.3, 17.1,
						16.4, 16.3, 15.9, 15.4, 14.9, 14.5, 14.3, 14.4, 14.4, 14.3, 14.1, 14.0, 14.0, 14.0, 14.1, 13.9,
						14.1, 14.9, 15.8, 18.1, 20.2, 19.1, 18.5, 17.2, 16.1, 15.5, 15.0, 14.6, 14.3, 14.0, 13.7, 13.2,
						12.5, 11.9, 11.1, 10.3, 9.6, 9.1, 8.6, 8.5, 8.8, 9.5, 10.1, 10.6, 11.0, 11.3, 11.4, 11.3, 11.1,
						10.8, 10.3, 9.7, 9.1, 8.5, 7.9, 7.4, 6.9, 6.5, 6.1, 5.8, 5.5, 5.2, 5.0, 4.8, 5.8, 7.4, 8.8,
						10.1, 11.2, 12.0, 12.2, 12.1, 11.8, 11.4, 10.8, 10.3, 10.0, 9.8, 9.6, 9.4, 9.3, 9.1, 9.0, 8.8,
						8.6, 8.5, 8.4, 8.4, 8.5, 8.7, 9.1, 9.7, 10.5, 10.9, 10.5, 9.8, 9.0, 8.4, 7.8, 7.2, 6.7, 6.1,
						5.6,
					],
					relativehumidity_2m: vec![
						96.0, 96.0, 95.0, 95.0, 95.0, 95.0, 95.0, 95.0, 95.0, 91.0, 85.0, 80.0, 75.0, 72.0, 73.0, 74.0,
						78.0, 85.0, 89.0, 91.0, 93.0, 94.0, 95.0, 96.0, 97.0, 97.0, 97.0, 96.0, 96.0, 95.0, 94.0, 95.0,
						95.0, 94.0, 88.0, 82.0, 76.0, 74.0, 72.0, 71.0, 72.0, 74.0, 80.0, 86.0, 87.0, 90.0, 92.0, 94.0,
						97.0, 99.0, 95.0, 90.0, 90.0, 97.0, 93.0, 93.0, 97.0, 95.0, 91.0, 83.0, 73.0, 65.0, 72.0, 70.0,
						72.0, 78.0, 76.0, 76.0, 77.0, 82.0, 88.0, 90.0, 89.0, 86.0, 85.0, 85.0, 84.0, 84.0, 85.0, 85.0,
						87.0, 87.0, 84.0, 80.0, 72.0, 61.0, 53.0, 53.0, 57.0, 61.0, 64.0, 66.0, 69.0, 73.0, 77.0, 79.0,
						77.0, 74.0, 71.0, 70.0, 70.0, 71.0, 72.0, 74.0, 74.0, 72.0, 69.0, 66.0, 64.0, 64.0, 63.0, 63.0,
						63.0, 64.0, 65.0, 67.0, 69.0, 72.0, 75.0, 78.0, 81.0, 84.0, 87.0, 89.0, 91.0, 91.0, 90.0, 88.0,
						86.0, 80.0, 73.0, 67.0, 62.0, 57.0, 54.0, 53.0, 53.0, 55.0, 59.0, 64.0, 68.0, 70.0, 72.0, 73.0,
						75.0, 77.0, 79.0, 81.0, 83.0, 85.0, 87.0, 90.0, 91.0, 92.0, 92.0, 90.0, 85.0, 78.0, 73.0, 72.0,
						72.0, 73.0, 74.0, 76.0, 77.0, 76.0, 75.0, 75.0,
					],
					apparent_temperature: vec![
						7.1, 6.6, 6.1, 5.7, 5.4, 5.0, 4.7, 4.3, 4.7, 4.9, 6.3, 8.6, 10.4, 11.8, 13.0, 14.1, 14.5, 14.3,
						14.4, 14.2, 14.0, 13.6, 13.4, 13.3, 12.6, 12.3, 11.9, 11.6, 11.4, 11.4, 11.5, 11.6, 11.8, 12.5,
						13.8, 15.5, 17.4, 18.9, 20.3, 20.9, 21.1, 20.8, 20.5, 19.9, 19.6, 18.6, 18.3, 17.9, 17.4, 17.3,
						15.2, 13.9, 13.7, 14.4, 14.2, 13.9, 13.9, 13.9, 14.1, 14.5, 15.8, 16.3, 15.9, 15.8, 15.8, 15.4,
						15.3, 14.8, 14.4, 14.2, 14.5, 14.1, 14.1, 13.8, 13.8, 13.8, 13.3, 12.9, 13.0, 12.9, 12.9, 12.7,
						13.5, 14.6, 16.7, 17.7, 15.5, 14.5, 13.1, 12.1, 11.5, 11.0, 10.7, 10.7, 10.9, 10.7, 10.0, 9.0,
						8.1, 7.2, 6.4, 5.7, 5.2, 4.8, 4.7, 5.1, 5.8, 6.3, 6.8, 7.3, 7.6, 7.9, 8.1, 8.1, 7.9, 7.5, 7.1,
						6.5, 5.9, 5.3, 4.8, 4.4, 4.1, 3.7, 3.5, 3.1, 2.8, 2.4, 2.2, 2.9, 4.4, 5.7, 6.8, 7.8, 8.5, 8.8,
						8.9, 8.9, 8.8, 8.4, 7.9, 7.6, 7.4, 7.3, 7.3, 7.3, 7.2, 7.1, 7.0, 6.9, 6.9, 6.9, 7.0, 7.1, 7.1,
						7.0, 7.3, 7.6, 7.7, 7.2, 6.2, 5.2, 4.9, 4.7, 4.2, 3.7, 3.1, 2.6,
					],
					surface_pressure: vec![
						1005.4, 1005.3, 1004.9, 1004.3, 1003.7, 1003.0, 1002.8, 1002.4, 1001.9, 1002.4, 1002.4, 1002.3,
						1002.3, 1001.3, 1001.5, 1002.0, 1001.9, 1001.4, 1001.7, 1001.8, 1001.6, 1001.7, 1001.8, 1002.0,
						1001.7, 1001.4, 1000.8, 1000.5, 1000.5, 1000.0, 999.4, 998.9, 998.5, 998.6, 998.4, 998.5,
						998.2, 997.9, 997.3, 997.0, 996.8, 996.2, 996.2, 996.4, 996.3, 995.6, 995.8, 995.2, 994.7,
						994.2, 994.2, 994.4, 994.7, 994.5, 994.4, 994.8, 994.8, 994.8, 995.0, 994.8, 993.8, 993.2,
						993.0, 992.6, 992.1, 991.6, 991.5, 991.3, 991.0, 990.8, 990.3, 989.6, 989.1, 988.6, 987.9,
						987.0, 985.9, 984.7, 983.9, 983.4, 983.6, 983.3, 982.8, 982.1, 981.8, 981.0, 980.9, 980.5,
						980.3, 980.4, 981.0, 982.1, 983.4, 985.0, 986.8, 988.6, 990.3, 991.8, 993.2, 994.4, 995.3,
						996.3, 997.3, 998.1, 999.1, 1000.1, 1001.1, 1002.0, 1002.8, 1003.5, 1004.2, 1004.8, 1005.4,
						1006.0, 1006.7, 1007.5, 1008.3, 1009.1, 1009.9, 1010.6, 1011.2, 1011.7, 1012.1, 1012.6, 1013.2,
						1013.6, 1013.9, 1014.2, 1014.4, 1012.7, 1012.9, 1012.8, 1012.4, 1011.7, 1011.1, 1010.7, 1010.3,
						1009.7, 1009.2, 1008.7, 1008.1, 1007.4, 1006.8, 1006.1, 1005.5, 1005.0, 1004.5, 1003.8, 1003.2,
						1002.6, 1002.2, 1001.9, 1001.9, 1002.1, 1002.5, 1003.1, 1003.7, 1004.5, 1005.5, 1006.8, 1008.4,
						1010.1, 1011.9, 1013.9, 1015.4, 1016.5, 1017.4, 1018.2,
					],
					dewpoint_2m: vec![
						8.0, 7.6, 7.3, 7.1, 6.8, 6.4, 6.2, 6.0, 6.1, 6.0, 6.3, 7.3, 8.1, 8.7, 9.9, 10.5, 11.2, 12.1,
						12.6, 12.6, 12.8, 12.6, 12.6, 12.5, 12.3, 12.1, 11.9, 11.5, 11.4, 11.3, 11.2, 11.6, 11.9, 12.2,
						12.6, 13.0, 13.4, 14.3, 14.8, 14.9, 15.1, 15.3, 15.7, 16.0, 15.7, 15.6, 15.5, 15.4, 15.5, 15.7,
						14.9, 13.2, 12.8, 13.8, 13.0, 12.8, 13.3, 13.1, 13.1, 12.8, 12.0, 11.2, 12.0, 11.7, 12.0, 12.5,
						12.0, 11.6, 11.4, 11.8, 12.6, 12.7, 12.6, 12.1, 11.8, 11.6, 11.4, 11.3, 11.5, 11.6, 11.8, 12.0,
						12.2, 12.4, 12.9, 12.4, 9.3, 8.8, 8.6, 8.6, 8.7, 8.7, 9.0, 9.5, 10.0, 10.1, 9.2, 8.0, 6.8, 5.9,
						5.1, 4.6, 4.3, 4.2, 4.1, 4.0, 4.1, 4.1, 4.1, 4.5, 4.5, 4.6, 4.5, 4.6, 4.5, 4.4, 4.3, 4.3, 4.3,
						4.3, 4.3, 4.4, 4.5, 4.4, 4.4, 4.1, 3.7, 3.1, 2.7, 2.6, 2.8, 3.0, 3.1, 3.0, 2.9, 2.9, 2.8, 3.1,
						3.6, 4.3, 4.7, 4.8, 4.9, 5.0, 5.2, 5.4, 5.7, 5.9, 6.0, 6.2, 6.4, 6.8, 7.0, 7.2, 7.5, 7.5, 7.3,
						6.8, 6.2, 5.7, 4.9, 4.4, 4.0, 3.8, 3.4, 2.7, 2.0, 1.5,
					],
					precipitation: vec![
						0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.0,
						0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
						0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.5, 1.4, 2.4, 3.8, 2.0, 1.5, 0.7, 0.0, 0.0,
						0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.4, 0.2, 0.4, 0.3, 0.1,
						0.0, 0.0, 0.0, 0.1, 0.5, 0.1, 0.1, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
						0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.1, 0.1,
						0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
						0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.7, 0.7,
						0.7, 0.6, 0.6, 0.6, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
					],
					precipitation_probability: vec![
						0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 25, 25, 10, 5, 0, 0, 0, 0, 0, 0, 0, 3, 8, 20, 50, 75,
						88, 65, 33, 13, 3, 3, 3, 0, 0, 0, 0, 3, 13, 43, 53, 70, 78, 75, 83, 88, 80, 98, 90, 83, 68, 63,
						38, 30, 24, 20, 17, 15, 15, 19, 24, 28, 27, 25, 23, 22, 22, 23, 27, 33, 38, 40, 42, 43, 48, 50,
						50, 49, 46, 43, 39, 36, 33, 31, 29, 27, 26, 24, 23, 22, 22, 22, 21, 21, 20, 18, 16, 14, 11, 9,
						8, 7, 7, 7, 7, 7, 8, 9, 12, 14, 16, 18, 18, 17, 14, 11, 8, 5, 3, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2,
						3, 3, 4, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 18, 19, 20, 22, 23, 25, 26, 27, 28, 29, 30,
						31, 31, 32, 32, 32, 32, 32, 32, 32,
					],
					weathercode: vec![
						1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 80, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 61, 61,
						61, 3, 3, 3, 3, 2, 1, 2, 3, 3, 3, 3, 61, 3, 3, 3, 61, 61, 61, 61, 63, 61, 61, 61, 3, 3, 3, 3,
						2, 1, 3, 3, 3, 3, 3, 3, 3, 3, 61, 61, 61, 61, 61, 61, 3, 3, 3, 3, 3, 61, 61, 61, 61, 3, 3, 2,
						3, 3, 3, 3, 3, 3, 61, 61, 61, 3, 3, 3, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 1,
						1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 61, 61,
						61, 61, 61, 61, 61, 61, 61, 80, 80, 80, 61, 61, 61, 0, 0, 0, 0, 0, 0,
					],
				},
				daily_units: weather::DailyUnits {
					temperature_2m_max: String::from("°C"),
					temperature_2m_min: String::from("°C"),
				},
				daily: weather::Daily {
					time: vec![
						String::from("2024-10-07"),
						String::from("2024-10-08"),
						String::from("2024-10-09"),
						String::from("2024-10-10"),
						String::from("2024-10-11"),
						String::from("2024-10-12"),
						String::from("2024-10-13"),
					],
					weathercode: vec![80, 61, 63, 61, 3, 3, 80],
					sunrise: vec![
						String::from("2024-10-07T07:18"),
						String::from("2024-10-08T07:20"),
						String::from("2024-10-09T07:22"),
						String::from("2024-10-10T07:24"),
						String::from("2024-10-11T07:25"),
						String::from("2024-10-12T07:27"),
						String::from("2024-10-13T07:29"),
					],
					sunset: vec![
						String::from("2024-10-07T18:29"),
						String::from("2024-10-08T18:26"),
						String::from("2024-10-09T18:24"),
						String::from("2024-10-10T18:22"),
						String::from("2024-10-11T18:20"),
						String::from("2024-10-12T18:17"),
						String::from("2024-10-13T18:15"),
					],
					temperature_2m_max: vec![15.1, 20.3, 17.9, 20.2, 13.2, 12.2, 10.9],
					temperature_2m_min: vec![6.8, 12.0, 13.7, 13.7, 7.9, 4.8, 5.6],
					apparent_temperature_max: vec![14.5, 21.1, 17.4, 17.7, 10.0, 8.9, 7.7],
					apparent_temperature_min: vec![4.3, 11.4, 13.7, 10.7, 4.7, 2.2, 2.6],
					precipitation_probability_max: vec![25, 88, 98, 50, 21, 11, 32],
					precipitation_sum: None,
				},
			},
			historical_weather: HashMap::new(),
		}
	}

	pub fn init_test_params() -> Params {
		Params {
			config: config::Config::default(),
			texts: localization::Locales::default(),
			historical_weather: HashSet::new(),
		}
	}
}
