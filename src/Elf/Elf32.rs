use std::fmt;
use std::mem;

mod Elf {
    #[allow(non_camel_case_types)]
    type Elf32_Addr  = u32;

    #[allow(non_camel_case_types)]
    type Elf32_Half  = u16;

    #[allow(non_camel_case_types)]
    type Elf32_Off   = u32;

    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    type Elf32_Sword = i32;

    #[allow(non_camel_case_types)]
    type Elf32_Word  = u32;

    #[allow(non_camel_case_types)]
    type Elf32_char  = u8;

    const EI_MAG: usize = 4;
    const EI_PAD: usize = 8;
    const EI_PAD_VAL: [Elf32_char; EI_PAD] = [0; EI_PAD];
    const EI_NIDENT: usize = 16;
    const EI_MAG_VAL: [Elf32_char; EI_MAG] = [0x7f, b'E', b'L', b'F'];

    const SHN_UNDEF: Elf32_Half = 0;

    #[allow(non_camel_case_types)]
    enum Elf32_Class {
        ELFCLASSNONE,
        ELFCLASS32,
        ELFCLASS64,
    }

    #[allow(non_camel_case_types)]
    enum Elf32_Data {
        ELFDATANONE,
        ELFDATA2LSB,
        ELFDATA2MSB,
    }

    #[allow(non_camel_case_types)]
    enum Elf32_Type {
        ET_NONE,
        ET_REL,
        ET_EXEC,
        ET_DYN,
        ET_CORE,
        ET_LOPROC = 0xff00,
        ET_HIPROC = 0xffff,
    }

    #[allow(non_camel_case_types)]
    enum Elf32_Arch {
        ET_NONE,
        EM_M32,
        EM_SPARC,
        EM_386,
        EM_68K,
        EM_88K,
        EM_860,
        EM_MIPS,
        EM_MIPS_RS4_BE,
    }

    #[allow(non_camel_case_types)]
    enum Elf32_Version {
        EV_NONE,
        EV_CURRENT,
    }

    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct elf32_ehdr {
        ei_magic:   [Elf32_char; EI_MAG],
        ei_class:    Elf32_char,
        ei_endian:   Elf32_char,
        ei_version:  Elf32_char,
        ei_pad:     [Elf32_char; EI_PAD],
        ei_size:     Elf32_char,
        e_type:      Elf32_Half,
        e_machine:   Elf32_Half,
        e_version:   Elf32_Word,
        e_entry:     Elf32_Addr,
        e_phoff:     Elf32_Off,
        e_shoff:     Elf32_Off,
        e_flags:     Elf32_Word,
        e_ehsize:    Elf32_Half,
        e_phentsize: Elf32_Half,
        e_phnum:     Elf32_Half,
        e_shentsize: Elf32_Half,
        e_shnum:     Elf32_Half,
        e_shstrndx:  Elf32_Half,
    }

    impl elf32_ehdr {
        pub fn new() -> Self {
            Self {
                ei_magic:    EI_MAG_VAL,
                ei_class:    Elf32_Class::ELFCLASS32 as Elf32_char,
                ei_endian:   Elf32_Data::ELFDATANONE as Elf32_char,
                ei_version:  Elf32_Version::EV_CURRENT as Elf32_char,
                ei_pad:      EI_PAD_VAL,
                ei_size:     EI_NIDENT as Elf32_char,
                e_type:      0,
                e_machine:   0,
                e_version:   1,
                e_entry:     0,
                e_phoff:     0,
                e_shoff:     0,
                e_flags:     0,
                e_ehsize:    mem::size_of::<elf32_ehdr>() as u16,
                e_phentsize: 0,
                e_phnum:     0,
                e_shentsize: 0,
                e_shnum:     0,
                e_shstrndx:  SHN_UNDEF,
            }
        }
    }

    impl fmt::Display for elf32_ehdr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Class: {}", match self.ei_class {
                x if x == Elf32_Class::ELFCLASSNONE as Elf32_char => "None",
                x if x == Elf32_Class::ELFCLASS32 as Elf32_char => "ELF32",
                x if x == Elf32_Class::ELFCLASS64 as Elf32_char => "ELF64",
                _ => "Unknown",
            }).unwrap();
            writeln!(f, "Endian: {}", match self.ei_endian {
                x if x == Elf32_Data::ELFDATANONE as Elf32_char => "None",
                x if x == Elf32_Data::ELFDATA2LSB as Elf32_char => "Little Endian",
                x if x == Elf32_Data::ELFDATA2MSB as Elf32_char => "Big Endian",
                _ => "Unknown",
            }).unwrap();
            writeln!(f, "Type: {}", match self.e_type {
                x if x == Elf32_Type::ET_NONE as Elf32_Half => "No file type",
                x if x == Elf32_Type::ET_REL as Elf32_Half  => "Relocatable file",
                x if x == Elf32_Type::ET_EXEC as Elf32_Half => "Executable file",
                x if x == Elf32_Type::ET_DYN as Elf32_Half => "Shared object file",
                x if x == Elf32_Type::ET_CORE as Elf32_Half => "Core file",
                x if x == Elf32_Type::ET_LOPROC as Elf32_Half => "LOPROC Processor-specific",
                x if x == Elf32_Type::ET_HIPROC as Elf32_Half => "HIPROC Processor-specific",
                _ => "Unknown",
            }).unwrap();
            writeln!(f, "Machine: {}", match self.e_machine {
                x if x == Elf32_Arch::ET_NONE as Elf32_Half => "No machine",
                x if x == Elf32_Arch::EM_M32 as Elf32_Half  => "AT&T WE 32100",
                x if x == Elf32_Arch::EM_SPARC as Elf32_Half => "SPARC",
                x if x == Elf32_Arch::EM_386 as Elf32_Half => "Intel Architecture",
                x if x == Elf32_Arch::EM_68K as Elf32_Half => "Motorola 68000",
                x if x == Elf32_Arch::EM_88K as Elf32_Half => "Motorola 88000",
                x if x == Elf32_Arch::EM_860 as Elf32_Half => "Intel 80860",
                x if x == Elf32_Arch::EM_MIPS as Elf32_Half => "MIPS RS3000 Big-Endian",
                x if x == Elf32_Arch::EM_MIPS_RS4_BE as Elf32_Half => "MIPS RS4000 Big-Endian",
                11..=16 => "Reserved for future use",
                _ => "Unknown",
            }).unwrap();
            writeln!(f, "Version: {}", match self.e_version {
                x if x == Elf32_Version::EV_NONE as Elf32_Word => "Invalid version",
                x if x == Elf32_Version::EV_CURRENT as Elf32_Word => "Current version",
                _ => "Unknown",
            }).unwrap();
            writeln!(f, "Entry:     {}", self.e_entry).unwrap();
            writeln!(f, "Program Header Offset: {}", self.e_phoff).unwrap();
            writeln!(f, "Section Header Offset: {}", self.e_shoff).unwrap();
            writeln!(f, "Flags: {}", self.e_flags).unwrap();
            writeln!(f, "ELF header size: {}", self.e_ehsize).unwrap();
            writeln!(f, "Program Header Size: {}", self.e_phentsize).unwrap();
            writeln!(f, "Program Header Count: {}", self.e_phnum).unwrap();
            writeln!(f, "Section Header Size: {}", self.e_shentsize).unwrap();
            writeln!(f, "Section Header Count: {}", self.e_shnum).unwrap();
            writeln!(f, "Section Header String Table Index: {}", self.e_shstrndx).unwrap();
            writeln!(f, "}}")
        }
    }
}