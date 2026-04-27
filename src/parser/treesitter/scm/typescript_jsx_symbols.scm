;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Modules (heuristic: export statements)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(export_statement
  (_) @Module)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Types
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(interface_declaration
  (type_identifier) @Interface)

(type_alias_declaration
  (type_identifier) @TypeAlias)

(enum_declaration
  name: (identifier) @Enum)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Enum members (only inside enums)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(enum_declaration
  (enum_body
    (property_identifier) @EnumMember))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Classes
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(class_declaration
  (type_identifier) @Class)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Class fields
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(public_field_definition
  (property_identifier) @Field)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(function_declaration
  (identifier) @Function)

;; Arrow functions assigned to variables (const/let/var)
(lexical_declaration
  (variable_declarator
    name: (identifier) @Function
    value: (arrow_function)))

;; Function expressions assigned to variables
(lexical_declaration
  (variable_declarator
    name: (identifier) @Function
    value: (function_expression)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Methods & Constructors
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Capture methods but exclude the constructor
(method_definition
  name: (property_identifier) @Method
  (#not-eq? @Method "constructor"))

;; Capture the constructor specifically
(method_definition
  name: (property_identifier) @Constructor
  (#eq? @Constructor "constructor"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Parameters
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(required_parameter
  (identifier) @Parameter)

(optional_parameter
  (identifier) @Parameter)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Variables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(variable_declarator
  (identifier) @Variable)

(lexical_declaration
  kind: "const"
  (variable_declarator
    (identifier) @Constant))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; JSX components
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(jsx_opening_element
  (identifier) @Component)

(jsx_self_closing_element
  (identifier) @Component)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; JSX props
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(jsx_attribute
  (property_identifier) @Property)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; JSX children (text)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(jsx_text) @String
