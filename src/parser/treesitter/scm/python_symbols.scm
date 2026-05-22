;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; IMPORTS → Module
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(import_statement
  name: (dotted_name (identifier) @Module))

(import_from_statement
  module_name: (dotted_name (identifier) @Module))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; VARIABLES
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(assignment
  left: (identifier) @Variable)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; CONSTANTS 
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  (assignment
    left: (identifier) @Constant)
  (#match? @Constant "^[A-Z][A-Z0-9_]*$")
)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; FUNCTIONS 
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(function_definition
  name: (identifier) @Function)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; CLASSES
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(class_definition
  name: (identifier) @Class)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; METHODS (inside class scope)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(class_definition
  body: (block
    (function_definition
      name: (identifier) @Method)))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; CONSTRUCTORS
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(class_definition
  body: (block
    (function_definition
      name: (identifier) @Constructor
      (#eq? @Constructor "__init__"))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; PARAMETERS
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(parameters
  (identifier) @Parameter)

(lambda_parameters
  (identifier) @Parameter)

(parameters
  (identifier) @SelfParameter
  (#match? @SelfParameter "^(self|cls)$"))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; STATIC / CLASS METHODS
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  class_definition
  body: (block
    (decorated_definition
      (decorator (identifier) @_decorator)
      definition: (function_definition
        name: (identifier) @StaticMethod)))
  (#match? @_decorator "^(staticmethod|classmethod)$")
)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; PROPERTIES
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  class_definition
  body: (block
    (decorated_definition
      (decorator (identifier) @_decorator)
      definition: (function_definition
        name: (identifier) @Property)))
  (#eq? @_decorator "property")
)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; GETTERS / SETTERS
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  class_definition
  body: (block
    (decorated_definition
      (decorator
        (attribute attribute: (identifier) @_attr))
      definition: (function_definition
        name: (identifier) @Getter)))
  (#eq? @_attr "getter")
)

(
  class_definition
  body: (block
    (decorated_definition
      (decorator
        (attribute attribute: (identifier) @_attr))
      definition: (function_definition
        name: (identifier) @Setter)))
  (#eq? @_attr "setter")
)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; CLASS FIELDS 
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(class_definition
  body: (block
    (expression_statement
      (assignment
        left: (identifier) @Field))))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; ENUMS 
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  class_definition
  name: (identifier) @Enum
  superclasses: (argument_list
    (identifier) @_base)
  (#eq? @_base "Enum")
)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; ENUM MEMBERS 
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  class_definition
  name: (identifier)
  superclasses: (argument_list
    (identifier) @_base)
  body: (block
    (expression_statement
      (assignment
        left: (identifier) @EnumMember)))
  (#eq? @_base "Enum")
  (#match? @EnumMember "^[A-Z][A-Z0-9_]*$")
)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; EXCEPTIONS
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(
  class_definition
  name: (identifier) @Error
  superclasses: (argument_list
    (identifier) @_base)
  (#eq? @_base "Exception")
)
