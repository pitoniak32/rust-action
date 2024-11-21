#[macro_export]
macro_rules! debug {
    () => {
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: "".to_string(),
                msg_type: crate::github::command::MsgType::Debug,
            },
        }
        .build());
    };
    ($($arg:tt)*) => {{
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: format!($($arg)*),
                msg_type: crate::github::command::MsgType::Debug,
            },
        }
        .build());
    }};
}

#[macro_export]
macro_rules! notice {
    () => {
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: "".to_string(),
                msg_type: crate::github::command::MsgType::Notice { parameters: vec![] },
            },
        }
        .build());
    };
    ($($arg:tt)*) => {{
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: format!($($arg)*),
                msg_type: crate::github::command::MsgType::Notice { parameters: vec![] },
            },
        }
        .build());
    }};
}

#[macro_export]
macro_rules! warn {
    () => {
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: "".to_string(),
                msg_type: crate::github::command::MsgType::Warning { parameters: vec![] },
            },
        }
        .build());
    };
    ($($arg:tt)*) => {{
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: format!($($arg)*),
                msg_type: crate::github::command::MsgType::Warning { parameters: vec![] },
            },
        }
        .build());
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: "".to_string(),
                msg_type: crate::github::command::MsgType::Error { parameters: vec![] },
            },
        }
        .build());
    };
    ($($arg:tt)*) => {{
        println!("{}", crate::github::command::GhaCommand {
            cmd_type: crate::github::command::CommandType::Msg {
                msg: format!($($arg)*),
                msg_type: crate::github::command::MsgType::Error { parameters: vec![] },
            },
        }
        .build());
    }};
}
