use std::fs;
use xshell::{cmd, Shell};

pub static CHIPS: &[(&str, &str)] = &[
    ("at32f413", "AT32F413xx_v2"),
    ("at32f415", "AT32F415xx_v2"),
    ("at32f421", "AT32F421xx_v2"),
    ("at32f435", "AT32F435xx_v2"),
    ("at32f437", "AT32F437xx_v2"),
];

pub fn generate() {
    let sh = Shell::new().unwrap();

    for (chip, svd) in CHIPS {
        let svd_path = format!("svds/{}.svd", svd);
        let crate_dir = format!("src/{}", chip);

        let _ = fs::remove_dir_all(&crate_dir);
        fs::create_dir_all(&crate_dir).unwrap();

        cmd!(
            sh,
            "svd2rust -m -g -s -i {svd_path} -o {crate_dir} --reexport-core-peripherals --reexport-interrupt --atomics --atomics_feature atomics --impl-debug --impl-defmt defmt"
        )
        .run()
        .unwrap();

        cmd!(sh, "form -i {crate_dir}/mod.rs -o {crate_dir}")
            .run()
            .unwrap();

        cmd!(sh, "rm {crate_dir}/mod.rs").run().unwrap();

        cmd!(sh, "mv {crate_dir}/lib.rs {crate_dir}/mod.rs")
            .run()
            .unwrap();
    }

    cmd!(sh, "cargo fmt").run().unwrap();
}
