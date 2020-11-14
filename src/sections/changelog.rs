use crate::sections::section_structs::Section;

pub const CHANGELOG: Section = Section {
    flag: "exclude-changelog",
    append_message: "[Info] Changelog section appended",
    exclude_message: "[Info] Changelog section excluded",
    content: "## Changelog\n\
              \n\
              The changelog file is `CHANGELOG.md`.\n\
              \n\
              All notable changes to this project are documented in the changelog file.\n\
              \n",
};
