;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Enum-like tables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assignment_statement
  (variable_list
    (identifier) @Enum)
  (expression_list
    (table_constructor
      (field) @EnumMember)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(function_declaration
  name: (identifier) @Function)

(function_declaration
  name: (method_index_expression
    method: (identifier) @Method))

(assignment_statement
  (variable_list
    (identifier) @Function)
  (expression_list
    (function_definition)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Variables
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assignment_statement
  (variable_list
    (identifier) @Variable))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Tables (Struct / Object equivalent)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assignment_statement
  (variable_list
    (identifier) @Struct)
  (expression_list
    (table_constructor)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Fields / Properties
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(field
  name: (identifier) @Field)

(dot_index_expression
  field: (identifier) @Property)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Methods assigned to table fields
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assignment_statement
  (variable_list
    (dot_index_expression
      field: (identifier) @Method))
  (expression_list
    (function_definition)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Constants (heuristic: uppercase names)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assignment_statement
  (variable_list
    (identifier) @Constant)
  (#match? @Constant "^[A-Z_][A-Z0-9_]*$"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Literals
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(string) @String
(number) @Number
(true) @Boolean
(false) @Boolean
(nil) @Null
