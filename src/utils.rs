pub fn remove_vec<T: std::cmp::PartialEq>(data: &mut Vec<T>, value: &T) -> bool {
	let mut index = -1;
	for (i, v) in data.iter().enumerate() {
		if *v == *value {
			index = i as i32;
			break;
		}
	}

	if index != -1 {
		data.remove(index as usize);
		true
	} else {
		false
	}
}
