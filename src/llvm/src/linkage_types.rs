use std::fmt;
use std::fmt::Formatter;
use LinkageTypes::*;

pub enum LinkageTypes {
    /// Global values with “private” linkage are only directly accessible by objects in the current module.
    /// In particular, linking code into a module with a private global value may cause the private to be renamed as necessary to avoid collisions.
    /// Because the symbol is private to the module, all references can be updated. This doesn’t show up in any symbol table in the object file.
    Private,
    /// Similar to private, but the value shows as a local symbol (STB_LOCAL in the case of ELF) in the object file.
    /// This corresponds to the notion of the ‘static’ keyword in C.
    Internal,
    /// Globals with “available_externally” linkage are never emitted into the object file corresponding to the LLVM module.
    ///  From the linker’s perspective, an available_externally global is equivalent to an external declaration.
    /// They exist to allow inlining and other optimizations to take place given knowledge of the definition of the global, which is known to be somewhere outside the module.
    /// Globals with available_externally linkage are allowed to be discarded at will, and allow inlining and other optimizations.
    /// This linkage type is only allowed on definitions, not declarations.
    AvailableExternally,
    /// Globals with “linkonce” linkage are merged with other globals of the same name when linkage occurs.
    ///  This can be used to implement some forms of inline functions, templates, or other code which must be generated in each translation unit that uses it, but where the body may be overridden with a more definitive definition later.
    /// Unreferenced linkonce globals are allowed to be discarded.
    /// Note that linkonce linkage does not actually allow the optimizer to inline the body of this function into callers because it doesn’t know if this definition of the function is the definitive definition within the program or whether it will be overridden by a stronger definition.
    /// To enable inlining and other optimizations, use “linkonce_odr” linkage.
    Linkonce,
    /// “weak” linkage has the same merging semantics as linkonce linkage, except that unreferenced globals with weak linkage may not be discarded.
    /// This is used for globals that are declared “weak” in C source code.
    Weak,
    /// “common” linkage is most similar to “weak” linkage, but they are used for tentative definitions in C, such as “int X;” at global scope.
    /// Symbols with “common” linkage are merged in the same way as weak symbols, and they may not be deleted if unreferenced.
    /// common symbols may not have an explicit section, must have a zero initializer, and may not be marked ‘constant’.
    /// Functions and aliases may not have common linkage.
    Common,
    /// “appending” linkage may only be applied to global variables of pointer to array type.
    /// When two global variables with appending linkage are linked together, the two global arrays are appended together.
    /// This is the LLVM, typesafe, equivalent of having the system linker append together “sections” with identical names when .o files are linked.
    /// Unfortunately this doesn’t correspond to any feature in .o files, so it can only be used for variables like llvm.global_ctors which llvm interprets specially.
    Appending,
    /// The semantics of this linkage follow the ELF object file model: the symbol is weak until linked, if not linked, the symbol becomes null instead of being an undefined reference.
    ExternWeak,
    /// Some languages allow differing globals to be merged, such as two functions with different semantics.
    /// Other languages, such as C++, ensure that only equivalent globals are ever merged (the “one definition rule” — “ODR”).
    /// Such languages can use the linkonce_odr and weak_odr linkage types to indicate that the global will only be merged with equivalent globals.
    /// These linkage types are otherwise the same as their non-odr versions.
    WeakOdr,
    /// Some languages allow differing globals to be merged, such as two functions with different semantics.
    /// Other languages, such as C++, ensure that only equivalent globals are ever merged (the “one definition rule” — “ODR”).
    /// Such languages can use the linkonce_odr and weak_odr linkage types to indicate that the global will only be merged with equivalent globals.
    /// These linkage types are otherwise the same as their non-odr versions.
    LinkonceOdr,
    /// If none of the above identifiers are used, the global is externally visible, meaning that it participates in linkage and can be used to resolve external symbol references.
    External,
    /// No linkage
    NULL,
}

impl fmt::Display for LinkageTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Private => " private",
                Internal => " internal",
                AvailableExternally => " available_externally",
                Linkonce => " linkonce",
                Weak => " weak",
                Common => " common",
                Appending => " appending",
                ExternWeak => " external_weak",
                WeakOdr => " weak_odr",
                LinkonceOdr => " linkonce_odr",
                External => " external",
                NULL => "",
            }
        )
    }
}
