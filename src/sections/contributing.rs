use crate::sections::section_structs::Section;

pub const CONTRIBUTING: Section = Section {
    flag: "contributing-exclude",
    append_message: "[Info] Contributing section appended",
    exclude_message: "[Info] Contributing section excluded",
    content: "## Contributing\n\
              \n\
              Contributions are welcome and appreciated.\n\
              \n\
              The contributing rules are in `CONTRIBUTING/CONTRIBUTING.md`.",
};
