static LAST_PRINT_VALUE: RwLock<Option<String>> = std::sync::RwLock::new(None);

fn print_sparsely(new_value: String) {
    if let Ok(old_value) = LAST_PRINT_VALUE.read()
        && old_value.as_ref() == Some(&new_value)
    {
        return;
    }

    match LAST_PRINT_VALUE.write() {
        Ok(mut writer) => {
            println!("{new_value}");
            *writer = Some(new_value)
        }
        Err(_) => println!("Writer thread is poisoned, no values will printed anymore!"),
    }
}

static LAST_DIGIT_VALUE: RwLock<usize> = std::sync::RwLock::new(usize::MAX);

fn print_lower(new_value: usize) {
    if let Ok(old_value) = LAST_DIGIT_VALUE.read()
        && new_value > *old_value
    {
        return;
    }

    match LAST_DIGIT_VALUE.write() {
        Ok(mut writer) => {
            println!("{new_value}");
            *writer = new_value
        }
        Err(_) => println!("Writer thread is poisoned, no values will printed anymore!"),
    }
}
