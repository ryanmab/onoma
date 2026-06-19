;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Modules / Exports
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(export_statement
  (_) @Module)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Classes
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(class_declaration
  name: (identifier) @Class)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Constants 
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(lexical_declaration
  kind: "const"
  (variable_declarator
    (identifier) @Constant))

(lexical_declaration
  (variable_declarator
    (identifier) @Variable))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Functions & Methods
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(function_declaration
  name: (identifier) @Function)

(variable_declarator
  name: (identifier) @Function
  value: (arrow_function))

;; Capture methods but exclude constructor
(method_definition
  name: (property_identifier) @Method
  (#not-eq? @Method "constructor"))

;; Capture the constructor specifically
(method_definition
  name: (property_identifier) @Constructor
  (#eq? @Constructor "constructor"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Getters & Setters
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(method_definition
  name: (property_identifier) @Getter
  (#match? @Getter "^get[A-Z_]*"))

(method_definition
  name: (property_identifier) @Setter
  (#match? @Setter "^set[A-Z_]*"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Parameters
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(formal_parameters
  (identifier) @Parameter)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Variables / Values
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(variable_declarator
  (identifier) @Variable)

(expression_statement
  (identifier) @Variable)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Member expressions (refined)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Object literal property declaration → Property
(object
  (pair
    key: (property_identifier) @Property))

;; Accessing a member → Value
(member_expression
  (_) @Value)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Literals
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(string) @String
(number) @Number
(true) @Boolean
(false) @Boolean
(null) @Null

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; JSX Elements (references)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Uppercase JSX components → references to component classes
(jsx_opening_element
  (identifier) @Value
  (#match? @Value "^[A-Z]"))

(jsx_self_closing_element
  (identifier) @Value
  (#match? @Value "^[A-Z]"))

;; Lowercase JSX elements (HTML tags) → generic Value
(jsx_opening_element
  (identifier) @Value
  (#match? @Value "^[a-z]"))

(jsx_self_closing_element
  (identifier) @Value
  (#match? @Value "^[a-z]"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; JSX Props
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(jsx_attribute
  (property_identifier) @Property)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; JSX Children
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(jsx_text) @String
