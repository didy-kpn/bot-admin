extern crate clap;

enum Command {
    Add,
    Update,
    Remove,
    Get,
    List,
    NoCommand,
}

struct Config {
    command: Command,
    option: std::collections::HashMap<String, String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Bot {
    id: i64,
    name: String,
    description: String,
    enable: bool,
    registered: i64,
    token: String,
    long_order: bool,
    short_order: bool,
    operate_type: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct BotList {
    bot: Vec<Bot>,
}

fn main() {
    // コマンドライン引数を取得する
    let args_matches = get_args_matches();

    // 実行コマンドとオプションを取得する
    let config = get_config(args_matches);

    // コマンドを実行する
    let result = actual_main(config);

    // 終了する
    std::process::exit(result);
}

// コマンドライン引数を取得する
fn get_args_matches() -> clap::ArgMatches<'static> {
    clap::App::new("bot-admin")
        .version("0.0.1")
        .author("Didy KUPANHY")
        .about("bot管理コマンド")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .subcommand(
            clap::SubCommand::with_name("add")
                .about("管理するbotを追加する")
                .setting(clap::AppSettings::DeriveDisplayOrder)
                .arg(
                    clap::Arg::with_name("name")
                        .help("bot名")
                        .long("name")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("description")
                        .help("botの説明")
                        .long("description")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("enable")
                        .help("有効かどうか")
                        .long("enable")
                        .possible_values(&["true", "false"])
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("long_order")
                        .help("ロング注文可能かどうか")
                        .long("long_order")
                        .possible_values(&["true", "false"])
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("short_order")
                        .help("ショート注文可能かどうか")
                        .long("short_order")
                        .possible_values(&["true", "false"])
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("operation")
                        .help("運用")
                        .long("operation")
                        .possible_values(&["backtest", "forwardtest", "product"])
                        .takes_value(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("update")
                .about("対象botの情報を更新する")
                .setting(clap::AppSettings::DeriveDisplayOrder)
                .arg(clap::Arg::with_name("id").help("対象bot id").required(true))
                .arg(
                    clap::Arg::with_name("name")
                        .help("bot名")
                        .long("name")
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("description")
                        .help("botの説明")
                        .long("description")
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("enable")
                        .help("有効かどうか")
                        .long("enable")
                        .possible_values(&["true", "false"])
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("long_order")
                        .help("ロング注文可能かどうか")
                        .long("long_order")
                        .possible_values(&["true", "false"])
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("short_order")
                        .help("ショート注文可能かどうか")
                        .long("short_order")
                        .possible_values(&["true", "false"])
                        .takes_value(true),
                )
                .arg(
                    clap::Arg::with_name("operation")
                        .help("運用")
                        .long("operation")
                        .possible_values(&["backtest", "forwardtest", "product"])
                        .takes_value(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("remove")
                .about("管理から対象botを取り除く")
                .arg(clap::Arg::with_name("id").help("対象bot id").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("get")
                .about("対象botを取得する")
                .args_from_usage(
                    "-j, --json 'json mode: output group'
                                  -y, --yaml 'yaml mode: output group'",
                )
                .arg(clap::Arg::with_name("id").help("対象bot id").required(true))
                .group(clap::ArgGroup::with_name("output").args(&["json", "yaml"])),
        )
        .subcommand(
            clap::SubCommand::with_name("list")
                .about("管理してるbot一覧")
                .args_from_usage(
                    "-j, --json 'json mode: output group'
                                  -y, --yaml 'yaml mode: output group'",
                )
                .group(
                    clap::ArgGroup::with_name("output")
                        .args(&["json", "yaml"])
                        .required(true),
                ),
        )
        .get_matches()
}

// 実行コマンドを取得する
fn get_config(args_matches: clap::ArgMatches<'static>) -> Config {
    let mut option = std::collections::HashMap::new();

    // Addコマンドのオプション取得
    if let Some(ref args_matches) = args_matches.subcommand_matches("add") {
        // サブコマンドのオプションのリスト
        let must_keys = vec!["name", "description", "enable"];
        let optional_keys = vec!["long_order", "short_order", "operation"];

        // 必須オプションを取得
        for key in must_keys {
            option.insert(
                String::from(key),
                args_matches.value_of(key).unwrap().to_string(),
            );
        }

        // 任意オプションを取得
        for key in optional_keys {
            if let Some(opt) = args_matches.value_of(key) {
                option.insert(String::from(key), opt.to_string());
            }
        }

        return Config {
            command: Command::Add,
            option: option,
        };
    }

    // Updateコマンドのオプション取得
    if let Some(ref args_matches) = args_matches.subcommand_matches("update") {
        // サブコマンドのオプションのリスト
        let must_keys = vec!["id"];
        let optional_keys = vec![
            "id",
            "name",
            "description",
            "enable",
            "long_order",
            "short_order",
            "operation",
        ];

        // 必須オプションを取得
        for key in must_keys {
            option.insert(
                String::from(key),
                args_matches.value_of(key).unwrap().to_string(),
            );
        }

        // 任意オプションを取得
        for key in optional_keys {
            if let Some(opt) = args_matches.value_of(key) {
                option.insert(String::from(key), opt.to_string());
            }
        }

        return Config {
            command: Command::Update,
            option: option,
        };
    }

    // Removeコマンドのオプション取得
    if let Some(ref args_matches) = args_matches.subcommand_matches("remove") {
        // サブコマンドのオプションのリスト
        let must_keys = vec!["id"];

        // 必須オプションを取得
        for key in must_keys {
            option.insert(
                String::from(key),
                args_matches.value_of(key).unwrap().to_string(),
            );
        }

        return Config {
            command: Command::Remove,
            option: option,
        };
    }

    // Getコマンドのオプション取得
    if let Some(ref args_matches) = args_matches.subcommand_matches("get") {
        // サブコマンドのオプションのリスト
        let must_keys = vec!["id"];

        // 必須オプションを取得
        for key in must_keys {
            option.insert(
                String::from(key),
                args_matches.value_of(key).unwrap().to_string(),
            );
        }

        // jsonかyamlオプションのいずれか取得
        if args_matches.is_present("json") {
            option.insert(String::from("json"), "1".to_string());
        } else {
            option.insert(String::from("yaml"), "1".to_string());
        }

        return Config {
            command: Command::Get,
            option: option,
        };
    }

    // Listコマンドのオプション取得
    if let Some(ref args_matches) = args_matches.subcommand_matches("list") {
        // jsonかyamlオプションのいずれか
        if args_matches.is_present("json") {
            option.insert(String::from("json"), "1".to_string());
        } else {
            option.insert(String::from("yaml"), "1".to_string());
        }

        return Config {
            command: Command::List,
            option: option,
        };
    }

    Config {
        command: Command::NoCommand,
        option: option,
    }
}

// コマンドを実行する
fn actual_main(config: Config) -> i32 {
    let result = match config.command {
        Command::Add => _add(&config.option),
        Command::Update => _update(&config.option),
        Command::Remove => _remove(config.option),
        Command::Get => _get(config.option),
        Command::List => _list(config.option),
        _ => Err("".to_string()),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        1
    } else {
        0
    }
}

fn _get_db_path() -> Result<String, std::io::Error> {
    std::fs::read_to_string(".bot-admin")
}

// Addコマンドを実行する
fn _add(option: &std::collections::HashMap<String, String>) -> Result<usize, String> {
    let mut option = option.clone();

    // enable値を1/0に変換する
    {
        let value = if option.get("enable") == Some(&"true".to_string()) {
            "1"
        } else {
            "0"
        }
        .to_string();
        option.insert("enable".to_string(), value);
    }

    // long_orderが指定されていれば、値を1/0に変換する
    if let Some(v) = option.get("long_order") {
        let value = if v == "true" { "1" } else { "0" }.to_string();
        option.insert("long_order".to_string(), value);
    }

    // short_orderが指定されていれば、値を1/0に変換する
    if let Some(v) = option.get("short_order") {
        let value = if v == "true" { "1" } else { "0" }.to_string();
        option.insert("short_order".to_string(), value);
    }

    // registeredに現在の日時を追加
    {
        let value = chrono::Utc::now().timestamp().to_string();
        option.insert("registered".to_string(), value);
    }

    // 取得したデータをデータベースに保存するためのSQL
    let sql_key = option.keys().map(|s| &**s).collect::<Vec<_>>().join(",");
    let sql_value = option.keys().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql_insert = &format!(
        "INSERT INTO bot ({}, token) VALUES ({}, hex(randomblob(16)))",
        sql_key, sql_value
    );

    // 保存先のDBを読み込む
    let raw_db_path = _get_db_path();
    if let Err(err) = raw_db_path {
        return Err(format!("{}", err));
    }

    // DBに接続する
    let conn = rusqlite::Connection::open(raw_db_path.unwrap().trim());
    if let Err(err) = conn {
        return Err(format!("{}", err));
    }

    // SQLを実行する
    let result = conn.unwrap().execute(
        sql_insert,
        option.values().map(|s| s.to_string()).collect::<Vec<_>>(),
    );

    if let Err(err) = result {
        return Err(format!("{}", err));
    }

    Ok(0)
}

// Updateコマンドを実行する
fn _update(option: &std::collections::HashMap<String, String>) -> Result<usize, String> {
    let mut option = option.clone();

    // idを取得する
    let id_value = option.remove("id");

    // enableが指定されていれば、値を1/0に変換する
    if let Some(v) = option.get("enable") {
        let value = if v == "true" { "1" } else { "0" }.to_string();
        option.insert("enable".to_string(), value);
    }

    // long_orderが指定されていれば、値を1/0に変換する
    if let Some(v) = option.get("long_order") {
        let value = if v == "true" { "1" } else { "0" }.to_string();
        option.insert("long_order".to_string(), value);
    }

    // short_orderが指定されていれば、値を1/0に変換する
    if let Some(v) = option.get("short_order") {
        let value = if v == "true" { "1" } else { "0" }.to_string();
        option.insert("short_order".to_string(), value);
    }

    // 更新用データをデータベースに上書きするためのSQL
    let sql_set = option
        .keys()
        .enumerate()
        .map(|key| format!("{} = ?{}", key.1, key.0 + 2))
        .collect::<Vec<_>>()
        .join(", ");
    let mut sql_value: Vec<String> = Vec::new();
    sql_value.push(id_value.unwrap());
    for value in option.values() {
        sql_value.push(value.to_string());
    }
    let sql_insert = &format!("UPDATE bot SET {} where id = ?1", sql_set);

    // 保存先のDBを読み込む
    let raw_db_path = _get_db_path();
    if let Err(err) = raw_db_path {
        return Err(format!("{}", err));
    }

    // DBに接続する
    let conn = rusqlite::Connection::open(raw_db_path.unwrap().trim());
    if let Err(err) = conn {
        return Err(format!("{}", err));
    }

    // SQLを実行する
    let result = conn.unwrap().execute(sql_insert, sql_value);

    if let Err(err) = result {
        return Err(format!("{}", err));
    }

    Ok(0)
}

// Removeコマンドを実行する
fn _remove(option: std::collections::HashMap<String, String>) -> Result<usize, String> {
    // idを取得する
    let id_value = option.get("id");

    // 保存先のDBを読み込む
    let raw_db_path = _get_db_path();
    if let Err(err) = raw_db_path {
        return Err(format!("{}", err));
    }

    // DBに接続する
    let conn = rusqlite::Connection::open(raw_db_path.unwrap().trim());
    if let Err(err) = conn {
        return Err(format!("{}", err));
    }

    // 指定したIDのbot情報を削除する
    let result = conn.unwrap().execute(
        "delete from bot where id = ?1",
        rusqlite::params![&id_value.unwrap()],
    );

    if let Err(err) = result {
        return Err(format!("{}", err));
    }

    Ok(0)
}

// Getコマンドを実行する
fn _get(option: std::collections::HashMap<String, String>) -> Result<usize, String> {
    // idを取得する
    let id_value = option.get("id");

    // 保存先のDBを読み込む
    let raw_db_path = _get_db_path();
    if let Err(err) = raw_db_path {
        return Err(format!("{}", err));
    }

    // DBに接続する
    let conn = rusqlite::Connection::open(raw_db_path.unwrap().trim());
    if let Err(err) = conn {
        return Err(format!("{}", err));
    }

    // 指定したIDのbot情報を取得する
    let result:Result<Bot, rusqlite::Error> = conn.unwrap().query_row(
        "select id, name, description, enable, registered, token, long_order, short_order, operate_type from bot where id = ?1",
        rusqlite::params![&id_value.unwrap()],
        |row| Ok(Bot {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            description: row.get(2).unwrap(),
            enable: row.get(3).unwrap(),
            registered: row.get(4).unwrap(),
            token: row.get(5).unwrap(),
            long_order: row.get(6).unwrap(),
            short_order: row.get(7).unwrap(),
            operate_type: row.get(8).unwrap(),
        })
    );

    if let Err(err) = result {
        return Err(format!("{}", err));
    }

    // jsonが指定されていればjson形式で返す
    if option.get("json").is_some() {
        let bot: Bot = result.unwrap();
        println!("{}", serde_json::to_string(&bot).unwrap());
        return Ok(0);
    }

    // yamlが指定されていればyaml形式で返す
    if option.get("yaml").is_some() {
        let bot: Bot = result.unwrap();
        println!("{}", serde_yaml::to_string(&bot).unwrap());
        return Ok(0);
    }

    Ok(0)
}

// Listコマンドを実行する
fn _list(option: std::collections::HashMap<String, String>) -> Result<usize, String> {
    // 保存先のDBを読み込む
    let raw_db_path = _get_db_path();
    if let Err(err) = raw_db_path {
        return Err(format!("{}", err));
    }

    // DBに接続する
    let conn = rusqlite::Connection::open(raw_db_path.unwrap().trim()).unwrap();

    // bot情報一覧を取得する
    let mut stmt = conn.prepare("SELECT * FROM bot").unwrap();
    let result = stmt.query_map(rusqlite::params![], |row| {
        Ok(Bot {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            description: row.get(2).unwrap(),
            enable: row.get(3).unwrap(),
            registered: row.get(4).unwrap(),
            token: row.get(5).unwrap(),
            long_order: row.get(6).unwrap(),
            short_order: row.get(7).unwrap(),
            operate_type: row.get(8).unwrap(),
        })
    });

    if let Err(err) = result {
        return Err(format!("{}", err));
    }

    let bot_list = BotList {
        bot: result
            .unwrap()
            .map(|bot| bot.unwrap())
            .collect::<Vec<Bot>>(),
    };

    // jsonが指定されていればjson形式で返す
    if option.get("json").is_some() {
        println!("{}", serde_json::to_string(&bot_list).unwrap());
        return Ok(0);
    }

    // yamlが指定されていればyaml形式で返す
    if option.get("yaml").is_some() {
        println!("{}", serde_yaml::to_string(&bot_list).unwrap());
        return Ok(0);
    }

    Ok(0)
}
