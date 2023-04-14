use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("shaders").join("shader.wgsl");

    let parser = ocl_include::Parser::builder()
        .add_source(
            ocl_include::source::Fs::builder()
                .include_dir(uni_path::Path::new("shaders"))?
                .build(),
        )
        .build();

    let path = path.canonicalize()?;
    let path_str = dbg!(path.to_string_lossy());
    let unicode_path = dbg!(uni_path::Path::new(&path_str));
    let parsed_res = parser.parse(unicode_path)?;
    let source = parsed_res.collect().0;

    dbg!(path);
    dbg!(source);

    Ok(())
}
