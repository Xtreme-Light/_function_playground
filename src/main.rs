fn main() {
    let module_one = my_req(Options {
        module: "one".to_string(),
    });
    let module_two = my_req(Options {
        module: "two".to_string(),
    });
    let get_list = partial(&module_one, "/api/get");
    let update = partial(&module_two, "/api/update");
    let to_get_list = check_id(check_null(&get_list));
    let to_update = check_id(check_null(&update));

    match to_get_list(Some("aaaa")) {
        Ok(_) => println!("aaaa成功"),
        Err(_) => println!("aaaa失败"),
    }
    match to_update(Some("bbbb")) {
        Ok(_) => println!("bbbb成功"),
        Err(_) => println!("bbbb失败"),
    }
}

#[derive(Clone, Debug)]
pub struct Options {
    module: String,
}
pub fn req(url: &str, params: &str, options: Options) {
    println!(
        "传入 {} {} 选项 {:?} {} ",
        url, params, options, options.module
    );
}

pub fn my_req(options: Options) -> impl Fn(&str, &str) {
    move |url, params| req(url, params, options.clone())
}

pub fn partial<'a>(req_fn: impl Fn(&'a str, &'a str), endpoint: &'a str) -> impl Fn(&'a str) {
    move |params| req_fn(endpoint, params)
}
pub fn check_null<'a, F>(fn_: F) -> impl Fn(&'a str) -> Result<&'a str, &'static str>
where
    F: Fn(&'a str),
{
    move |params| {
        if params.is_empty() {
            Err("参数不可为空")
        } else {
            fn_(params);
            Ok("")
        }
    }
}

fn check_id<'a, F>(fn_: F) -> impl Fn(Option<&'a str>) -> Result<&'a str, &'static str>
where
    F: Fn(&'a str) -> Result<&'a str, &'static str>,
{
    move |params| {
        if let Some(id) = params {
            fn_(id)
        } else {
            Err("ID不可为空")
        }
    }
}
