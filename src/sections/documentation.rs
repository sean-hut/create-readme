use crate::sections::section_structs::Section;

pub const DOCUMENTATION: Section = Section {
    flag: "documentation-exclude",
    append_message: "[Info] Documentation section appended",
    exclude_message: "[Info] Documentation section excluded",
    content: "## Documentation\n\
              \n\
              The documentation is in `DOCUMENTATION.md`.\n\
              \n",
};
