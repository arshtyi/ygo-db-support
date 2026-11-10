use std::env;

use anyhow::{Context, Result, anyhow, bail};
use typst::{
    Library, LibraryExt, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime, Duration},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_kit::{
    datetime::Time,
    diagnostics::{self, DiagnosticFormat, DiagnosticWorld, termcolor},
    files::{FileLoader, FileStore, FsRoot},
    fonts::{self, FontStore},
};
use typst_layout::PagedDocument;

const INPUT: &str = "ygo-db-support.typ";
const OUTPUT: &str = "ygo-db-support.png";

pub(crate) fn compile() -> Result<()> {
    let mut fonts = FontStore::new();
    fonts.extend(fonts::system());
    fonts.extend(fonts::embedded());
    let world = CompilerWorld {
        library: LazyHash::new(Library::default()),
        main: RootedPath::new(VirtualRoot::Project, VirtualPath::new(INPUT)?).intern(),
        files: FileStore::new(ProjectFiles(FsRoot::new(
            env::current_dir().context("读取当前目录失败")?,
        ))),
        fonts,
        time: Time::system(),
    };
    let result = typst::compile::<PagedDocument>(&world);
    let mut stderr = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    diagnostics::emit(
        &mut stderr,
        &world,
        result.warnings.iter().chain(
            result
                .output
                .as_ref()
                .err()
                .into_iter()
                .flat_map(|errors| errors.iter()),
        ),
        DiagnosticFormat::Human,
    )
    .context("输出 Typst 诊断失败")?;
    let document = result.output.map_err(|_| anyhow!("编译 {INPUT} 失败"))?;
    let [page] = document.pages() else {
        bail!(
            "{OUTPUT} 需要单页文档，实际生成了 {} 页",
            document.pages().len()
        );
    };
    let options = typst_render::RenderOptions {
        // Match `typst compile`'s default PNG resolution of 144 PPI.
        pixel_per_pt: 2.0.into(),
        ..Default::default()
    };
    typst_render::render(page, &options)
        .save_png(OUTPUT)
        .with_context(|| format!("写入 {OUTPUT} 失败"))?;
    println!("{INPUT} -> {OUTPUT}");
    Ok(())
}

struct ProjectFiles(FsRoot);

impl FileLoader for ProjectFiles {
    fn load(&self, id: FileId) -> FileResult<Bytes> {
        if !matches!(id.root(), VirtualRoot::Project) {
            return Err(FileError::Other(Some(
                "此项目仅支持本地 Typst 文件，不加载外部包".into(),
            )));
        }
        self.0.load(id.vpath())
    }
}

struct CompilerWorld {
    library: LazyHash<Library>,
    main: FileId,
    files: FileStore<ProjectFiles>,
    fonts: FontStore,
    time: Time,
}

impl World for CompilerWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        self.fonts.book()
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.files.source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.files.file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        self.time.today(offset)
    }
}

impl DiagnosticWorld for CompilerWorld {
    fn name(&self, id: FileId) -> String {
        id.vpath().get_without_slash().into()
    }
}
