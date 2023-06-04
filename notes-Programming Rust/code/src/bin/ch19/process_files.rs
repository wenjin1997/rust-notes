// //! P462
// use std::{thread, io};
//
// fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
//     // Divide the work into several chunks.
//     const NTHREADS: usize = 8;
//     let worklists = split_vec_into_chunks(filenames, NTHREADS);
//
//     // Fork: Spawn a thread to handle each chunk.
//     let mut thread_handles = vec![];
//     for worklist in worklists {
//         thread_handles.push(
//             thread::spawn(move || process_files_in_parallel(worklist))
//         );
//     }
//
//     // Join: Wait for all threads to finish.
//     for handle in thread_handles {
//         handle.join().unwrap()?;
//     }
//
//     Ok(())
// }
//
// use rayon::prelude::*;
// fn process_files_in_parallel2(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()> {
//     filenames.par_iter()
//         .map(|filename| process_file(filename, glossary))
//         .reduce_with(|r1, r2| {
//             if r1.is_err() { r1 } else { r2 }
//         })
//         .unwrap_or(Ok(()))
// }