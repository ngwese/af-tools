use std::convert::TryFrom;
use std::path::PathBuf;
use std::cell::RefCell;
use std::sync::RwLock;

use median::{
    max_sys,
    attr::{AttrBuilder, AttrType},
    atom::Atom,
    builder::MaxWrappedBuilder,
    class::Class,
    method::Method,
    outlet::{OutBang, OutList},
    post, error,
    symbol::SymbolRef,
    wrapper::{attr_get_tramp, attr_set_tramp, tramp, MaxObjWrapped, MaxObjWrapper},
};
use std::os::raw::c_long;

use ignore::{WalkBuilder, Walk};

const MODE_EAGER: &str = "eager";
const MODE_LAZY: &str = "lazy";

median::external! {
    #[name="af.path.walk"]
    pub struct PathWalkExtn {
        // attributes
        directory: SymbolRef,
        mode: SymbolRef,

        // state
        lazy_walker: RwLock<Option<Walk>>,

        // outputs
        main_out: OutList,
        complete_out: OutBang,
    }

    impl MaxObjWrapped<PathWalkExtn> for PathWalkExtn {
        fn new(builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
            Self {
                directory: SymbolRef::default(),
                mode: SymbolRef::try_from(MODE_LAZY).unwrap(),
                lazy_walker: RwLock::new(None),
                main_out: builder.add_list_outlet(),
                complete_out: builder.add_bang_outlet_with_assist("walk complete"),
            }
        }

        // Register method and attributes
        fn class_setup(c: &mut Class<MaxObjWrapper<Self>>) {
            c.add_method(Method::SelVarArg("walk", Self::walk_tramp)).unwrap();

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "directory",
                    AttrType::SymbolRef,
                    Self::directory_tramp,
                    Self::set_directory_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add directory attribute");

            c.add_attribute(
                AttrBuilder::new_accessors(
                    "mode",
                    AttrType::SymbolRef,
                    Self::mode_tramp,
                    Self::set_mode_tramp,
                )
                .build()
                .unwrap(),
            )
            .expect("failed to add mode attribute");
        }
    }

    impl PathWalkExtn {
        // fn maybe_string(&self, s: &SymbolRef) -> Option<String> {
        //     if s.is_empty() {
        //         None
        //     } else {
        //         Some(s.to_string().unwrap())
        //     }
        // }

        // FIXME: Move this into a common crate for use across all extensions
        fn maybe_pathbuf(&self, s: &SymbolRef) -> Option<PathBuf> {
            if s.is_empty() {
                None
            } else {
                Some(PathBuf::from(s.to_string().unwrap()))
            }
        }

        #[bang]
        pub fn bang(&self) {
            self.lazy_next();
        }

        #[tramp]
        pub fn walk(&self, sel: *mut max_sys::t_symbol, ac: c_long, av: *const max_sys::t_atom) {
            use median::method::sel_list;

            // FIXME: Not sure why this can't be mutated withing the sel_list
            // closure without a RefCell, assuming this is because the closure
            // is `Fn` instead of `FnMut`. Seems like the assumption is that
            // everything is implemented inside the closure.
            let directory_arg = RefCell::new(self.directory.clone());

            // process arguments
            sel_list(sel, ac, av, |_selector, atoms| {
                if !atoms.is_empty() {
                    // Take the first argument as the directory
                    *directory_arg.borrow_mut() = atoms[0].get_symbol();
                }
            });

            let directory = self.maybe_pathbuf(&directory_arg.borrow());

            if let Some(path) = directory {
                post!("start walk of path: {}", path.display());

                let walker = WalkBuilder::new(path.as_path())
                    .standard_filters(true)
                    .build();

                self.lazy_cancel();

                match self.mode.to_string().unwrap().as_str() {
                    MODE_LAZY => {
                        self.lazy_start(walker);
                    }
                    MODE_EAGER => {
                        self.eager_output(walker);
                    }
                    _ => {
                        error!("invalid mode: {}", self.mode.to_string().unwrap());
                    }
                }
            } else {
                error!("no directory specified");
            }
        }

        fn atom_from_entry(&self, entry: ignore::DirEntry) -> Atom {
            let path_str = entry.path().to_str().unwrap();
            let path_sym = SymbolRef::try_from(path_str).unwrap();
            Atom::from(path_sym)
        }

        fn lazy_start(&self, walker: Walk) {
            let mut lazy_walker = self.lazy_walker.write().unwrap();
            *lazy_walker = Some(walker);
            post!("starting lazy walk");
        }

        fn lazy_next(&self) {
            let mut lazy_walker = self.lazy_walker.write().unwrap();

            if let Some(walker) = lazy_walker.as_mut() {
                if let Some(result) = walker.next() {
                    match result {
                        Ok(entry) => {
                            let _ = self.main_out.send(&[self.atom_from_entry(entry)]);
                        },
                        Err(err) => {
                            error!("error walking path: {}", err);
                        }
                    }
                } else {
                    *lazy_walker = None;
                    post!("lazy walk complete");
                    let _ = self.complete_out.send(());
                }
            }
        }

        fn lazy_cancel(&self) {
            let mut lazy_walker = self.lazy_walker.write().unwrap();
            if lazy_walker.is_some() {
                *lazy_walker = None;
                post!("cancelling existing lazy walk");
            }
        }

        fn eager_output(&self, walker: Walk) {
            let mut paths = Vec::new();
            for result in walker {
                match result {
                    Ok(entry) => {
                        paths.push(self.atom_from_entry(entry));
                    },
                    Err(err) => {
                        error!("error walking path: {}", err);
                    }
                }
            }
            post!("collected {} paths", paths.len());
            let _ = self.main_out.send(paths.as_slice());
            let _ = self.complete_out.send(());
        }

        #[attr_get_tramp]
        pub fn directory(&self) -> SymbolRef {
            self.directory.clone()
        }

        #[attr_set_tramp]
        pub fn set_directory(&self, v: SymbolRef) {
            self.directory.assign(&v);
        }

        #[attr_get_tramp]
        pub fn mode(&self) -> SymbolRef {
            self.mode.clone()
        }

        #[attr_set_tramp]
        pub fn set_mode(&self, v: SymbolRef) {
            self.mode.assign(&v);
        }
    }
}