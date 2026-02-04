;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Lists
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Any parenthesized list
(list_lit) @Unknown

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Collections
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Vector -> Array
(vec_lit) @Array

;; Map -> Object
(map_lit) @Object

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Literals -> Value
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(num_lit) @Value
(str_lit) @Value
(bool_lit) @Value
(nil_lit) @Value
(kwd_lit) @Value

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; Symbols (for variables, functions, macros, namespace)
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Function definitions: (defn NAME …)
(list_lit
  (sym_lit) @head
  (#eq? @head "defn")
  (sym_lit) @Function)

;; Anonymous function literal
(anon_fn_lit) @Function

;; Macro definitions: (defmacro NAME …)
(list_lit
  (sym_lit) @head
  (#eq? @head "defmacro")
  (sym_lit) @Macro)

;; Let bindings (variable vector)
(list_lit
  (sym_lit) @head
  (#eq? @head "let")
  (vec_lit) @Variable)

;; Namespace declaration: (ns NAME …)
(list_lit
  (sym_lit) @head
  (#eq? @head "ns")
  (sym_lit) @Namespace)

;; Def forms: (def NAME …)
(list_lit
  (sym_lit) @head
  (#eq? @head "def")
  (sym_lit) @Variable)
