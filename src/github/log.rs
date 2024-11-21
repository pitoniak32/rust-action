#[macro_export]
macro_rules! log_debug {
    () => {
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: "".to_string(),
                msg_type: MsgType::Debug,
            },
        }
        println!(t);
    };
    ($($arg:tt)*) => {{
        let m = format!($($arg)*);
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: m,
                msg_type: MsgType::Debug,
            },
        }
        .build();
        println!("{}", t);
    }};
}

#[macro_export]
macro_rules! log_notice {
    () => {
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: "".to_string(),
                msg_type: MsgType::Notice { parameters: vec![] },
            },
        }
        println!(t);
    };
    ($($arg:tt)*) => {{
        let m = format!($($arg)*);
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: m,
                msg_type: MsgType::Notice { parameters: vec![] },
            },
        }
        .build();
        println!("{}", t);
    }};
}

#[macro_export]
macro_rules! log_warn {
    () => {
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: "".to_string(),
                msg_type: MsgType::Warning { parameters: vec![] },
            },
        }
        println!(t);
    };
    ($($arg:tt)*) => {{
        let m = format!($($arg)*);
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: m,
                msg_type: MsgType::Warning { parameters: vec![] },
            },
        }
        .build();
        println!("{}", t);
    }};
}

#[macro_export]
macro_rules! log_error {
    () => {
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: "".to_string(),
                msg_type: MsgType::Error { parameters: vec![] },
            },
        }
        println!(t);
    };
    ($($arg:tt)*) => {{
        let m = format!($($arg)*);
        let t = GhaCommand {
            cmd_type: CommandType::Msg {
                msg: m,
                msg_type: MsgType::Error { parameters: vec![] },
            },
        }
        .build();
        println!("{}", t);
    }};
}
