//! # [`Merge`] Runnable
//!
//! This module provides basic data types to convert the [`crate::cli::subcommands::MergeArgs`]
//! into a runnable executor.
mod depth;
mod errors;
mod run_success;

#[cfg(test)]
mod tests;

use crate::{
    cli::subcommands::MergeArgs,
    utils::{
        check::CheckableItem,
        path::normalize_path_buf,
        print::{PrintableTag, Printer},
        run::RunnableItem,
    },
};
use depth::Depth;
pub use errors::*;
use lopdf::{self, Bookmark, Document, Object, ObjectId};
pub use run_success::RunSuccess;
use std::{
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

/// Merge action executor. It stores data to be converted in input/output file paths and action
/// arguments.
#[derive(Debug, PartialEq)]
pub struct Merge {
    /// Path to all inputs to merge. Can be a single directory path or +2 pdf file paths.
    input: Vec<PathBuf>,
    /// Where to place the output merge.
    output: PathBuf,
    /// Override output file if it already exists.
    ovrrd: bool,
    /// Allow input repetition.
    repetition: bool,
    /// Catch files until the until the defined depth.
    depth: Depth,
    /// Create parent dirs of the output if not exists.
    parent: bool,
}

impl TryFrom<MergeArgs> for Merge {
    type Error = MergeBuildError;
    fn try_from(value: MergeArgs) -> Result<Self, Self::Error> {
        let MergeArgs {
            input,
            output,
            override_output,
            allow_repetition,
            depth,
            parent,
        } = value;
        let input = match input {
            x if x.is_empty() => Err(MergeBuildError::InputIsEmpty),
            x => Ok(x
                .into_iter()
                .map(|path| normalize_path_buf(Path::new(&path)))
                .collect()),
        }?;
        let output = match output {
            Some(o) => Ok(PathBuf::from(o)),
            None => Err(MergeBuildError::OutputIsEmpty),
        }?;
        let ovrrd = override_output;
        let repetition = allow_repetition;
        let depth = depth.map_or(Ok(Depth::default()), |d| {
            Depth::try_from(d.trim().to_string())
        })?;
        Ok(Merge {
            input,
            output,
            ovrrd,
            repetition,
            depth,
            parent,
        })
    }
}

impl CheckableItem for Merge {
    type CheckableOutput = Result<(), MergeCheckError>;
    fn check_item(&self) -> Self::CheckableOutput {
        // check input paths
        let mut inp_count = 0;
        let mut contains_dir = false;
        for path in &self.input {
            if path.iter().any(|sd| sd == OsStr::new("..")) {
                return Err(MergeCheckError::InputIsDirectoryReference(path.into()));
            }
            if path.is_file()
                && path
                    .try_exists()
                    .map_err(|_| MergeCheckError::InputIsNotPdfFile(path.into()))?
                && path.extension().is_some_and(|ext| ext != "pdf")
            {
                return Err(MergeCheckError::InputIsNotPdfFile(path.into()));
            }
            match (path, &self.depth) {
                (p, _) if p.iter().any(|sd| sd == OsStr::new("..")) => {
                    Err(MergeCheckError::InputIsDirectoryReference(path.into()))
                }
                (p, _) if p.is_file() && p.extension().is_some_and(|ext| ext != "pdf") => {
                    Err(MergeCheckError::InputIsNotPdfFile(path.into()))
                }
                (p, Depth::NotSpecified) if p.is_dir() => Err(MergeCheckError::DepthNotSpecified),
                _ => Ok(()),
            }?;
            contains_dir |= path.is_dir();
            inp_count += 1;
        }
        if inp_count == 1 && !contains_dir {
            return Err(MergeCheckError::InputIsSingleFile((&self.input[0]).into()));
        }
        // check output paths
        let out = self.output.clone();
        if out.is_dir() {
            return Err(MergeCheckError::OutputIsDirectory(out));
        } else if out.extension().is_some_and(|ext| ext != "pdf") {
            return Err(MergeCheckError::OutputIsNotPdfFile(out));
        } else if out.iter().any(|sd| sd == OsStr::new("..")) {
            return Err(MergeCheckError::OutputIsDirectoryReference(out));
        }
        // check repetitions
        if !self.repetition {
            let mut remains = self.input.clone();
            let mut pop = remains.pop();
            while let Some(p) = pop {
                if remains.contains(&p) {
                    return Err(MergeCheckError::InputRepetitionWithoutFlag(p));
                }
                pop = remains.pop();
            }
        }
        // check parent flag
        let mut buf = PathBuf::new();
        for dir in self.output.iter().take(self.output.iter().count() - 1) {
            buf.push(dir);
            if !buf.exists() && !self.parent {
                return Err(MergeCheckError::ParentOutputWithoutFlag(out));
            }
        }
        // finally: return dir not exists or if override is allowed
        self.output
            .try_exists()
            .map_err(|_| MergeCheckError::CouldNotReadOrCheckFilePath(out.clone()))
            .and_then(|res| {
                if !res || self.ovrrd {
                    Ok(())
                } else {
                    Err(MergeCheckError::OutputAlreadyExists(out))
                }
            })
    }
}

/// Get PDF paths recursively + handle depth flag.
fn get_pdf_paths(
    paths: Vec<PathBuf>,
    cur_depth: usize,
    max_depth: &Depth,
) -> Result<Vec<PathBuf>, MergeRunError> {
    if let Depth::Max(deep) = max_depth {
        if cur_depth > *deep {
            return Ok(Vec::new());
        }
    }
    let mut result = Vec::new();

    paths.into_iter().try_for_each(|entry| {
        if !entry
            .try_exists()
            .map_err(|_| MergeRunError::CouldNotReadEntry(entry.clone()))?
        {
            return Err(MergeRunError::EntryDoesNotExists(entry));
        }
        if entry.is_file() && entry.extension().is_some_and(|ext| ext == "pdf") {
            result.push(entry.as_path().to_path_buf());
        } else if entry.is_dir() {
            let mut recursive = Vec::new();
            for path in entry
                .clone()
                .read_dir()
                .map_err(|_| MergeRunError::CouldNotReadEntry(entry.clone()))?
            {
                match path {
                    Ok(p) => recursive.push(p.path().to_path_buf()),
                    Err(_) => return Err(MergeRunError::CouldNotReadEntry(entry)),
                }
            }
            let recursive = get_pdf_paths(recursive, cur_depth + 1, max_depth)?;
            recursive.into_iter().for_each(|p| result.push(p));
        }
        Ok(())
    })?;
    Ok(result)
}

/// Generates a new [`lopdf::Document`] based on paths at `paths` slice.
///
/// Will return the [`Ok`] variant if the doc generate successes, or the suitable [`MergeRunError`]
/// for the occasion.
///
/// This code was copy + paste from the official documentation
/// (https://docs.rs/lopdf/0.38.0/lopdf/index.html).
fn gen_pdf(paths: &[PathBuf]) -> Result<Document, MergeRunError> {
    // Define a starting `max_id` (will be used as start index for object_ids).
    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");
    let mut doc: Document;
    let mut first: bool;

    for p in paths {
        first = false;
        doc = Document::load(p).map_err(|_| MergeRunError::CouldNotLoadInput(p.into()))?;
        doc.renumber_objects_with(max_id);
        max_id = doc.max_id + 1;
        documents_pages.extend(
            doc.get_pages()
                .into_values()
                .map(|object_id| {
                    if !first {
                        let bookmark = Bookmark::new(
                            format!("Page_{}", pagenum),
                            [0.0, 0.0, 0.1],
                            0,
                            object_id,
                        );
                        document.add_bookmark(bookmark, None);
                        first = true;
                        pagenum += 1;
                    }
                    (object_id, doc.get_object(object_id).unwrap().to_owned())
                })
                .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // "Catalog" and "Pages" are mandatory.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
        // All other objects should be collected and inserted into the main Document.
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages".
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found, abort.
    if pages_object.is_none() {
        return Err(MergeRunError::RootPageNotFound);
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                .objects
                .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found, abort.
    let catalog_object = catalog_object.ok_or(MergeRunError::CatalogIsNone)?;
    let pages_object = pages_object.ok_or(MergeRunError::RootPageNotFound)?;

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                .into_keys()
                .map(Object::Reference)
                .collect::<Vec<_>>(),
        );

        document
            .objects
            .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
            .objects
            .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    // Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_object.0) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    Ok(document)
}

impl RunnableItem for Merge {
    type ArgType = ();
    type Output = Result<RunSuccess, MergeRunError>;
    fn run_item(self) -> Result<RunSuccess, MergeRunError> {
        let now = Instant::now();
        let input_queue = get_pdf_paths(self.input, 0, &self.depth)?;
        let mut rep_set_aux = HashSet::new();
        input_queue.iter().try_for_each(|x| {
            if !rep_set_aux.insert(x) && !self.repetition {
                Err(MergeRunError::PathRepetitionWithoutFlag(x.into()))
            } else {
                Ok(())
            }
        })?;
        let mut doc_file = gen_pdf(&input_queue)?;
        if self.output.exists() {
            Printer::title(
                PrintableTag::Warning,
                Some("overriding an older file with this one"),
            );
        }

        if let Some(parent) = self.output.parent() {
            if !self.output.exists() {
                fs::create_dir_all(parent)
                    .map_err(|_| MergeRunError::CouldNotSaveTheOutput(self.output.clone()))?;
            }
        }
        doc_file
            .save(&self.output)
            .map(|_| {
                RunSuccess::new(
                    input_queue,
                    now.elapsed().as_secs_f64(),
                    self.output.clone(),
                )
            })
            .map_err(|_| MergeRunError::CouldNotSaveTheOutput(self.output))
    }

    #[allow(unused_variables)]
    fn run_with_arg(self, arg: ()) -> Result<RunSuccess, MergeRunError> {
        panic!("This item shouldn't be ran with args. aborting.");
    }
}
