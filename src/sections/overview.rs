use crate::sections::section_structs::Section;

pub const OVERVIEW: Section = Section {
    flag: "overview-exclude",
    append_message: "[Info] Overview section appended",
    exclude_message: "[Info] Overview section excluded",
    content: "TODO Overview section.\n\
              \n",
};
