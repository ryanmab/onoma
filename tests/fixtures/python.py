############################
# Modules (imports)
############################

import os
from sys import path

############################
# Constants / Variables
############################

MAX_COUNT = 10
variable = 5

############################
# Decorators
############################

def decorator(func):
    return func

@decorator
def decorated_function(x, y):
    local_var = x + y
    return local_var

############################
# Class
############################

class MyClass:
    CLASS_CONST = 100

    def __init__(self, value):
        self.field = value

    def regular_method(self, a, b):
        temp = a * b
        return temp

    async def async_method(self, param):
        result = self.field + param
        return result

############################
# Top-level functions
############################

async def async_function(n):
    return n * 2


def top_level_function(a, b):
    return a + b

############################
# Nested functions 
############################

def outer_function(x):
    def inner_function(y):
        return x + y
    return inner_function(x)

############################
# Class instance usage
############################

obj = MyClass(1)
value = obj.field

############################
# Literals 
############################

string_value = "hello"
int_value = 42
float_value = 3.14
bool_true = True
bool_false = False
none_value = None

############################
# Enum 
############################

from enum import Enum

class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3

############################
# Exception 
############################

class MyError(Exception):
    pass

############################
# Typed class 
############################

class User:
    name = "alice"
    AGE = 30
