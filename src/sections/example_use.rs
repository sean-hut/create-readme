use crate::sections::section_structs::Section;

pub const EXAMPLE_USE: Section = Section {
    flag: "exclude-example-use",
    append_message: "[Info] Example use section appended",
    exclude_message: "[Info] Example use section excluded",
    content: "## Example Use\n\
              \n\
              Projects that are known to be using this project are listed in\n\
              `example-use.md`.\n\
              \n",
};
