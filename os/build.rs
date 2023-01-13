use std::io::Write;
use std::{
    fs::{read_dir, File},
    io,
};

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";
fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

fn insert_app_data() -> io::Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find(".").unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        &mut f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    #quad 把参数当做8字节整数
    .quad {}"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    for (i, app_name) in apps.into_iter().enumerate() {
        writeln!(
            f,
            r#"
        .section .data
        .global app_{i}_start
        .global app_{i}_end
    app_{i}_start:
        # 把对应文件的内容复制到这个位置
        .incbin "{TARGET_PATH}{app_name}.bin"
    app_{i}_end:"#
        )?;
    }
    Ok(())
}
