use crate::sections::section_structs::Section;

pub const DEVELOPMENT_VERSION: Section = Section {
    flag: "development-version-exclude",
    append_message: "[Info] Development version section appended",
    exclude_message: "[Info] Development version section excluded",
    content: "## Development Version\n\
              \n\
              The development version is at the head of the `develop` branch.\n\
              \n",
};

pub const STABLE_VERSION: Section = Section {
    flag: "stable-version-exclude",
    append_message: "[Info] Stable version section appended",
    exclude_message: "[Info] Stable version section excluded",
    content: "## Stable Versions\n\
              \n\
              Stable releases are tagged on the `releases` branch.\n\
              \n\
              The [SemVar][semvar] version of semantic versioning is used.\n\
              \n\
              [semvar]: <https://web.archive.org/web/20201009135328/https://semver.org/>\n\
              \n",
};
