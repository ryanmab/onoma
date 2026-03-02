############################
# Modules (imports)
############################

import os
from sys import path

############################
# Constants / Variables
############################

MAX_COUNT = 10        # @Constant
variable = 5          # @Variable

############################
# Decorator
############################

def decorator(func):
    return func

@decorator               # @Value
def decorated_function(x, y):   # @Function + @Parameter
    local_var = x + y    # @Variable
    return local_var

############################
# Class
############################

class MyClass:           # @Class
    CLASS_CONST = 100    # @Constant

    def __init__(self, value):  # @Method + @Parameter
        self.field = value      # @Field

    async def async_method(self, param):  # @Method + @Parameter
        return self.field + param         # @Property

    def regular_method(self, a, b):       # @Method + @Parameter
        temp = a * b                      # @Variable
        return temp

############################
# Top-level function
############################

async def async_function(n):   # @Function + @Parameter
    return n * 2

############################
# Attribute / Property usage
############################

obj = MyClass(1)
value = obj.field              # @Property

############################
# Literals
############################

string_value = "hello"         # @String
int_value = 42                 # @Number
float_value = 3.14             # @Number
bool_true = True               # @Boolean
bool_false = False             # @Boolean
none_value = None              # @Null
