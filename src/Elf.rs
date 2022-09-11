use std::fmt;
mod Elf32;
mod Elf64;

#[allow(non_camel_case_types)]
pub enum elf {
    ELF32(Elf::Elf32),
    ELF64(Elf::Elf64),
}

enum e_ident {

}

impl elf_ehdr {
    pub fn open() -> elf_ehdr {

    }
}

impl fmt::Display for elf_ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            elf_ehdr::ELF32(ehdr) => write!(f, "{}", ehdr),
            elf_ehdr::ELF64(ehdr) => write!(f, "{}", ehdr),
        }
    }
}
