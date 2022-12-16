use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Note<const N: usize> {
    note: usize,
}

impl<const N: usize> Note<N> {
    pub fn new(note: usize) -> Self {
        assert!(note > 0 && note <= N);
        Self { note }
    }

    /// # Safety
    ///
    /// note의 값은 note > 0 && note <= N을 충족해야 함.
    pub unsafe fn new_unchecked(note: usize) -> Self {
        Self { note }
    }

    pub fn get_note(&self) -> usize {
        self.note
    }

    pub fn get_zero_offset(&self) -> usize {
        self.note - 1
    }

    pub fn note_iter() -> NoteIter<N> {
        NoteIter { cur: 0 }
    }
}

impl<const N: usize> Clone for Note<N> {
    fn clone(&self) -> Self {
        Self { note: self.note }
    }
}

impl<const N: usize> Copy for Note<N> {}

impl<const N: usize> PartialEq for Note<N> {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note
    }
}

impl<const N: usize> Eq for Note<N> {}

impl<const N: usize> Display for Note<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.note)
    }
}

pub struct NoteIter<const N: usize> {
    cur: usize,
}

impl<const N: usize> Iterator for NoteIter<N> {
    type Item = Note<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;
        if self.cur > N {
            return None;
        }

        unsafe { Some(Note::new_unchecked(self.cur)) }
    }
}
