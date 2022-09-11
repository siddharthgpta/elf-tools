use std::fmt;

mod Elf {
    #[allow(non_camel_case_types)]
    type Elf64_Addr  = u32;

    #[allow(non_camel_case_types)]
    type Elf64_Half  = u16;

    #[allow(non_camel_case_types)]
    type Elf64_Off   = u32;

    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    type Elf64_Sword = i32;

    #[allow(non_camel_case_types)]
    type Elf64_Word  = u32;

    #[allow(non_camel_case_types)]
    type Elf64_char  = u8;

    const EI_MAG: usize = 4;
    const EI_PAD: usize = 8;
    const EI_PAD_VAL: [Elf64_char; EI_PAD] = [0; EI_PAD];
    const EI_NIDENT: usize = 16;
    const EI_MAG_VAL: [Elf64_char; EI_MAG] = [0x7f, b'E', b'L', b'F'];

    const SHN_UNDEF: Elf64_Half = 0;

    #[allow(non_camel_case_types)]
    enum Elf64_Class {
        ELFCLASSNONE,
        ELFCLASS32,
        ELFCLASS64,
    }

    #[allow(non_camel_case_types)]
    enum Elf64_Data {
        ELFDATANONE,
        ELFDATA2LSB,
        ELFDATA2MSB,
    }

    #[allow(non_camel_case_types)]
    enum Elf64_Version {
        EV_NONE,
        EV_CURRENT,
    }

    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    pub struct elf64_ehdr {
        ei_magic:   [Elf64_char; EI_MAG],
        ei_class:    Elf64_char,
        ei_endian:   Elf64_char,
        ei_version:  Elf64_char,
        ei_pad:     [Elf64_char; EI_PAD],
        ei_size:     Elf64_char,
    }

    impl elf64_ehdr {
        pub fn new() -> Self {
            Self {
                ei_magic:    EI_MAG_VAL,
                ei_class:    Elf64_Class::ELFCLASS64 as Elf64_char,
                ei_endian:   Elf64_Data::ELFDATANONE as Elf64_char,
                ei_version:  Elf64_Version::EV_CURRENT as Elf64_char,
                ei_pad:      EI_PAD_VAL,
                ei_size:     EI_NIDENT as Elf64_char,
            }
        }
    }

    impl fmt::Display for elf64_ehdr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Class: {}", match self.ei_class {
                x if x == Elf64_Class::ELFCLASSNONE as Elf64_char => "None",
                x if x == Elf64_Class::ELFCLASS32 as Elf64_char => "ELF64",
                x if x == Elf64_Class::ELFCLASS64 as Elf64_char => "ELF64",
                _ => "Unknown",
            }).unwrap();
            writeln!(f, "Endian: {}", match self.ei_endian {
                x if x == Elf64_Data::ELFDATANONE as Elf64_char => "None",
                x if x == Elf64_Data::ELFDATA2LSB as Elf64_char => "Little Endian",
                x if x == Elf64_Data::ELFDATA2MSB as Elf64_char => "Big Endian",
                _ => "Unknown",
            })
        }
    }
}