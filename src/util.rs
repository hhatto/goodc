macro_rules! set_ok_color {
    ($a:ident) => {
        $a.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(77))))
            .unwrap();
    };
}

macro_rules! set_fail_color {
    ($a:ident) => {
        $a.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(9))))
            .unwrap();
    };
}

macro_rules! set_normal_color {
    ($a:ident) => {
        $a.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(255))))
            .unwrap();
    };
}
