use serde::{Deserialize, Serialize};

/// The semantic kind of a symbol defined by a programming language.
///
/// `SymbolKind` provides a language-agnostic classification of symbols that may
/// appear in source code, such as types, functions, methods, fields, logical
/// constructs, and language-specific abstractions.
///
/// The set of symbol kinds and their intended semantics are inspired by the
/// SCIP (SCIP Indexing Protocol) `Kind` enum: <https://github.com/sourcegraph/scip/blob/main/scip.proto#L264>
#[derive(
    Debug,
    sqlx::Type,
    strum_macros::EnumString,
    strum_macros::Display,
    strum_macros::EnumIter,
    Clone,
    Copy,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
#[derive(Default)]
pub enum SymbolKind {
    /// An unknown or unspecified symbol kind.
    #[default]
    Unknown,

    /// A method which may or may not have an implementation body.
    /// Common in Java- and Kotlin-like languages.
    ///
    /// ```java
    /// abstract class Shape {
    ///     abstract double area();
    /// }
    /// ```
    AbstractMethod,

    /// An automatically generated accessor, such as Ruby's `attr_accessor`.
    ///
    /// ```ruby
    /// class User
    ///   attr_accessor :name
    /// end
    /// ```
    Accessor,

    /// An array type or value.
    ///
    /// ```rust,ignore
    /// let xs = [1, 2, 3];
    /// ```
    Array,

    /// A logical assertion, as used in Alloy.
    ///
    /// ```alloy
    /// assert NoCycles {
    ///   all n: Node | n not in n.^next
    /// }
    /// ```
    Assertion,

    /// A type associated with another type, such as in a Rust trait.
    ///
    /// This symbol refers specifically to the associated type `Item`,
    /// not to the enclosing trait `Iterator`.
    ///
    /// ```rust,ignore
    /// trait Iterator {
    ///     type Item;
    ///     // ^^^ associated type
    /// }
    /// ```
    AssociatedType,

    /// A language attribute or annotation, commonly used in C++.
    ///
    /// ```cpp
    /// [[nodiscard]] int compute();
    /// ```
    Attribute,

    /// A foundational logical axiom, as used in Lean.
    ///
    /// ```lean
    /// axiom choice : ∀ α : Type, Nonempty α → α
    /// ```
    Axiom,

    /// A boolean value or type.
    ///
    /// ```rust,ignore
    /// let done: bool = false;
    /// ```
    Boolean,

    /// A class definition.
    ///
    /// ```go
    /// type User struct {
    ///     Name string
    /// }
    /// ```
    Class,

    /// A C++20 concept, defining compile-time constraints.
    ///
    /// ```cpp
    /// template<typename T>
    /// concept Addable = requires(T a, T b) { a + b; };
    /// ```
    Concept,

    /// A constant value.
    ///
    /// ```rust,ignore
    /// const MAX_RETRIES: usize = 5;
    /// ```
    Constant,

    /// A constructor used to create instances of a type.
    ///
    /// ```rust,ignore
    /// impl User {
    ///     fn new(name: String) -> Self {
    ///         Self { name }
    ///     }
    /// }
    /// ```
    Constructor,

    /// A Solidity contract.
    ///
    /// ```solidity
    /// contract Wallet {
    ///     uint balance;
    /// }
    /// ```
    Contract,

    /// A Haskell data family declaration.
    ///
    /// ```haskell
    /// data family Vector a
    /// ```
    DataFamily,

    /// A delegate type, as in C# or F#.
    ///
    /// ```csharp
    /// public delegate void Handler(int code);
    /// ```
    Delegate,

    /// An enumeration type.
    ///
    /// ```rust,ignore
    /// enum Color {
    ///     Red,
    ///     Green,
    /// }
    /// ```
    Enum,

    /// A single member (variant) of an enumeration.
    ///
    /// This symbol refers specifically to the enum member `Red`,
    /// not to the enclosing enum `Color`.
    ///
    /// ```rust,ignore
    /// enum Color {
    ///     Red,
    ///     // ^^^ enum member
    /// }
    /// ```
    EnumMember,

    /// An error type.
    ///
    /// ```rust,ignore
    /// enum ParseError {
    ///     InvalidInput,
    /// }
    /// ```
    Error,

    /// An event symbol.
    ///
    /// ```csharp
    /// public event EventHandler Clicked;
    /// ```
    Event,

    /// An extension declaration, as used in Dart.
    ///
    /// ```dart
    /// extension StringExtras on String {
    ///   int get wordCount => split(' ').length;
    /// }
    /// ```
    Extension,

    /// A logical fact, as used in Alloy.
    ///
    /// ```alloy
    /// fact Acyclic {
    ///   no n: Node | n in n.^next
    /// }
    /// ```
    Fact,

    /// A field declared within a struct or class.
    ///
    /// This symbol refers specifically to the field `name`,
    /// not to the enclosing struct `User`.
    ///
    /// ```rust,ignore
    /// struct User {
    ///     name: String,
    ///     // ^^^ field
    /// }
    /// ```
    Field,

    /// A source file.
    ///
    /// ```text
    /// main.rs
    /// ```
    File,

    /// A free-standing function.
    ///
    /// ```rust,ignore
    /// fn add(a: i32, b: i32) -> i32 {
    ///     a + b
    /// }
    /// ```
    Function,

    /// A getter accessor.
    ///
    /// This symbol refers specifically to the getter for `name`,
    /// not to the enclosing property or struct.
    ///
    /// ```swift
    /// var name: String {
    ///   get { _name }
    ///   // ^^^ getter
    /// }
    /// ```
    Getter,

    /// A grammar definition, as used in Raku.
    ///
    /// ```raku
    /// grammar Expr {
    ///   rule term { \d+ }
    /// }
    /// ```
    Grammar,

    /// A type class or trait instance, as in Purescript or Lean.
    ///
    /// ```lean
    /// instance : Inhabited Nat := ⟨0⟩
    /// ```
    Instance,

    /// An interface definition.
    ///
    /// ```go
    /// type Reader interface {
    ///     Read(p []byte) int
    /// }
    /// ```
    Interface,

    /// A key in a key-value structure.
    ///
    /// ```lua
    /// t = { name = "Alice" }
    /// ```
    Key,

    /// A language declaration, as used in Racket.
    ///
    /// ```racket
    /// #lang racket
    /// ```
    Lang,

    /// A lemma in formal proofs (Lean).
    ///
    /// ```lean
    /// lemma add_zero (n : Nat) : n + 0 = n := by rfl
    /// ```
    Lemma,

    /// A Solidity library.
    ///
    /// ```solidity
    /// library Math {
    ///     function add(uint a, uint b) internal pure returns (uint) {
    ///         return a + b;
    ///     }
    /// }
    /// ```
    Library,

    /// A macro definition.
    ///
    /// ```rust,ignore
    /// macro_rules! debug {
    ///     ($x:expr) => { println!("{:?}", $x) };
    /// }
    /// ```
    Macro,

    /// A method associated with a type.
    ///
    /// This symbol refers specifically to the method `login`,
    /// not to the enclosing struct `User`.
    ///
    /// ```rust,ignore
    /// impl User {
    ///     fn login(&self) {}
    ///     // ^^^ method
    /// }
    /// ```
    Method,

    /// A method alias, as in Ruby.
    ///
    /// ```ruby
    /// alias old_name new_name
    /// ```
    MethodAlias,

    /// A method receiver without a conventional name (Go).
    ///
    /// This symbol refers specifically to the receiver parameter.
    ///
    /// ```go
    /// func (User) Login() {}
    /// // ^^^ method receiver
    /// ```
    MethodReceiver,

    /// A method specification without implementation (Go interface).
    ///
    /// ```go
    /// type Reader interface {
    ///     Read([]byte) int
    /// }
    /// ```
    MethodSpecification,

    /// A Protobuf message definition.
    ///
    /// ```proto
    /// message User {
    ///   string name = 1;
    /// }
    /// ```
    Message,

    /// A mixin declaration (Dart).
    ///
    /// ```dart
    /// mixin Flyable {
    ///   void fly() {}
    /// }
    /// ```
    Mixin,

    /// A Solidity modifier.
    ///
    /// ```solidity
    /// modifier onlyOwner() {
    ///     require(msg.sender == owner);
    ///     _;
    /// }
    /// ```
    Modifier,

    /// A module declaration.
    ///
    /// ```rust,ignore
    /// mod utils;
    /// ```
    Module,

    /// A namespace used to group symbols.
    ///
    /// ```cpp
    /// namespace math {}
    /// ```
    Namespace,

    /// A null or absent value.
    ///
    /// ```rust,ignore
    /// let x: Option<i32> = None;
    /// ```
    Null,

    /// A numeric value or type.
    ///
    /// ```rust,ignore
    /// let n: i64 = 42;
    /// ```
    Number,

    /// An object value.
    ///
    /// ```lua
    /// user = { name = "Alice" }
    /// ```
    Object,

    /// An operator symbol.
    ///
    /// ```rust,ignore
    /// let x = a + b;
    /// ```
    Operator,

    /// A package declaration.
    ///
    /// ```go
    /// package main
    /// ```
    Package,

    /// A package-level object (Scala).
    ///
    /// ```scala
    /// package object util
    /// ```
    PackageObject,

    /// A function or method parameter.
    ///
    /// This symbol refers specifically to the parameter `x`,
    /// not to the enclosing function `foo`.
    ///
    /// ```rust,ignore
    /// fn foo(x: i32) {}
    /// //      ^^^ parameter
    /// ```
    Parameter,

    /// A labeled parameter (Swift).
    ///
    /// ```swift
    /// func greet(name person: String) {}
    /// ```
    ParameterLabel,

    /// A pattern synonym (Haskell).
    ///
    /// This symbol refers specifically to the pattern `Zero`,
    /// not to any enclosing module or type.
    ///
    /// ```haskell
    /// pattern Zero <- 0
    /// // ^^^ pattern
    /// ```
    Pattern,

    /// A logical predicate (Alloy).
    ///
    /// ```alloy
    /// pred isEven[n: Int] { n % 2 = 0 }
    /// // ^^^ predicate
    /// ```
    Predicate,

    /// A property symbol.
    ///
    /// This symbol refers specifically to the property `Age`,
    /// not to the enclosing class.
    ///
    /// ```csharp
    /// public int Age { get; set; }
    /// // ^^^ property
    /// ```
    Property,

    /// A protocol definition (Swift / Objective-C).
    ///
    /// ```swift
    /// protocol Drawable {
    ///   func draw()
    /// }
    /// ```
    Protocol,

    /// A protocol method without implementation.
    ///
    /// This symbol refers specifically to the method `draw`,
    /// not to the enclosing protocol `Drawable`.
    ///
    /// ```swift
    /// func draw()
    /// // ^^^ protocol method
    /// ```
    ProtocolMethod,

    /// A pure virtual method (C++).
    ///
    /// ```cpp
    /// virtual void draw() = 0;
    /// // ^^^ pure virtual method
    /// ```
    PureVirtualMethod,

    /// A Haskell quasiquoter.
    ///
    /// ```haskell
    /// [sql| SELECT * FROM users |]
    /// ```
    Quasiquoter,

    /// The `self` parameter in methods.
    ///
    /// This symbol refers specifically to `self`,
    /// not to the enclosing method.
    ///
    /// ```rust,ignore
    /// fn foo(&self) {}
    /// //       ^^^ self parameter
    /// ```
    SelfParameter,

    /// A setter accessor.
    ///
    /// This symbol refers specifically to the setter for `value`,
    /// not to the enclosing property.
    ///
    /// ```swift
    /// set { _value = newValue }
    /// // ^^^ setter
    /// ```
    Setter,

    /// An Alloy signature, analogous to a struct.
    ///
    /// ```alloy
    /// sig Node {
    ///   next: set Node
    ///   // ^^^ signature
    /// }
    /// ```
    Signature,

    /// A Ruby singleton class.
    ///
    /// ```ruby
    /// class << self
    /// end
    /// ```
    SingletonClass,

    /// A Ruby singleton method.
    ///
    /// ```ruby
    /// def self.run; end
    /// // ^^^ singleton method
    /// ```
    SingletonMethod,

    /// A static data member (C++).
    ///
    /// This symbol refers specifically to the static field `count`,
    /// not to the enclosing struct.
    ///
    /// ```cpp
    /// struct S {
    ///     static int count;
    ///     // ^^^ static data member
    /// };
    /// ```
    StaticDataMember,

    /// A static event (C#).
    ///
    /// ```csharp
    /// public static event Action OnExit;
    /// // ^^^ static event
    /// ```
    StaticEvent,

    /// A static field.
    ///
    /// ```csharp
    /// public static int Max;
    /// // ^^^ static field
    /// ```
    StaticField,

    /// A static method.
    ///
    /// ```java
    /// static void log() {}
    /// // ^^^ static method
    /// ```
    StaticMethod,

    /// A static property.
    ///
    /// ```typescript
    /// static get version(): number { return 1; }
    /// // ^^^ static property
    /// ```
    StaticProperty,

    /// A static variable.
    ///
    /// ```c
    /// static int counter = 0;
    /// // ^^^ static variable
    /// ```
    StaticVariable,

    /// A string value or type.
    ///
    /// ```rust,ignore
    /// let s = "hello";
    /// ```
    String,

    /// A struct type.
    ///
    /// ```rust,ignore
    /// struct Point { x: i32, y: i32 }
    /// ```
    Struct,

    /// A subscript (Swift).
    ///
    /// This symbol refers specifically to the subscript itself,
    /// not to the enclosing type.
    ///
    /// ```swift
    /// subscript(index: Int) -> Int {
    ///   return data[index]
    ///   // ^^^ subscript
    /// }
    /// ```
    Subscript,

    /// A proof tactic (Lean).
    ///
    /// ```lean
    /// by
    ///   intro x
    ///   rfl
    /// ```
    Tactic,

    /// A proven theorem (Lean).
    ///
    /// ```lean
    /// theorem add_comm (a b : Nat) : a + b = b + a := by
    ///   exact Nat.add_comm a b
    /// ```
    Theorem,

    /// A `this` receiver parameter.
    ///
    /// This symbol refers specifically to `this`,
    /// not to the enclosing method.
    ///
    /// ```cpp
    /// void foo(this MyType& self);
    /// //           ^^^ this parameter
    /// ```
    ThisParameter,

    /// A trait definition.
    ///
    /// ```rust,ignore
    /// trait Display {
    ///     fn fmt(&self);
    /// }
    /// ```
    Trait,

    /// A trait method without implementation.
    ///
    /// This symbol refers specifically to the method `fmt`,
    /// not to the enclosing trait `Display`.
    ///
    /// ```rust,ignore
    /// fn fmt(&self);
    /// // ^^^ trait method
    /// ```
    TraitMethod,

    /// A type definition (OCaml style `type` keyword).
    ///
    /// ```ocaml
    /// type point = { x : int; y : int }
    /// ```
    Type,

    /// A type alias.
    ///
    /// ```rust,ignore
    /// type UserId = u64;
    /// ```
    TypeAlias,

    /// A type class definition (Haskell/Purescript).
    ///
    /// ```haskell
    /// class Eq a where
    ///   (==) :: a -> a -> Bool
    /// ```
    TypeClass,

    /// A method belonging to a type class.
    ///
    /// This symbol refers specifically to `(==)`,
    /// not to the enclosing type class `Eq`.
    ///
    /// ```haskell
    /// (==) :: a -> a -> Bool
    /// // ^^^ type class method
    /// ```
    TypeClassMethod,

    /// A type family declaration (Haskell).
    ///
    /// ```haskell
    /// type family F a
    /// ```
    TypeFamily,

    /// A generic type parameter.
    ///
    /// This symbol refers specifically to `T`,
    /// not to the enclosing function `foo`.
    ///
    /// ```rust,ignore
    /// fn foo<T>(x: T) {}
    /// //        ^^^ type parameter
    /// ```
    TypeParameter,

    /// A union type.
    ///
    /// ```c
    /// union Value {
    ///   int i;
    ///   float f;
    /// };
    /// ```
    Union,

    /// A value-level symbol.
    ///
    /// ```rust,ignore
    /// let x = 10;
    /// ```
    Value,

    /// A variable binding.
    ///
    /// ```rust,ignore
    /// let mut count = 0;
    /// ```
    Variable,
}

impl From<String> for SymbolKind {
    fn from(value: String) -> Self {
        Self::try_from(value.as_str()).unwrap_or_default()
    }
}
