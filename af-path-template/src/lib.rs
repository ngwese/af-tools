use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use median::{
    attr::{AttrBuilder, AttrType},
    builder::MaxWrappedBuilder,
    class::Class,
    method::Method,
    outlet::OutList,
    post,
    symbol::SymbolRef,
    wrapper::{attr_get_tramp, attr_set_tramp, tramp,MaxObjWrapped, MaxObjWrapper},
};

mod template;

median::external! {
    #[name="af.path.template"]
    pub struct PathTemplateExtn {
        // attributes
        parent_dir: SymbolRef,
        sub_dir: SymbolRef,
        file_template: SymbolRef,
        file_stem: SymbolRef,
        file_prefix: SymbolRef,
        file_suffix: SymbolRef,
        file_extension: SymbolRef,
        // serial_regex: SymbolRef,

        // outputs
        main_out: OutList,
    }

    impl MaxObjWrapped<PathTemplateExtn> for PathTemplateExtn {
        fn new(builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
            Self {
                file_template: SymbolRef::try_from(template::DEFAULT_FILE_TEMPLATE).unwrap(),

                parent_dir: SymbolRef::default(),
                sub_dir: SymbolRef::default(),
                file_stem: SymbolRef::default(),
                file_prefix: SymbolRef::default(),
                file_suffix: SymbolRef::default(),
                file_extension: SymbolRef::default(),

                main_out: builder.add_list_outlet(),
            }
        }

        // Register method and attributes
        fn class_setup(c: &mut Class<MaxObjWrapper<Self>>) {
            c.add_method(Method::SelS("path", Self::path_tramp, 0)).unwrap();

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "parent_dir",
                    AttrType::SymbolRef,
                    Self::parent_dir_tramp,
                    Self::set_parent_dir_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add parent_dir attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "sub_dir",
                    AttrType::SymbolRef,
                    Self::sub_dir_tramp,
                    Self::set_sub_dir_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add sub_dir attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "template",
                    AttrType::SymbolRef,
                    Self::template_tramp,
                    Self::set_template_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add template attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "prefix",
                    AttrType::SymbolRef,
                    Self::prefix_tramp,
                    Self::set_prefix_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add prefix attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "stem",
                    AttrType::SymbolRef,
                    Self::stem_tramp,
                    Self::set_stem_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add stem attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "suffix",
                    AttrType::SymbolRef,
                    Self::suffix_tramp,
                    Self::set_suffix_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add suffix attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "extension",
                    AttrType::SymbolRef,
                    Self::extension_tramp,
                    Self::set_extension_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add extension attribute");
        }
    }

    impl PathTemplateExtn {
        fn maybe_string(&self, s: &SymbolRef) -> Option<String> {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string().unwrap())
            }
        }

        fn maybe_pathbuf(&self, s: &SymbolRef) -> Option<PathBuf> {
            if s.is_empty() {
                None
            } else {
                Some(PathBuf::from(s.to_string().unwrap()))
            }
        }

        #[tramp]
        pub fn path(&self, input: SymbolRef) {
            // post!("path: {}", input);

            let mut template = template::PathTemplate::new(&self.file_template.to_string().unwrap());
            template.parent_dir = self.maybe_pathbuf(&self.parent_dir);
            template.sub_dir = self.maybe_pathbuf(&self.sub_dir);
            template.prefix = self.maybe_string(&self.file_prefix);
            template.stem = self.maybe_string(&self.file_stem);
            template.suffix = self.maybe_string(&self.file_suffix);
            template.extension = self.maybe_string(&self.file_extension);

            let binding = input.to_string().unwrap();
            let input = Path::new(binding.as_str());
            match template.apply(input) {
                Ok(path) => {
                    let path_str = path.to_string_lossy().to_string();
                    // post!("resulting path: {}", path_str);
                    let _ = self.main_out.send(&[
                        SymbolRef::try_from("path").unwrap().into(),
                        SymbolRef::try_from(path_str.as_str()).unwrap().into(),
                    ]);
                },
                Err(e) => {
                    post!("error: {}", e);
                }
            }
        }

        #[attr_get_tramp]
        pub fn parent_dir(&self) -> SymbolRef {
            self.parent_dir.clone()
        }

        #[attr_set_tramp]
        pub fn set_parent_dir(&self, v: SymbolRef) {
            self.parent_dir.assign(&v);
            // post!("set_parent_dir: {}", v);
        }

        #[attr_get_tramp]
        pub fn sub_dir(&self) -> SymbolRef {
            self.sub_dir.clone()
        }

        #[attr_set_tramp]
        pub fn set_sub_dir(&self, v: SymbolRef) {
            self.sub_dir.assign(&v);
            // post!("set_sub_dir: {}", v);
        }

        #[attr_get_tramp]
        pub fn template(&self) -> SymbolRef {
            self.file_template.clone()
        }

        #[attr_set_tramp]
        pub fn set_template(&self, v: SymbolRef) {
            self.file_template.assign(&v);
            // post!("set_template: {}", v);
        }

        #[attr_get_tramp]
        pub fn extension(&self) -> SymbolRef {
            self.file_extension.clone()
        }

        #[attr_set_tramp]
        pub fn set_extension(&self, v: SymbolRef) {
            self.file_extension.assign(&v);
            // post!("set_extension: {}", v);
        }

        #[attr_get_tramp]
        pub fn suffix(&self) -> SymbolRef {
            self.file_suffix.clone()
        }

        #[attr_set_tramp]
        pub fn set_suffix(&self, v: SymbolRef) {
            self.file_suffix.assign(&v);
            // post!("set_suffix: {}", v);
        }

        #[attr_get_tramp]
        pub fn prefix(&self) -> SymbolRef {
            self.file_prefix.clone()
        }

        #[attr_set_tramp]
        pub fn set_prefix(&self, v: SymbolRef) {
            self.file_prefix.assign(&v);
            // post!("set_prefix: {}", v);
        }

        #[attr_get_tramp]
        pub fn stem(&self) -> SymbolRef {
            self.file_stem.clone()
        }

        #[attr_set_tramp]
        pub fn set_stem(&self, v: SymbolRef) {
            self.file_stem.assign(&v);
            // post!("set_stem: {}", v);
        }

    }
}